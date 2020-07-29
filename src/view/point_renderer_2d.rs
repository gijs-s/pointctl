extern crate nalgebra as na;

// Third party
use super::marcos;
use gl;
use image::{self, DynamicImage};
use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::renderer::PlanarRenderer;
use kiss3d::renderer::Renderer;
use kiss3d::resource::{
    AllocationType, BufferType, Effect, GPUVec, ShaderAttribute, ShaderUniform, Texture,
};
use na::{Matrix3, Point2, Point3};
use std::path::Path;

/// 2D
pub struct PointRenderer2D {
    shader: Effect,
    pos_attribute: ShaderAttribute<Point2<f32>>,
    color_attribute: ShaderAttribute<Point3<f32>>,
    // position on the texture (0..1)
    texture_pos_attribute: ShaderAttribute<Point2<f32>>,
    proj_uniform: ShaderUniform<Matrix3<f32>>,
    view_uniform: ShaderUniform<Matrix3<f32>>,
    alpha_texture_uniform: ShaderUniform<i32>,
    // GPU vecs
    points_vec: GPUVec<Point2<f32>>,
    colors_vec: GPUVec<Point3<f32>>,
    texture_points_vec: GPUVec<Point2<f32>>,
    // Normal variables
    alpha_texture: Texture,
    point_size: f32,
    visible: bool,
}

#[allow(dead_code)]
impl PointRenderer2D {
    pub fn new() -> PointRenderer2D {
        let mut shader = Effect::new_from_str(VERTEX_SHADER_SRC_2D, FRAGMENT_SHADER_SRC_2D);

        shader.use_program();

        PointRenderer2D {
            points_vec: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            colors_vec: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            texture_points_vec: GPUVec::new(
                Vec::new(),
                BufferType::Array,
                AllocationType::StreamDraw,
            ),
            // Shader variables
            pos_attribute: shader
                .get_attrib::<Point2<f32>>("position")
                .expect("Failed to get shader attribute."),
            color_attribute: shader
                .get_attrib::<Point3<f32>>("color")
                .expect("Failed to get shader attribute."),
            texture_pos_attribute: shader
                .get_attrib::<Point2<f32>>("textureCoordinate")
                .expect("Failed to get `textureCoordinate` shader attribute."),
            proj_uniform: shader
                .get_uniform::<Matrix3<f32>>("proj")
                .expect("Failed to get shader attribute."),
            view_uniform: shader
                .get_uniform::<Matrix3<f32>>("view")
                .expect("Failed to get shader attribute."),
            alpha_texture_uniform: shader
                .get_uniform("alphaTexture")
                .expect("Failed to get 'alphaTexture' unifrom shader attribute"),
            // Shader itself
            shader,
            // GL variables
            point_size: 4.0,
            visible: true,
            alpha_texture: PointRenderer2D::load_texture(),
        }
    }

    /// Insert a single point with a color
    pub fn push(&mut self, point: Point2<f32>, color: Point3<f32>) {
        for points in self.points_vec.data_mut().iter_mut() {
            points.push(point);
        }
        for colors in self.colors_vec.data_mut().iter_mut() {
            colors.push(color)
        }
    }

    /// Clear all the points
    pub fn clear(&mut self) {
        for points in self.points_vec.data_mut().iter_mut() {
            points.clear()
        }
        for color in self.colors_vec.data_mut().iter_mut() {
            color.clear()
        }
    }

    /// Indicates whether some points have to be drawn.
    pub fn needs_rendering(&self) -> bool {
        self.num_points() != 0 && self.visible
    }

    // Turn of the rendering for this renderer and clear the screen.
    pub fn hide(&mut self) {
        self.visible = false;

        // Clear the screen
        let ctxt = Context::get();
        verify!(ctxt.active_texture(Context::TEXTURE0));
    }

    // Turn on the rendering for this renderer
    pub fn show(&mut self) {
        self.visible = true;
    }

    // Set the point size
    pub fn set_point_size(&mut self, point_size: f32) {
        self.point_size = point_size;
    }

    // Retrieve the number of points
    pub fn num_points(&self) -> usize {
        self.points_vec.len()
    }

    pub fn load_texture() -> Texture {
        let ctxt = Context::get();

        // Is this correct?
        verify!(ctxt.active_texture(Context::TEXTURE1));
        verify!(ctxt.pixel_storei(Context::UNPACK_ALIGNMENT, 1));

        // Assign a index for the texture
        let texture = verify!(ctxt
            .create_texture()
            .expect("Alpha texture creation failed."));

        // All following actions need to be preformed on the texture we just created.
        verify!(ctxt.bind_texture(Context::TEXTURE_2D, Some(&texture)));

        // Load in a image containing a static blob alpha map
        // TODO: Generate this image programmatically. Easier se than done though because
        // of the variable lifetimes.
        let dyn_img: DynamicImage =
            image::open(&Path::new("resources/blob.png")).expect("Failed to load texture");
        match dyn_img {
            DynamicImage::ImageRgba8(img) => {
                verify!(ctxt.tex_image2d(
                    Context::TEXTURE_2D,
                    0,
                    Context::RGBA as i32,
                    img.width() as i32,
                    img.height() as i32,
                    0,
                    Context::RGBA,
                    Some(&img.into_raw()[..])
                ));
            }
            _ => {
                panic!("'resources/blob.png' is not an RGBA image.");
            }
        }

        // Set the correct texture parameters.
        let settings = vec![
            (Context::TEXTURE_WRAP_S, Context::CLAMP_TO_EDGE as i32),
            (Context::TEXTURE_WRAP_T, Context::CLAMP_TO_EDGE as i32),
            (Context::TEXTURE_MIN_FILTER, Context::LINEAR as i32),
            (Context::TEXTURE_MAG_FILTER, Context::LINEAR as i32),
        ];
        for (pname, param) in settings {
            verify!(ctxt.tex_parameteri(Context::TEXTURE_2D, pname, param));
        }

        // Return the created texture
        texture
    }
}

impl PlanarRenderer for PointRenderer2D {
    /// Actually draws the lines.
    fn render(&mut self, camera: &mut dyn PlanarCamera) {
        if !self.needs_rendering() {
            return;
        }

        self.shader.use_program();

        // Enable the attributes
        self.pos_attribute.enable();
        self.color_attribute.enable();
        self.texture_pos_attribute.enable();

        // Camera settings
        camera.upload(&mut self.proj_uniform, &mut self.view_uniform);

        // Set the texture
        self.alpha_texture_uniform.upload(&1);

        // Bind the buffers with daa to all the shader attributes
        self.pos_attribute
            .bind_sub_buffer(&mut self.points_vec, 0, 0);
        self.color_attribute
            .bind_sub_buffer(&mut self.colors_vec, 0, 0);
        self.texture_pos_attribute
            .bind_sub_buffer(&mut self.texture_points_vec, 0, 0);

        // Dive into gl calls!
        let ctxt = Context::get();

        // Set the correct drawing method of the polygons
        let _ = verify!(ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::FILL));

        // Enable blending with alpha
        verify!(ctxt.enable(Context::BLEND));
        verify!(ctxt.blend_func(Context::SRC_ALPHA, Context::ONE_MINUS_SRC_ALPHA));

        // Ensure we use the correct texture
        verify!(ctxt.bind_texture(Context::TEXTURE_2D, Some(&self.alpha_texture)));

        // Actually draw the textured polygons. Each point is split represented by
        // 2 textured triangles.
        verify!(ctxt.draw_arrays(Context::TRIANGLES, 0, self.num_points() as i32));

        // Disable the blending again.
        verify!(ctxt.disable(Context::BLEND));

        // Disable the attributes again.
        self.pos_attribute.disable();
        self.color_attribute.disable();
        self.texture_pos_attribute.disable();
    }
}

const VERTEX_SHADER_SRC_2D: &'static str = "#version 460
    // Input to this shader
    in vec2 position;
    in vec2 textureCoordinate;
    in vec3 color;

    uniform   mat3 proj;
    uniform   mat3 view;

    // Passed on to the rest of the shader pipeline
    out vec3 PointColor;
    out vec2 TextureCoordinate;

    // Transfrom a HSV color to an RGB color
    // Here all components are in the range [0…1], including hue.
    vec3 hsv2rgb(vec3 c)
    {
        vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
        vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
        return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
    }

    void main() {
        // Transform the world coordinate to a screen coordinate
        vec3 projected_pos = proj * view * vec3(position, 1.0);
        // We set the z to 0 to draw all points along a plane.
        projected_pos.z = 0.0;

        gl_Position = vec4(projected_pos, 1.0);

        // Make the color and tex coordinate available to the fragment shader.
        PointColor = hsv2rgb(color);
        TextureCoordinate = textureCoordinate;
    }";

/// Fragment shader used by the point renderer
const FRAGMENT_SHADER_SRC_2D: &'static str = "#version 460
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif

    // input color
    in vec3 PointColor;
    in vec2 TextureCoordinate;

    // Uniform containing the alpha texture, we use this to
    // draw the point in the middle of one edge of the triangle
    // and then have the alpha drop off towards all sides.
    // Changing the dropoff rate requires changing the texture, the
    // size should be determined by the size of the triangles
    uniform sampler2D alphaTexture;

    // output color
    layout( location = 0 ) out vec4 FragColor;

    void main() {
        float alpha = texture(alphaTexture, TextureCoordinate);

        // Don't even draw the point if the alpha is 0
        if(alpha == 0.0)
            discard;

        FragColor = vec4(PointColor, alpha);
    }";
