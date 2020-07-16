extern crate nalgebra as na;

// Third party
use super::marcos;
use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::renderer::PlanarRenderer;
use kiss3d::renderer::Renderer;
use kiss3d::resource::{
    AllocationType, BufferType, Effect, GPUVec, ShaderAttribute, ShaderUniform,
};
use na::{Matrix3, Point2, Point3};

/// 2D
#[allow(dead_code)]
pub struct PointRenderer2D {
    shader: Effect,
    pos: ShaderAttribute<Point2<f32>>,
    color: ShaderAttribute<Point3<f32>>,
    view: ShaderUniform<Matrix3<f32>>,
    proj: ShaderUniform<Matrix3<f32>>,
    points: GPUVec<Point2<f32>>,
    colors: GPUVec<Point3<f32>>,
    point_size: f32,
    visible: bool,
}

#[allow(dead_code)]
impl PointRenderer2D {
    pub fn new() -> PointRenderer2D {
        let mut shader = Effect::new_from_str(VERTEX_SHADER_SRC_2D, FRAGMENT_SHADER_SRC_2D);

        shader.use_program();

        PointRenderer2D {
            points: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            colors: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            // Shader variables
            pos: shader
                .get_attrib::<Point2<f32>>("position")
                .expect("Failed to get shader attribute."),
            color: shader
                .get_attrib::<Point3<f32>>("color")
                .expect("Failed to get shader attribute."),
            proj: shader
                .get_uniform::<Matrix3<f32>>("proj")
                .expect("Failed to get shader attribute."),
            view: shader
                .get_uniform::<Matrix3<f32>>("view")
                .expect("Failed to get shader attribute."),
            // Shader itself
            shader,
            // GL variables
            point_size: 4.0,
            visible: true,
        }
    }

    /// Insert a single point with a color
    pub fn push(&mut self, point: Point2<f32>, color: Point3<f32>) {
        for points in self.points.data_mut().iter_mut() {
            points.push(point);
        }
        for colors in self.colors.data_mut().iter_mut() {
            colors.push(color)
        }
    }

    /// Clear all the points
    pub fn clear(&mut self) {
        for points in self.points.data_mut().iter_mut() {
            points.clear()
        }
        for color in self.colors.data_mut().iter_mut() {
            color.clear()
        }
    }

    /// Indicates whether some points have to be drawn.
    pub fn needs_rendering(&self) -> bool {
        self.points.len() != 0 && self.visible
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
        self.points.len()
    }
}

impl PlanarRenderer for PointRenderer2D {
    /// Actually draws the lines.
    fn render(&mut self, camera: &mut dyn PlanarCamera) {
        if !self.needs_rendering() {
            return;
        }

        self.shader.use_program();
        self.pos.enable();
        self.color.enable();

        camera.upload(&mut self.proj, &mut self.view);

        self.color.bind_sub_buffer(&mut self.colors, 0, 0);
        self.pos.bind_sub_buffer(&mut self.points, 0, 0);

        let ctxt = Context::get();
        ctxt.draw_arrays(Context::POINTS, 0, self.points.len() as i32);
        assert_eq!(ctxt.get_error(), 0);

        self.pos.disable();
        self.color.disable();
    }
}

const VERTEX_SHADER_SRC_2D: &'static str = "#version 100
    attribute vec2 position;
    attribute vec3 color;
    varying   vec3 vColor;
    uniform   mat3 proj;
    uniform   mat3 view;

    void main() {
        vec3 projected_pos = proj * view * vec3(position, 1.0);
        projected_pos.z = 0.0;

        gl_Position = vec4(projected_pos, 1.0);
        vColor = color;
    }";

const FRAGMENT_SHADER_SRC_2D: &'static str = "#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif

    varying vec3 vColor;
    void main() {
        gl_FragColor = vec4(vColor, 1.0);
    }";
