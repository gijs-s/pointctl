extern crate kiss3d;
extern crate nalgebra as na;

// Third party
use super::marcos;
use crate::view::color_map::ColorMap;
use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::renderer::Renderer;
use kiss3d::resource::{
    AllocationType, BufferType, Effect, GPUVec, ShaderAttribute, ShaderUniform,
};
use na::{Matrix3, Matrix4, Point3};
use gl;

/// 3D
pub struct PointRenderer3D {
    shader: Effect,
    pos: ShaderAttribute<Point3<f32>>,
    color: ShaderAttribute<Point3<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
    proj: ShaderUniform<Matrix4<f32>>,
    gamma_uniform: ShaderUniform<f32>,
    points: GPUVec<Point3<f32>>,
    point_size: f32,
    gamma: f32,
    visible: bool,
}

impl PointRenderer3D {
    pub fn new() -> PointRenderer3D {
        let mut shader = Effect::new_from_str(VERTEX_SHADER_SRC_3D, FRAGMENT_SHADER_SRC_3D);
        shader.use_program();

        PointRenderer3D {
            // Points with colours interleaved.
            points: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            // Shader variables
            pos: shader
                .get_attrib::<Point3<f32>>("position")
                .expect("Failed to get `position` shader attribute."),
            color: shader
                .get_attrib::<Point3<f32>>("color")
                .expect("Failed to get `color` shader attribute."),
            gamma_uniform: shader
                .get_uniform::<f32>("gamma")
                .expect("Could not get `gamma` shader attribute"),
            proj: shader
                .get_uniform::<Matrix4<f32>>("proj")
                .expect("Failed to get `proj` shader attribute."),
            view: shader
                .get_uniform::<Matrix4<f32>>("view")
                .expect("Failed to get `view` shader attribute."),
            // Shader itself
            shader,
            // GL variables
            point_size: 4.0,
            // Gamma variable
            gamma: 1.0,
            // Variable to set when skipping all rendering while keeping data loaded.
            visible: true,
        }
    }

    /// Insert a single point with a color
    pub fn push(&mut self, point: Point3<f32>, color: Point3<f32>) {
        for points in self.points.data_mut().iter_mut() {
            points.push(point);
            points.push(color);
        }
    }

    /// Insert a large number of points with colors at once
    pub fn batch_insert(&mut self, points_x_colors: Vec<(Point3<f32>, Point3<f32>)>) {
        for points_buffer in self.points.data_mut().iter_mut() {
            for &(point, color) in points_x_colors.iter() {
                points_buffer.push(point);
                points_buffer.push(color);
            }
        }
    }

    /// Clear all the points
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
    pub fn set_gamma(&mut self, gamma: f32){
        self.gamma = gamma;
    }

    // Retrieve the number of points
    pub fn num_points(&self) -> usize {
        self.points.len() / 2
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
        self.pos.enable();
        self.color.enable();

        self.gamma_uniform.upload(&self.gamma);
        camera.upload(pass, &mut self.proj, &mut self.view);

        self.color.bind_sub_buffer(&mut self.points, 1, 1);
        self.pos.bind_sub_buffer(&mut self.points, 1, 0);

        let ctxt = Context::get();
        // Enable gl blending
        verify!(ctxt.enable(gl::BLEND));
        verify!(ctxt.blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
        ctxt.point_size(self.point_size);
        ctxt.draw_arrays(Context::POINTS, 0, self.num_points() as i32);

        verify!(ctxt.disable(gl::BLEND));
        self.pos.disable();
        self.color.disable();
    }
}

/// Turn into blobs using: https://community.khronos.org/t/geometry-shader-point-sprite-to-sphere/63015
/// Easily turned into a discreet point again with a radius of 0 and a intensity drop of of 0.
///
/// The vertex shader still need the following parameters:
///  - uniform float gamma

/// Vertex shader used by the point renderer
const VERTEX_SHADER_SRC_3D: &'static str = "#version 100
    attribute vec3 position;
    attribute vec3 color;
    varying   vec3 Color;
    uniform   float billboard_size;
    uniform   float billboard_dropoff;
    uniform   float gamma;
    uniform   mat4 proj;
    uniform   mat4 view;

    // All components are in the range [0…1], including hue.
    vec3 hsv2rgb(vec3 c, float g)
    {
        vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
        vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
        return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
    }

    void main() {
        gl_Position = proj * view * vec4(position, gamma);
        Color = hsv2rgb(color, gamma);
    }";

/// Fragment shader used by the point renderer
const FRAGMENT_SHADER_SRC_3D: &'static str = "#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif

    varying vec3 Color;
    void main() {
        gl_FragColor = vec4(Color, 1.0);
    }";
