extern crate kiss3d;
extern crate nalgebra as na;

// Third party
use crate::util::types::PointN;
use kiss3d::camera::ArcBall;
use kiss3d::camera::Camera;
use kiss3d::event::{Action, WindowEvent};
use kiss3d::light::Light;
use kiss3d::planar_camera::{PlanarCamera, Sidescroll};
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::renderer::Renderer;
use kiss3d::text::Font;
use kiss3d::window::{State, Window};
use na::{Point2, Point3};
use rstar::RTree;

// First party
use super::point_renderer::{PointRenderer2D, PointRenderer3D};
use crate::exp::common::{IndexedPoint2D, IndexedPoint3D, RTreeParameters2D, RTreeParameters3D};
use crate::exp::da_silva::DaSilvaExplanation;

// Buildin
use std::collections::HashMap;

// Easy access to buttons
mod buttons {
    use kiss3d::event::Key;
    pub const GAMMA_UP_KEY: Key = Key::A;
    pub const GAMMA_DOWN_KEY: Key = Key::B;
    pub const SWITCH_RENDER_MODE: Key = Key::F;
    pub const RESET_VIEW: Key = Key::R;
    pub const QUIT: Key = Key::Q;
    pub const ESC: Key = Key::Escape;
}

// Rendering mode used by the program, you can only switch
// if two d data is provided.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RenderMode {
    ThreeD,
    TwoD,
}

// Everything related to the colours in the visualization
pub struct ColorMap {
    // Map of dimension to colour
    map: HashMap<usize, Point3<f32>>,
    // Min max values for the confidence, used for normalization
    normalization_bounds: (f32, f32),
    // gamma correction value, static for now
    gamma: f32,
}

impl ColorMap {
    // TODO: Create the actual colour map
    pub fn new(
        min_confidence: f32,
        max_confidence: f32,
        _dimension_ranking: Vec<usize>,
    ) -> ColorMap {
        ColorMap {
            map: HashMap::new(),
            normalization_bounds: (min_confidence, max_confidence),
            gamma: 2.2,
        }
    }

    // Get a RGB colour based on the current pallet
    pub fn get_colour(&self, dimension: usize, confidence: f32) -> Point3<f32> {
        let normalized_conf = confidence
            - self.normalization_bounds.0
                / (self.normalization_bounds.1 - self.normalization_bounds.0);
        // TODO: Put in an actual colour map
        let base_color = match self.map.get(&dimension) {
            Some(c) => c.clone(),
            None => Point3::new(0.0f32, 0.0, 0.0),
        };
        ColorMap::scale_color(normalized_conf, base_color)
    }

    // Scale a color in rgb / hsv.
    fn scale_color(scale: f32, color: Point3<f32>) -> Point3<f32> {
        // TODO: add scaling function to dim a color.
        unimplemented!()
    }
}

pub struct VisualizationState3D {
    // Camera used by this view. : Create custom camera .
    pub camera: ArcBall,
    // Useful when searching points that have been selected or clicked on.
    pub tree: RTree<IndexedPoint3D, RTreeParameters3D>,
    // Used for rendering points.
    pub renderer: PointRenderer3D,
}

impl VisualizationState3D {
    fn new(
        points: Vec<Point3<f32>>,
        explanations: Vec<DaSilvaExplanation>,
        color_map: &ColorMap,
    ) -> VisualizationState3D {
        // Create the tree
        let indexed_points: Vec<IndexedPoint3D> = points
            .iter()
            .enumerate()
            .map(|(index, point)| IndexedPoint3D {
                index,
                x: point.x,
                y: point.y,
                z: point.z,
            })
            .collect();
        let rtree =
            RTree::<IndexedPoint3D, RTreeParameters3D>::bulk_load_with_params(indexed_points);

        // Create the renderer and add all the points:
        let mut point_renderer = PointRenderer3D::new();
        for (&p, e) in points.iter().zip(explanations) {
            let color = color_map.get_colour(e.attribute_index, e.confidence);
            point_renderer.push(p, color);
        }

        VisualizationState3D {
            camera: VisualizationState3D::get_default_camera(),
            tree: rtree,
            renderer: point_renderer,
        }
    }

    fn get_default_camera() -> ArcBall {
        // Create arcball camera with custom FOV.
        let eye = Point3::new(0.0f32, 0.0, -1.5);
        let at = Point3::new(0.0f32, 0.0f32, 0.0f32);
        ArcBall::new_with_frustrum(std::f32::consts::PI / 3.0, 0.01, 1024.0, eye, at)
    }
}

#[allow(dead_code)]
pub struct VisualizationState2D {
    // Camera used by this view.
    pub camera: Sidescroll,
    // Useful when searching points that have been selected or clicked on.
    pub tree: RTree<IndexedPoint2D, RTreeParameters2D>,
    // Used for rendering points. TODO BUILD THIS
    pub renderer: PointRenderer2D,
}

#[allow(dead_code)]
impl VisualizationState2D {
    fn new(points: Vec<Point2<f32>>) -> VisualizationState2D {
        unimplemented!()
    }
}

pub struct Scene {
    // Render mode in which the visualization can be used
    pub render_mode: RenderMode,
    // Original ND data
    pub original_points: Vec<Vec<f32>>,
    // 3D state, required
    pub state_3d: VisualizationState3D,
    // 2D state: Optional
    pub state_2d: Option<VisualizationState2D>,
    // Used for decided what color each point should get given its dimension and confidence.
    pub colors: ColorMap,
}

impl Scene {
    // Create a new 3D visualization
    pub fn new(
        points: Vec<Point3<f32>>,
        original_points: Vec<PointN>,
        explanations: Vec<DaSilvaExplanation>,
    ) -> Scene {
        let dimension_count = original_points.first().unwrap().len();
        let color_map = ColorMap::new(
            DaSilvaExplanation::min_confidence(&explanations),
            DaSilvaExplanation::max_confidence(&explanations),
            DaSilvaExplanation::calculate_dimension_rankings(dimension_count, &explanations),
        );
        let state = VisualizationState3D::new(points, explanations, &color_map);

        Scene {
            render_mode: RenderMode::ThreeD,
            state_3d: state,
            state_2d: None,
            original_points: original_points,
            colors: color_map,
        }
    }

    // Add the 2D view, allow switching to it.
    pub fn add_2d_view(&mut self) {
        unimplemented!()
    }

    // Switch the render mode of the visualization if possible
    // You can not switch if the 2D view is not present.
    pub fn switch_render_mode(&mut self) {
        self.render_mode = match self.render_mode {
            // You can always switch to 3D because 3D state is always present
            RenderMode::TwoD => RenderMode::ThreeD,
            // Only switch to 2D if the 2d state is available
            RenderMode::ThreeD => match self.state_2d {
                None => RenderMode::ThreeD,
                Some(_) => RenderMode::TwoD,
            },
        };
    }

    // TODO: handle is dirty case?
    fn handle_input(&mut self, window: &mut Window) {
        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(buttons::GAMMA_UP_KEY, Action::Press, _) => {
                    println!("Gamma up pressed")
                }
                WindowEvent::Key(buttons::GAMMA_DOWN_KEY, Action::Press, _) => {
                    println!("Gamma up pressed")
                }
                WindowEvent::Key(buttons::SWITCH_RENDER_MODE, Action::Press, _) => {
                    self.switch_render_mode()
                }
                WindowEvent::Key(buttons::RESET_VIEW, Action::Press, _) => {
                    // TODO: add 2d reset
                    self.state_3d.camera = VisualizationState3D::get_default_camera();
                }
                WindowEvent::Key(buttons::ESC, Action::Release, _)
                | WindowEvent::Key(buttons::QUIT, Action::Release, _)
                | WindowEvent::Close => {
                    window.close();
                }
                _ => (),
            }
        }
    }

    fn draw_overlay(&mut self, window: &mut Window) {
        let num_points_text = format!("Number of points: {}", self.original_points.len());
        window.draw_text(
            &num_points_text,
            &Point2::new(0.0, 20.0),
            60.0,
            &Font::default(),
            &Point3::new(1.0, 1.0, 1.0),
        );
    }
}

impl State for Scene {
    // Return the required custom renderer that will be called at each
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
            Some(&mut self.state_3d.camera),
            None,
            Some(&mut self.state_3d.renderer),
            None,
        )
    }

    fn step(&mut self, mut window: &mut Window) {
        self.handle_input(&mut window);
        self.draw_overlay(&mut window);
    }
}

// Main display function
pub fn display(
    original_points: Vec<Vec<f32>>,
    points_3d: Vec<Point3<f32>>,
    explanations: Vec<DaSilvaExplanation>,
    _point_2d: Option<Vec<Point2<f32>>>,
) {
    const WINDOW_WIDTH: u32 = 1024;
    const WINDOW_HEIGHT: u32 = 768;
    let mut window = Window::new_with_size("Pointctl visualizer", WINDOW_WIDTH, WINDOW_HEIGHT);
    window.set_background_color(1.0, 1.0, 1.0);
    window.set_light(Light::StickToCamera);
    window.set_point_size(4.);

    // Fix this init
    let scene = Scene::new(points_3d, original_points, explanations);

    // TODO add 2d points in present

    // Start the render loop
    window.render_loop(scene)
}
