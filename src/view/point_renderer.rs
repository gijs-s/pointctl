extern crate kiss3d;
extern crate nalgebra as na;

// Third party
use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::renderer::Renderer;
use kiss3d::resource::{
    AllocationType, BufferType, Effect, GPUVec, ShaderAttribute, ShaderUniform,
};
use na::{Matrix3, Matrix4, Point2, Point3};

/// 3D
pub struct PointRenderer3D {
    shader: Effect,
    pos: ShaderAttribute<Point3<f32>>,
    color: ShaderAttribute<Point3<f32>>,
    gamma: ShaderAttribute<f32>,
    view: ShaderUniform<Matrix4<f32>>,
    proj: ShaderUniform<Matrix4<f32>>,
    points: GPUVec<Point3<f32>>,
    point_size: f32,
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
                .expect("Failed to get shader attribute."),
            color: shader
                .get_attrib::<Point3<f32>>("color")
                .expect("Failed to get shader attribute."),
            gamma: shader
                .get_attrib::<f32>("gamma")
                .expect("Failed to get shader attribute."),
            proj: shader
                .get_uniform::<Matrix4<f32>>("proj")
                .expect("Failed to get shader attribute."),
            view: shader
                .get_uniform::<Matrix4<f32>>("view")
                .expect("Failed to get shader attribute."),
            // Shader itself
            shader,
            // GL variables
            point_size: 4.0,
        }
    }

    // TODO: Batch insert?
    pub fn push(&mut self, point: Point3<f32>, color: Point3<f32>) {
        if let Some(colored_points) = self.points.data_mut() {
            colored_points.push(point);
            colored_points.push(color);
        }
    }

    /// Indicates whether some points have to be drawn.
    pub fn needs_rendering(&self) -> bool {
        self.points.len() != 0
    }

    // Set the point size
    pub fn set_point_size(&mut self, point_size: f32) {
        self.point_size = point_size;
    }

    // Retrieve the number of points
    pub fn num_points(&self) -> usize {
        self.points.len() / 2
    }
}

impl Renderer for PointRenderer3D {
    /// Actually draws the points.
    fn render(&mut self, pass: usize, camera: &mut dyn Camera) {
        if self.points.len() == 0 {
            return;
        }

        self.shader.use_program();
        self.pos.enable();
        self.color.enable();

        camera.upload(pass, &mut self.proj, &mut self.view);

        self.color.bind_sub_buffer(&mut self.points, 1, 1);
        self.pos.bind_sub_buffer(&mut self.points, 1, 0);

        let ctxt = Context::get();
        ctxt.point_size(self.point_size);
        ctxt.draw_arrays(Context::POINTS, 0, self.num_points() as i32);

        self.pos.disable();
        self.color.disable();
    }
}

/// Vertex shader used by the point renderer
const VERTEX_SHADER_SRC_3D: &'static str = "#version 100
    attribute vec3 position;
    attribute vec3 color;
    varying   vec3 Color;
    attribute float gamma;
    uniform   mat4 proj;
    uniform   mat4 view;

    // All components are in the range [0…1], including hue.
    vec3 hsv2rgb(vec3 c)
    {
        vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
        vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
        return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
    }

    void main() {
        gl_Position = proj * view * vec4(position, 1.0);
        Color = hsv2rgb(color);
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

/// 2D
#[allow(dead_code)]
pub struct PointRenderer2D {
    shader: Effect,
    pos: ShaderAttribute<Point2<f32>>,
    color: ShaderAttribute<Point3<f32>>,
    gamma: ShaderAttribute<f32>,
    view: ShaderUniform<Matrix3<f32>>,
    proj: ShaderUniform<Matrix3<f32>>,
    points: GPUVec<Point2<f32>>,
    colors: GPUVec<Point3<f32>>,
    point_size: f32,
}

#[allow(dead_code)]
impl PointRenderer2D {
    fn new() -> PointRenderer2D {
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
            gamma: shader
                .get_attrib::<f32>("gamma")
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
        }
    }

    /// Indicates whether some points have to be drawn.
    pub fn needs_rendering(&self) -> bool {
        self.points.len() != 0
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

#[allow(dead_code)]
impl PointRenderer2D {
    /// Actually draws the lines.
    fn render(&mut self, _pass: usize, camera: &mut dyn PlanarCamera) {
        if self.points.len() == 0 {
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
