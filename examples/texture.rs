#![feature(core_intrinsics)]

extern crate kiss3d;
extern crate nalgebra as na;
extern crate pointctl;

// Third party
use gl;
use image::{self, DynamicImage};
use kiss3d::camera::ArcBall;
use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::renderer::Renderer;
use kiss3d::text::Font;
use kiss3d::{
    light::Light,
    post_processing::PostProcessingEffect,
    resource::{
        AllocationType, BufferType, Effect, GPUVec, ShaderAttribute, ShaderUniform, Texture,
    },
    window::{State, Window},
};
use na::{Matrix3, Matrix4, Point2, Point3};
use std::path::Path;

// TODO: Add render mode for continuous or discreet!

fn main() {
    let mut window = Window::new("Texture testing");
    let state = AppState::new();

    window.set_background_color(0.9, 0.9, 0.9);
    window.set_light(Light::StickToCamera);
    window.render_loop(state);
}

fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

struct AppState {
    renderer: PointRenderer3D,
    camera: ArcBall,
}

impl AppState {
    pub fn new() -> Self {
        let mut renderer = PointRenderer3D::new();
        renderer.batch_insert(vec![
            (Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 1.0)),
            (Point3::new(0.0, 1.0, 0.0), Point3::new(0.0, 1.0, 1.0)),
            (Point3::new(-1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 1.0)),
            (Point3::new(-1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 1.0)),
            (Point3::new(0.0, -1.0, 0.0), Point3::new(0.0, 1.0, 1.0)),
            (Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 1.0)),
            (Point3::new(-1.0, 0.0, 2.0), Point3::new(0.5, 1.0, 1.0)),
            (Point3::new(0.0, 1.0, 2.0), Point3::new(0.5, 1.0, 1.0)),
            (Point3::new(1.0, 0.0, 2.0), Point3::new(0.5, 1.0, 1.0)),
            (Point3::new(1.0, 0.0, 2.0), Point3::new(0.5, 1.0, 1.0)),
            (Point3::new(0.0, -1.0, 2.0), Point3::new(0.5, 1.0, 1.0)),
            (Point3::new(-1.0, 0.0, 2.0), Point3::new(0.5, 1.0, 1.0)),
        ]);
        let eye = Point3::new(0.0f32, 0.0, -1.5);
        let at = Point3::new(0.0f32, 0.0f32, 0.0f32);
        let camera = ArcBall::new_with_frustrum(std::f32::consts::PI / 3.0, 0.01, 1024.0, eye, at);
        AppState { renderer, camera }
    }
}

impl State for AppState {
    // Return the custom renderer that will be called at each
    // render loop.
    fn cameras_and_effect_and_renderer(
        &mut self,
    ) -> (
        Option<&mut dyn Camera>,
        Option<&mut dyn PlanarCamera>,
        Option<&mut dyn Renderer>,
        Option<&mut dyn PostProcessingEffect>,
    ) {
        (Some(&mut self.camera), None, Some(&mut self.renderer), None)
    }

    fn step(&mut self, window: &mut Window) {
        let num_points_text = format!("Number of points: {}", self.renderer.num_points());
        window.draw_text(
            &num_points_text,
            &Point2::new(0.0, 20.0),
            60.0,
            &Font::default(),
            &Point3::new(0.0, 0.0, 0.0),
        );
    }
}

/// 3D
pub struct PointRenderer3D {
    // The shader itself
    shader: Effect,
    /// Shader attributes
    pos_attribute: ShaderAttribute<Point3<f32>>,
    color_attribute: ShaderAttribute<Point3<f32>>,
    // position on the texture (0..1)
    tex_pos_attributes: ShaderAttribute<Point2<f32>>,
    // Shader uniform
    view_uniform: ShaderUniform<Matrix4<f32>>,
    proj_uniform: ShaderUniform<Matrix4<f32>>,
    gamma_uniform: ShaderUniform<f32>,
    alpha_texture_uniform: ShaderUniform<i32>,
    // Data allocation
    points: GPUVec<Point3<f32>>,
    tex_positions: GPUVec<Point2<f32>>,
    // Normal variables
    alpha_texture: Texture,
    point_size: f32,
    gamma: f32,
    visible: bool,
    splat_size: f32,
}

macro_rules! verify(
    ($e: expr) => {
        {
            let res = $e;
            { assert_eq!(Context::get().get_error(), 0); }
            res
        }
    }
);

impl PointRenderer3D {
    pub fn new() -> PointRenderer3D {
        let mut shader = Effect::new_from_str(VERTEX_SHADER_SRC_3D, FRAGMENT_SHADER_SRC_3D);
        shader.use_program();

        PointRenderer3D {
            // Points and their color interleaved. note that each point in the cloud will have 6 points here as it defines
            // 2 triangles in the continous render mode
            points: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            tex_positions: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            // Shader variables
            pos_attribute: shader
                .get_attrib::<Point3<f32>>("position")
                .expect("Failed to get `position` shader attribute."),
            color_attribute: shader
                .get_attrib::<Point3<f32>>("color")
                .expect("Failed to get `color` shader attribute."),
            tex_pos_attributes: shader
                .get_attrib::<Point2<f32>>("textureCoordinate")
                .expect("Failed to get `textureCoordinate` shader attribute."),
            view_uniform: shader
                .get_uniform::<Matrix4<f32>>("view")
                .expect("Failed to get `view` shader attribute."),
            proj_uniform: shader
                .get_uniform::<Matrix4<f32>>("proj")
                .expect("Failed to get `proj` shader attribute."),
            gamma_uniform: shader
                .get_uniform::<f32>("gamma")
                .expect("Could not get `gamma` shader attribute"),
            alpha_texture_uniform: shader
                .get_uniform("alphaTexture")
                .expect("Could not get `alphaTexture` shader attribute"),
            // Shader itself
            shader,
            // GL variables
            alpha_texture: PointRenderer3D::load_texture(),
            point_size: 4.0,
            // Gamma variable
            gamma: 1.0,
            // Variable to set when skipping all rendering while keeping data loaded.
            visible: true,
            // The size of all splats (normaly the average distance to the nearest neighbor)
            splat_size: 1.0,
        }
    }

    /// Insert a single point with a color
    pub fn push(&mut self, point: Point3<f32>, color: Point3<f32>) {
        // let offsets = vec![]
        for points_buffer in self.points.data_mut().iter_mut() {
            points_buffer.push(point);
            points_buffer.push(color);
        }
        // TODO push correct data.
        for tex_pos_buffer in self.tex_positions.data_mut().iter_mut() {
            tex_pos_buffer.push(Point2::new(0.0, 0.0));
        }
    }

    /// Insert a large number of points with colors at once
    pub fn batch_insert(&mut self, points_x_colors: Vec<(Point3<f32>, Point3<f32>)>) {
        for points_buffer in self.points.data_mut().iter_mut() {
            for &(point, color) in points_x_colors.iter() {
                // TODO: Add three points per point, size based on continuous drop off.
                points_buffer.push(point);
                points_buffer.push(color);
            }
        }

        // TODO Push correct data
        for tex_pos_buffer in self.tex_positions.data_mut().iter_mut() {
            for _ in 0..(points_x_colors.len() / 3) {
                tex_pos_buffer.push(Point2::new(1.0, 0.0));
                tex_pos_buffer.push(Point2::new(0.0, 0.0));
                tex_pos_buffer.push(Point2::new(0.0, 1.0));
            }
        }
    }

    /// Clear all the points and their colors
    pub fn clear(&mut self) {
        for points in self.points.data_mut().iter_mut() {
            points.clear()
        }
    }

    /// Indicates whether some points have to be drawn.
    pub fn needs_rendering(&self) -> bool {
        self.points.len() != 0 && self.visible
    }

    // Turn off the rendering for this renderer and clear the screen.
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

    /// Set the gamma which will be used to next render loop
    pub fn set_gamma(&mut self, gamma: f32) {
        self.gamma = gamma;
    }

    /// Set the splat size
    // TODO: Redraw all the points with new splat size
    pub fn set_splat_size(&mut self, size: f32) {
        // TODO: Redraw all the points
        self.splat_size = size;
    }

    // Retrieve the number of points
    pub fn num_points(&self) -> usize {
        self.points.len() / 2
    }

    /// Create the alpha map texture and load it onto the gpu
    fn load_texture() -> Texture {
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

        // Generate the alpha maps source data
        // let texture_data = PointRenderer3D::generate_alpha_texture();
        // let texture_data:[u8; 16] = [255,0,0,255,0,255,0,255,0,0,255,255,255,0,255,255];
        let img: DynamicImage =
            image::open(&Path::new("resources/blob.png")).expect("Failed to load texture");
        match img {
            DynamicImage::ImageRgba8(image) => {
                let width = image.width();
                let height = image.height();
                verify!(ctxt.tex_image2d(
                    Context::TEXTURE_2D,
                    0,
                    Context::RGBA as i32,
                    image.width() as i32,
                    image.height() as i32,
                    0,
                    Context::RGBA,
                    Some(&image.into_raw()[..])
                ));
            }
            _ => {}
        };

        // Load the texture as image, mark is as alpha map so the rest of the values will
        // be 0.
        // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml

        // TODO: These are probably not correct. Look at mipmap tutorial instead.
        verify!(ctxt.tex_parameteri(
            Context::TEXTURE_2D,
            Context::TEXTURE_WRAP_S,
            Context::CLAMP_TO_EDGE as i32
        ));
        verify!(ctxt.tex_parameteri(
            Context::TEXTURE_2D,
            Context::TEXTURE_WRAP_T,
            Context::CLAMP_TO_EDGE as i32
        ));
        verify!(ctxt.tex_parameteri(
            Context::TEXTURE_2D,
            Context::TEXTURE_MIN_FILTER,
            Context::LINEAR as i32
        ));
        verify!(ctxt.tex_parameteri(
            Context::TEXTURE_2D,
            Context::TEXTURE_MAG_FILTER,
            Context::LINEAR as i32
        ));

        texture
    }
}

impl Renderer for PointRenderer3D {
    /// Actually draws the points.
    fn render(&mut self, pass: usize, camera: &mut dyn Camera) {
        // If there are no points to draw or the render is not set to visible to not do anything
        if !self.needs_rendering() {
            return;
        }

        self.shader.use_program();
        self.pos_attribute.enable();
        self.color_attribute.enable();
        self.tex_pos_attributes.enable();

        self.gamma_uniform.upload(&self.gamma);
        self.alpha_texture_uniform.upload(&1);
        camera.upload(pass, &mut self.proj_uniform, &mut self.view_uniform);

        self.pos_attribute.bind_sub_buffer(&mut self.points, 1, 0);
        self.color_attribute.bind_sub_buffer(&mut self.points, 1, 1);
        self.tex_pos_attributes
            .bind_sub_buffer(&mut self.tex_positions, 0, 0);

        // self.alpha_texture_uniform.upload(&1);

        let ctxt = Context::get();
        // Draw the polygons in the correct way
        let _ = verify!(ctxt.polygon_mode(Context::FRONT_AND_BACK, Context::FILL));

        // Enable gl blending
        verify!(ctxt.enable(Context::BLEND));
        verify!(ctxt.blend_func(Context::SRC_ALPHA, Context::ONE_MINUS_SRC_ALPHA));

        // verify!(ctxt.active_texture(Context::TEXTURE0));
        verify!(ctxt.bind_texture(Context::TEXTURE_2D, Some(&self.alpha_texture)));

        // Set the point size
        ctxt.point_size(self.point_size);

        // TODO: Instead of drawing a series of points each point should a billboard center.
        // http://www.opengl-tutorial.org/intermediate-tutorials/billboards-particles/billboards/
        // https://solarianprogrammer.com/2013/05/17/opengl-101-textures/
        ctxt.draw_arrays(Context::TRIANGLES, 0, self.num_points() as i32);
        ctxt.draw_arrays(Context::POINTS, 0, self.num_points() as i32);

        verify!(ctxt.disable(Context::BLEND));

        self.pos_attribute.disable();
        self.color_attribute.disable();
        self.tex_pos_attributes.disable();
    }
}

/// Turn into blobs using: https://community.khronos.org/t/geometry-shader-point-sprite-to-sphere/63015
/// Easily turned into a discreet point again with a radius of 0 and a intensity drop of of 0.
///
/// The vertex shader still need the following parameters:
///  - uniform float gamma

/// Vertex shader used by the point renderer
const VERTEX_SHADER_SRC_3D: &'static str = "#version 460
    // Input to this shader
    layout (location = 0) in vec3 position;
    layout (location = 1) in vec3 color;
    layout (location = 2) in vec2 textureCoordinate;

    // Uniform variables for all vertices.
    uniform float gamma;
    uniform mat4 proj;
    uniform mat4 view;

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
        // change gamma with one, this is just a place holder so it does not
        // get optimized out.
        gl_Position = proj * view * vec4(position, gamma);

        // Make the color and tex coordinate available to the fragment shader.
        PointColor = hsv2rgb(color);
        TextureCoordinate = textureCoordinate;
    }";

/// Fragment shader used by the point renderer
const FRAGMENT_SHADER_SRC_3D: &'static str = "#version 460
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
        // Take the red (r) value of the rgba color
        float alpha = texture(alphaTexture, TextureCoordinate).a;
        if(alpha == 0.0)
            discard;
        FragColor = vec4(PointColor, alpha);
    }";
