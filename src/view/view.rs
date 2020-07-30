extern crate kiss3d;
extern crate nalgebra as na;

// Third party
use kiss3d::camera::ArcBall;
use kiss3d::camera::Camera;
use kiss3d::event::{Action, WindowEvent};
use kiss3d::light::Light;
use kiss3d::planar_camera::{PlanarCamera, Sidescroll};
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::renderer::PlanarRenderer;
use kiss3d::renderer::Renderer;
use kiss3d::text::Font;
use kiss3d::window::{CustomWindow, ExtendedState};
// Conrod
use kiss3d::conrod::{widget, widget_ids, Color, Colorable, Positionable, Sizeable, Widget};
use na::{Point2, Point3};
use rstar::{PointDistance, RTree};

// First party
use super::color_map::ColorMap;
use super::point_renderer_2d::PointRenderer2D;
use super::point_renderer_3d::PointRenderer3D;
use crate::exp::common::{IndexedPoint2D, IndexedPoint3D, RTreeParameters2D, RTreeParameters3D};
use crate::exp::da_silva::DaSilvaExplanation;
use crate::util::types::PointN;

// Easy access to buttons
mod buttons {
    use kiss3d::event::Key;
    pub const GAMMA_UP_KEY: Key = Key::PageUp;
    pub const GAMMA_DOWN_KEY: Key = Key::PageDown;
    pub const SWITCH_RENDER_MODE: Key = Key::F;
    pub const SWITCH_DISCREET: Key = Key::M;
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

pub struct VisualizationState3D {
    // Camera used by this view. : Create custom camera .
    pub camera: ArcBall,
    // Useful when searching points that have been selected or clicked on.
    pub tree: RTree<IndexedPoint3D, RTreeParameters3D>,
    // Used for rendering points.
    pub renderer: PointRenderer3D,
    // Colour map used by the 3D visualizer
    pub color_map: ColorMap,
}

impl VisualizationState3D {
    fn new(
        points: Vec<Point3<f32>>,
        explanations: Vec<DaSilvaExplanation>,
        color_map: ColorMap,
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
            color_map: color_map,
        }
    }

    fn get_default_camera() -> ArcBall {
        // Create arcball camera with custom FOV.
        let eye = Point3::new(0.0f32, 0.0, -1.5);
        let at = Point3::new(0.0f32, 0.0f32, 0.0f32);
        let mut cam = ArcBall::new_with_frustrum(std::f32::consts::PI / 3.0, 0.01, 1024.0, eye, at);
        cam.set_dist_step(10.0);
        cam
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
    // Colour map used by the 3D visualizer
    pub color_map: ColorMap,
    // Used to denote if the 2d data is present. if not than this state will be empty
    pub initialized: bool,
}

impl VisualizationState2D {
    pub fn new_empty() -> VisualizationState2D {
        VisualizationState2D {
            camera: VisualizationState2D::get_default_camera(),
            tree: RTree::<IndexedPoint2D, RTreeParameters2D>::new_with_params(),
            renderer: PointRenderer2D::new(),
            color_map: ColorMap::new_dummy(),
            initialized: false,
        }
    }

    pub fn new(
        points: Vec<Point2<f32>>,
        explanations: Vec<DaSilvaExplanation>,
        color_map: ColorMap,
    ) -> VisualizationState2D {
        let mut state = VisualizationState2D::new_empty();
        state.initialize(points, explanations, color_map);
        state
    }

    pub fn initialize(
        &mut self,
        points: Vec<Point2<f32>>,
        explanations: Vec<DaSilvaExplanation>,
        color_map: ColorMap,
    ) {
        let indexed_points: Vec<IndexedPoint2D> = points
            .iter()
            .enumerate()
            .map(|(index, point)| IndexedPoint2D {
                index,
                x: point.x,
                y: point.y,
            })
            .collect();
        self.tree =
            RTree::<IndexedPoint2D, RTreeParameters2D>::bulk_load_with_params(indexed_points);
        // Ensure the renderer is empty.
        self.renderer.clear();
        // Then add all the points.
        for (&p, e) in points.iter().zip(explanations) {
            let color = color_map.get_colour(e.attribute_index, e.confidence);
            self.renderer.push(p, color);
        }

        // TODO: Is this even correct?
        let nn_distance = self.find_average_nearest_neightbor_distance();
        // println!("Average nn distance {:}", nn_distance);
        self.renderer.set_blob_size(nn_distance);

        self.initialized = true;
    }

    /// Find the average first nearest neighbor distance over all the points.
    fn find_average_nearest_neightbor_distance(&self) -> f32 {
        let mut res = Vec::<f32>::new();
        for query_point in self.tree.iter() {
            // Get the second nearest neighbor from the query point, the first will be itself.
            let &nn = self
                .tree
                .nearest_neighbor_iter(&[query_point.x, query_point.y])
                .take(2)
                .skip(1)
                .collect::<Vec<&IndexedPoint2D>>()
                .first()
                .expect("Could not get nearest neighbor");

            let dist = query_point.distance_2(&[nn.x, nn.y]).sqrt();
            res.push(dist);
        }
        res.iter().sum::<f32>() / (res.len() as f32)
    }

    // TODO: Get a good camera that just views all the points
    fn get_default_camera() -> Sidescroll {
        let mut cam = Sidescroll::new();
        cam.set_zoom(8.0);
        cam.set_zoom_step(2.7);
        cam
    }
}

pub struct Scene {
    // Render mode in which the visualization can be used
    pub render_mode: RenderMode,
    // Original ND data
    pub original_points: Vec<Vec<f32>>,
    // 3D state
    pub state_3d: VisualizationState3D,
    // 2D state
    pub state_2d: VisualizationState2D,
    // Used by conrod to assign widget ids
    pub conrod_ids: Ids,
}

impl Scene {
    // Create a new 3D visualization without a initializing the 2D view
    pub fn new(
        points_3d: Vec<Point3<f32>>,
        explanations: Vec<DaSilvaExplanation>,
        original_points: Vec<PointN>,
        conrod_ids: Ids,
    ) -> Scene {
        let dimension_count = original_points.first().unwrap().len();
        let color_map = ColorMap::from_explanations(&explanations, dimension_count);

        Scene {
            render_mode: RenderMode::ThreeD,
            state_3d: VisualizationState3D::new(points_3d, explanations, color_map),
            state_2d: VisualizationState2D::new_empty(),
            original_points: original_points,
            conrod_ids,
        }
    }

    // Creata a new 3D visualization with a 2D view.
    pub fn new_both(
        points_3d: Vec<Point3<f32>>,
        explanations_3d: Vec<DaSilvaExplanation>,
        points_2d: Vec<Point2<f32>>,
        explanations_2d: Vec<DaSilvaExplanation>,
        original_points: Vec<PointN>,
        conrod_ids: Ids,
    ) -> Scene {
        let dimension_count = original_points.first().unwrap().len();
        let color_map_2d = ColorMap::from_explanations(&explanations_2d, dimension_count);
        let color_map_3d = ColorMap::from_explanations(&explanations_3d, dimension_count);

        Scene {
            render_mode: RenderMode::ThreeD,
            state_3d: VisualizationState3D::new(points_3d, explanations_3d, color_map_3d),
            state_2d: VisualizationState2D::new(points_2d, explanations_2d, color_map_2d),
            original_points: original_points,
            conrod_ids,
        }
    }

    // Switch the render mode of the visualization if possible
    // You can not switch if the 2D view is not present.
    // Returns a boolean representing if the mode changed
    pub fn switch_render_mode(&mut self) -> bool {
        match self.render_mode {
            // You can always switch to 3D because 3D state is always present
            RenderMode::TwoD => {
                self.render_mode = RenderMode::ThreeD;
                true
            }
            // Only switch to 2D if the 2d state is available
            RenderMode::ThreeD => match self.state_2d.initialized {
                false => {
                    println!("Cannot switch to 2D since there is no 2D data loaded");
                    self.render_mode = RenderMode::ThreeD;
                    false
                }
                true => {
                    self.render_mode = RenderMode::TwoD;
                    true
                }
            },
        }
    }

    // TODO: handle is dirty case?
    fn handle_input(&mut self, window: &mut CustomWindow) {
        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(buttons::GAMMA_UP_KEY, Action::Press, _) => {
                    println!("Gamma up pressed")
                }
                WindowEvent::Key(buttons::GAMMA_DOWN_KEY, Action::Press, _) => {
                    println!("Gamma down pressed")
                }
                WindowEvent::Key(buttons::SWITCH_RENDER_MODE, Action::Press, _) => {
                    if self.switch_render_mode() {
                        window.switch_rendering_mode();
                    }
                }
                WindowEvent::Key(buttons::RESET_VIEW, Action::Press, _) => {
                    println!("Reset render mode");
                    match self.render_mode {
                        RenderMode::ThreeD => {
                            self.state_3d.camera = VisualizationState3D::get_default_camera()
                        }
                        RenderMode::TwoD => {
                            self.state_2d.camera = VisualizationState2D::get_default_camera()
                        }
                    }
                }
                WindowEvent::Key(buttons::SWITCH_DISCREET, Action::Press, _) => {
                    match self.render_mode {
                        RenderMode::ThreeD => {
                            print!("Only discreet rendering available in 3D");
                        }
                        RenderMode::TwoD => {
                            println!("Switching between discreet / continuos");
                            self.state_2d.renderer.switch_rendering_mode()
                        }
                    }
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

    fn draw_overlay(&mut self, window: &mut CustomWindow) {
        let num_points_text = format!("Number of points: {}", self.original_points.len());

        // use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
        // let mut conrod_ui = window.conrod_ui_mut().set_widgets();
        // widget::Text::new(&num_points_text)
        //     .color(Color::Rgba(1.0, 0.0, 0.0, 0.0))
        //     .set(self.conrod_ids.text, &mut conrod_ui);

        window.draw_text(
            &num_points_text,
            &Point2::new(0.0, 20.0),
            60.0,
            &Font::default(),
            &Point3::new(0.0, 0.0, 0.0),
        );
    }
}

// State will not work for 2D scenes.
impl ExtendedState for Scene {
    // Return the required custom renderer that will be called at each
    // render loop.
    fn cameras_and_effect_and_renderers(
        &mut self,
    ) -> (
        Option<&mut dyn Camera>,
        Option<&mut dyn PlanarCamera>,
        Option<&mut dyn Renderer>,
        Option<&mut dyn PlanarRenderer>,
        Option<&mut dyn PostProcessingEffect>,
    ) {
        (
            Some(&mut self.state_3d.camera),
            Some(&mut self.state_2d.camera),
            Some(&mut self.state_3d.renderer),
            Some(&mut self.state_2d.renderer),
            None,
        )
    }

    fn step(&mut self, mut window: &mut CustomWindow) {
        self.handle_input(&mut window);
        self.draw_overlay(&mut window);
    }
}

// Main display function
pub fn display(
    original_points: Vec<Vec<f32>>,
    points_3d: Vec<Point3<f32>>,
    explanations_3d: Vec<DaSilvaExplanation>,
    points_2d: Option<Vec<Point2<f32>>>,
    explanations_2d: Option<Vec<DaSilvaExplanation>>,
) {
    const WINDOW_WIDTH: u32 = 1024;
    const WINDOW_HEIGHT: u32 = 768;
    let mut window =
        CustomWindow::new_with_size("Pointctl visualizer", WINDOW_WIDTH, WINDOW_HEIGHT);
    window.set_background_color(1.0, 1.0, 1.0);
    window.set_light(Light::StickToCamera);

    let conrod_ids = Ids::new(window.conrod_ui_mut().widget_id_generator());

    // only init the scene with 2d points if both the points and explanations are provided
    let scene = match (points_2d, explanations_2d) {
        (Some(points), Some(explanations)) => Scene::new_both(
            points_3d,
            explanations_3d,
            points,
            explanations,
            original_points,
            conrod_ids,
        ),
        _ => Scene::new(points_3d, explanations_3d, original_points, conrod_ids),
    };

    // Start the render loop.
    window.render_loop(scene)
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        text,
        canvas
    }
}
