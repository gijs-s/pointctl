extern crate nalgebra as na;

// Third party
use gl;
use image::{self, DynamicImage};
use kiss3d::{
    planar_camera::PlanarCamera,
    context::Context,
    renderer::PlanarRenderer,
    resource::{
        AllocationType, BufferType, Effect, GPUVec, ShaderAttribute, ShaderUniform, Texture,
    }
};
use na::{Matrix3, Point2, Point3};
use std::path::Path;

// Internal
use super::{RenderMode,marcos};

/// 2D
pub struct PointRenderer2D {
    shader: Effect,
    pos_attribute: ShaderAttribute<Point2<f32>>,
    color_attribute: ShaderAttribute<Point3<f32>>,
    // Uniform attributes
    proj_uniform: ShaderUniform<Matrix3<f32>>,
    view_uniform: ShaderUniform<Matrix3<f32>>,
    alpha_texture_uniform: ShaderUniform<i32>,
    render_mode_uniform: ShaderUniform<i32>,
    blob_size_uniform: ShaderUniform<f32>,
    // GPU vecs
    points_vec: GPUVec<Point2<f32>>,
    colors_vec: GPUVec<Point3<f32>>,
    // Normal variables
    alpha_texture: Texture,
    point_size: f32,
    blob_size: f32,
    visible: bool,
    pub render_mode: RenderMode,
}

impl PointRenderer2D {
    pub fn new() -> PointRenderer2D {
        let mut shader = Effect::new_from_str(VERTEX_SHADER_SRC_2D, FRAGMENT_SHADER_SRC_2D);

        shader.use_program();

        PointRenderer2D {
            points_vec: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            colors_vec: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            // Shader variables
            pos_attribute: shader
                .get_attrib::<Point2<f32>>("position")
                .expect("Failed to get 'position' shader attribute."),
            color_attribute: shader
                .get_attrib::<Point3<f32>>("color")
                .expect("Failed to get 'color' shader attribute."),
            proj_uniform: shader
                .get_uniform::<Matrix3<f32>>("proj")
                .expect("Failed to get 'proj' uniform shader attribute"),
            view_uniform: shader
                .get_uniform::<Matrix3<f32>>("view")
                .expect("Failed to get 'view' uniform shader attribute"),
            blob_size_uniform: shader
                .get_uniform("blobSize")
                .expect("Failed to get 'blobSize' uniform shader attribute"),
            alpha_texture_uniform: shader
                .get_uniform("alphaTexture")
                .expect("Failed to get 'alphaTexture' uniform shader attribute"),
            render_mode_uniform: shader
                .get_uniform("renderMode")
                .expect("Failed to get 'renderMode' uniform shader attribute"),
            // Shader itself
            shader,
            // GL variables
            point_size: 4.0,
            blob_size: 1.0,
            visible: true,
            alpha_texture: PointRenderer2D::load_texture(),
            render_mode: RenderMode::Discreet,
        }
    }

    /// Insert a single point with a color
    pub fn push(&mut self, point: Point2<f32>, color: Point3<f32>) {
        for points in self.points_vec.data_mut().iter_mut() {
            for _ in 0..6 {
                points.push(point);
            }
        }
        for colors in self.colors_vec.data_mut().iter_mut() {
            for _ in 0..6 {
                colors.push(color);
            }
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

    pub fn set_blob_size(&mut self, blob_size: f32) {
        self.blob_size = blob_size
    }

    // Retrieve the number of points
    pub fn num_points(&self) -> usize {
        self.points_vec.len()
    }

    pub fn switch_rendering_mode(&mut self) {
        self.render_mode = match self.render_mode {
            RenderMode::Discreet => RenderMode::Continuous,
            RenderMode::Continuous => RenderMode::Discreet,
        }
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
        match image::open(&Path::new("resources/blob2.png")).expect("Failed to load texture") {
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
                panic!("'resources/blob2.png' is not an RGBA image.");
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
        // If there are no points we do not even need to attempt the render
        if !self.needs_rendering() {
            return;
        }

        self.shader.use_program();

        // Enable the attributes
        self.pos_attribute.enable();
        self.color_attribute.enable();

        // Camera settings
        camera.upload(&mut self.proj_uniform, &mut self.view_uniform);

        // Set the texture
        self.alpha_texture_uniform.upload(&1);

        // Set the blob size
        self.blob_size_uniform.upload(&self.blob_size);

        // Dive into gl calls!
        let ctxt = Context::get();

        // Enable blending with alpha
        verify!(ctxt.enable(Context::BLEND));
        verify!(ctxt.blend_func(Context::SRC_ALPHA, Context::ONE_MINUS_SRC_ALPHA));

        match self.render_mode {
            RenderMode::Discreet => {
                // set the correct render mode in the shader.
                self.render_mode_uniform.upload(&0);

                // Draw the first point of all the triangle sets, hence the stride of 5
                self.pos_attribute
                    .bind_sub_buffer(&mut self.points_vec, 5, 0);
                self.color_attribute
                    .bind_sub_buffer(&mut self.colors_vec, 5, 0);

                verify!(ctxt.draw_arrays(Context::POINTS, 0, self.num_points() as i32));
            }
            RenderMode::Continuous => {
                // set the correct render mode in the shader.
                self.render_mode_uniform.upload(&1);

                // Bind the buffers with daa to all the shader attributes
                self.pos_attribute
                    .bind_sub_buffer(&mut self.points_vec, 0, 0);
                self.color_attribute
                    .bind_sub_buffer(&mut self.colors_vec, 0, 0);

                // Set the correct drawing method of the polygons
                let _ = verify!(ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::FILL));

                // Ensure we use the correct texture
                verify!(ctxt.bind_texture(Context::TEXTURE_2D, Some(&self.alpha_texture)));

                // Actually draw the textured polygons. Each point is split represented by
                // 2 textured triangles.
                verify!(ctxt.draw_arrays(Context::TRIANGLES, 0, self.num_points() as i32));
            }
        }

        // Disable the blending again.
        verify!(ctxt.disable(Context::BLEND));

        // Disable the attributes again.
        self.pos_attribute.disable();
        self.color_attribute.disable();
    }
}

const VERTEX_SHADER_SRC_2D: &'static str = "#version 460
    // Input to this shader
    in vec2 position;
    in vec3 color;

    // Uniform variables over all the inputs
    uniform mat3 proj;
    uniform mat3 view;
    uniform float blobSize;
    uniform int renderMode;

    // Passed on to the rest of the shader pipeline
    out vec2 TextureCoordinate;
    out vec3 PointColor;

    // Get the texture coordinate
    vec2 getTextureCoordinate() {
        float index = mod(gl_VertexID, 3);
        if (index == 0.0) {
            return vec2(1.0, 0.0);
        }
        if (index == 1.0) {
            return vec2(0.0, 0.0);
        }
        if (index == 2.0) {
            return vec2(0.0, 1.0);
        }
    }

    // Get the offset vector.
    vec2 getOffset() {
        float scale = blobSize;
        float negScale = -1.0 * blobSize;

        float index = mod(gl_VertexID, 6);
        if (index == 0.0) {
            return vec2(negScale, 0.0);
        }
        if (index == 1.0) {
            return vec2(0.0, scale);
        }
        if (index == 2.0) {
            return vec2(scale, 0.0);
        }
        if (index == 3.0) {
            return vec2(negScale, 0.0);
        }
        if (index == 4.0) {
            return vec2(0.0, negScale);
        }
        if (index == 5.0) {
            return vec2(scale, 0.0);
        }
    }

    void render_discreet() {
        // Transform the world coordinate to a screen coordinate
        vec3 projected_pos = proj * view * vec3(position, 1.0);
        // We set the z to 0 to draw all points along a plane.
        projected_pos.z = 0.0;

        // Set the screenspace position
        gl_Position = vec4(projected_pos, 1.0);

        // Make the color and tex coordinate available to the fragment shader.
        PointColor = color;
        TextureCoordinate = getTextureCoordinate();
    }

    void render_continuos() {
        // Get the offset to one of the triangle corners
        vec2 offset_position = position + getOffset();
        // Get the projected triangle corner position
        vec3 projected_pos = proj * view * vec3(offset_position, 1.0);
        // We set the z to 0 to draw all points along a plane.
        projected_pos.z = 0.0;

        // Set the screenspace position
        gl_Position = vec4(projected_pos, 1.0);

        // Make the color and tex coordinate available to the fragment shader.
        PointColor = color;
        TextureCoordinate = getTextureCoordinate();
    }

    void main() {
        if (renderMode == 0) {
            render_discreet();
            return;
        } else {
            render_continuos();
            return;
        }
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

    // Uniform variables over all the inputs
    uniform sampler2D alphaTexture;
    uniform int renderMode;

    // output color
    layout( location = 0 ) out vec4 FragColor;

    // Transfrom a HSV color to an RGB color (0..1)
    vec3 hsv2rgb(vec3 c)
    {
        vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
        vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
        return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
    }

    // Discreet render mode, just draw the point.
    void render_discreet() {
        vec3 rgb_color = hsv2rgb(PointColor);
        FragColor = vec4(rgb_color, 1.0);
    }

    // Continous render mode, here we need to use the texture for the alpha
    void render_continuos() {
        // Get alpha from the texture
        float alpha = texture(alphaTexture, TextureCoordinate).a;
        // Don't even draw the point if the alpha is 0
        if(alpha == 0.0)
            discard;

        vec3 rgb_color = hsv2rgb(PointColor);
        FragColor = vec4(rgb_color, alpha);
    }

    void main() {
        if (renderMode == 0){
            render_discreet();
        } else {
            render_continuos();
        }
    }";
