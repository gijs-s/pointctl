extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::camera::ArcBall;
use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::renderer::Renderer;
use kiss3d::resource::{
    AllocationType, BufferType, Effect, GPUVec, ShaderAttribute, ShaderUniform,
};
use kiss3d::text::Font;
use kiss3d::window::{State, Window};

use na::{Matrix4, Point2, Point3};

use crate::exp::da_silva::DaSilvaExplanation;
use std::cmp::Ordering;

pub fn display(title: &str, points: Vec<Point3<f32>>, explanations: Vec<DaSilvaExplanation>) {
    // Create the window
    let mut window = Window::new(title);
    window.set_background_color(1.0, 1.0, 1.0);

    let app = init_create_state(points, explanations);
    window.render_loop(app)
}

pub fn init_create_state(
    points: Vec<Point3<f32>>,
    explanations: Vec<DaSilvaExplanation>,
) -> AppState {
    let mut point_cloud_renderer = PointCloudRenderer::new(4.0);
    let max_confidence = explanations
        .iter()
        .map(|v| v.confidence)
        .max_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
        .unwrap();
    let min_confidence = explanations
        .iter()
        .map(|v| v.confidence)
        .min_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
        .unwrap();

    for (&p, e) in points.iter().zip(explanations) {
        let normalized_conf = (e.confidence - min_confidence) / (max_confidence - min_confidence);
        let color = Point3::<f32>::new(0.33f32 * e.attribute_index as f32, 1.0f32, normalized_conf);
        point_cloud_renderer.push(p, color);
    }

    // Create arcball camera with customer FOV.
    let eye = Point3::new(0.0f32, 0.0, -1.5);
    let at = Point3::new(0.0f32, 0.0f32, 0.0f32);
    let arc_ball = ArcBall::new_with_frustrum(std::f32::consts::PI / 3.0, 0.01, 1024.0, eye, at);

    AppState {
        camera: arc_ball,
        point_cloud_renderer,
    }
}

pub struct AppState {
    camera: ArcBall,
    point_cloud_renderer: PointCloudRenderer,
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
        (
            Some(&mut self.camera),
            None,
            Some(&mut self.point_cloud_renderer),
            None,
        )
    }

    fn step(&mut self, window: &mut Window) {
        let num_points_text = format!(
            "Number of points: {}",
            self.point_cloud_renderer.num_points()
        );
        window.draw_text(
            &num_points_text,
            &Point2::new(0.0, 20.0),
            60.0,
            &Font::default(),
            &Point3::new(1.0, 1.0, 1.0),
        );
    }
}

/// Structure which manages the display of long-living points.
struct PointCloudRenderer {
    shader: Effect,
    pos: ShaderAttribute<Point3<f32>>,
    color: ShaderAttribute<Point3<f32>>,
    proj: ShaderUniform<Matrix4<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
    colored_points: GPUVec<Point3<f32>>,
    point_size: f32,
}

impl PointCloudRenderer {
    /// Creates a new points renderer.
    fn new(point_size: f32) -> PointCloudRenderer {
        let mut shader = Effect::new_from_str(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);

        shader.use_program();

        PointCloudRenderer {
            colored_points: GPUVec::new(Vec::new(), BufferType::Array, AllocationType::StreamDraw),
            pos: shader.get_attrib::<Point3<f32>>("position").unwrap(),
            color: shader.get_attrib::<Point3<f32>>("color").unwrap(),
            proj: shader.get_uniform::<Matrix4<f32>>("proj").unwrap(),
            view: shader.get_uniform::<Matrix4<f32>>("view").unwrap(),
            shader,
            point_size,
        }
    }

    fn push(&mut self, point: Point3<f32>, color: Point3<f32>) {
        if let Some(colored_points) = self.colored_points.data_mut() {
            colored_points.push(point);
            colored_points.push(color);
        }
    }

    fn num_points(&self) -> usize {
        self.colored_points.len() / 2
    }
}

impl Renderer for PointCloudRenderer {
    /// Actually draws the points.
    fn render(&mut self, pass: usize, camera: &mut dyn Camera) {
        if self.colored_points.len() == 0 {
            return;
        }

        self.shader.use_program();
        self.pos.enable();
        self.color.enable();

        camera.upload(pass, &mut self.proj, &mut self.view);

        self.color.bind_sub_buffer(&mut self.colored_points, 1, 1);
        self.pos.bind_sub_buffer(&mut self.colored_points, 1, 0);

        let ctxt = Context::get();
        ctxt.point_size(self.point_size);
        ctxt.draw_arrays(Context::POINTS, 0, (self.colored_points.len() / 2) as i32);

        self.pos.disable();
        self.color.disable();
    }
}

const VERTEX_SHADER_SRC: &'static str = "#version 100
    attribute vec3 position;
    attribute vec3 color;
    varying   vec3 Color;
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

const FRAGMENT_SHADER_SRC: &'static str = "#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif

    varying vec3 Color;
    void main() {
        gl_FragColor = vec4(Color, 1.0);
    }";
