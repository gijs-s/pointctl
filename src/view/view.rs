extern crate kiss3d;
extern crate nalgebra as na;

// Third party
use crate::view::RenderMode;
use kiss3d::{
    camera::{ArcBall, Camera},
    event::{Action, WindowEvent},
    light::Light,
    planar_camera::{PlanarCamera, Sidescroll},
    post_processing::PostProcessingEffect,
    renderer::{PlanarRenderer, Renderer},
    window::{CustomWindow, ExtendedState, RenderMode as WindowRenderMode},
};
use na::{Point2, Point3};
use rstar::{PointDistance, RTree};

// First party
use crate::{
    exp::{
        common::{IndexedPoint2D, IndexedPoint3D, RTreeParameters2D, RTreeParameters3D},
        da_silva::DaSilvaExplanation,
    },
    util::types::PointN,
    view::{
        color_map::ColorMap,
        point_renderer_2d::PointRenderer2D,
        point_renderer_3d::PointRenderer3D,
        ui::{draw_overlay, WidgetId},
    },
};

// Easy access to buttons
mod buttons {
    use kiss3d::event::Key;
    pub const GAMMA_UP_KEY: Key = Key::PageUp;
    pub const GAMMA_DOWN_KEY: Key = Key::PageDown;
    // Switch between Discreet and Continous
    pub const SWITCH_RENDER_MODE: Key = Key::N;
    // Switch between 2D and 3D
    pub const SWITCH_DIMENSIONALITY: Key = Key::M;
    pub const RESET_VIEW: Key = Key::R;
    pub const QUIT: Key = Key::Q;
    pub const ESC: Key = Key::Escape;
}

// Rendering mode used by the program, you can only switch
// if two d data is provided.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DimensionalityMode {
    ThreeD,
    TwoD,
}

impl DimensionalityMode {
    // Convert the current value to a string
    pub fn to_str(self) -> String {
        match self {
            DimensionalityMode::TwoD => "2D".to_string(),
            DimensionalityMode::ThreeD => "3D".to_string(),
        }
    }

    /// Get the inverse of the current value
    pub fn inverse(self) -> Self {
        match self {
            DimensionalityMode::TwoD => DimensionalityMode::ThreeD,
            DimensionalityMode::ThreeD => DimensionalityMode::TwoD,
        }
    }
}

pub struct VisualizationState3D {
    // Camera used by this view. : Create custom camera .
    pub camera: ArcBall,
    // Useful when searching points that have been selected or clicked on.
    pub tree: RTree<IndexedPoint3D, RTreeParameters3D>,
    // Used for rendering points.
    pub renderer: PointRenderer3D,
    // color map used by the 3D visualizer
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
            let color = color_map.get_color(e.attribute_index, e.confidence);
            point_renderer.push(p, color);
        }


        let nn_distance = IndexedPoint3D::find_average_nearest_neightbor_distance(&rtree);
        // point_renderer.set_blob_size((nn_distance.powi(2) * 2.0).sqrt());
        point_renderer.set_blob_size(nn_distance);


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

pub struct VisualizationState2D {
    // Camera used by this view.
    pub camera: Sidescroll,
    // Useful when searching points that have been selected or clicked on.
    pub tree: RTree<IndexedPoint2D, RTreeParameters2D>,
    // Used for rendering points. TODO BUILD THIS
    pub renderer: PointRenderer2D,
    // color map used by the 3D visualizer
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
        // Initialize the search tree
        self.tree =
            RTree::<IndexedPoint2D, RTreeParameters2D>::bulk_load_with_params(indexed_points);

        // Initialize the color map
        self.color_map = ColorMap::from_explanations(&explanations, 30);

        // Ensure the renderer is empty.
        self.renderer.clear();

        // Then add all the points.
        for (&p, e) in points.iter().zip(explanations) {
            let color = color_map.get_color(e.attribute_index, e.confidence);
            self.renderer.push(p, color);
        }

        // TODO: Is this even correct?
        let nn_distance = IndexedPoint2D::find_average_nearest_neightbor_distance(&self.tree);
        self.renderer.set_blob_size((nn_distance.powi(2) * 2.0).sqrt());

        self.initialized = true;
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
    pub dimensionality_mode: DimensionalityMode,
    // Original ND data
    pub original_points: Vec<Vec<f32>>,
    // 3D state
    pub state_3d: VisualizationState3D,
    // 2D state
    pub state_2d: VisualizationState2D,
    // Used by conrod to assign widget ids
    pub conrod_ids: WidgetId,
    // Used when rendering, set to dirty if the window needs to be synced
    // with the scene
    dirty: bool,
}

impl Scene {
    // Create a new 3D visualization without a initializing the 2D view
    pub fn new(
        points_3d: Vec<Point3<f32>>,
        explanations: Vec<DaSilvaExplanation>,
        original_points: Vec<PointN>,
        conrod_ids: WidgetId,
    ) -> Scene {
        let dimension_count = original_points.first().unwrap().len();
        let color_map = ColorMap::from_explanations(&explanations, dimension_count);

        Scene {
            dimensionality_mode: DimensionalityMode::ThreeD,
            state_3d: VisualizationState3D::new(points_3d, explanations, color_map),
            state_2d: VisualizationState2D::new_empty(),
            original_points: original_points,
            conrod_ids,
            dirty: false,
        }
    }

    // Creata a new 3D visualization with a 2D view.
    pub fn new_both(
        points_3d: Vec<Point3<f32>>,
        explanations_3d: Vec<DaSilvaExplanation>,
        points_2d: Vec<Point2<f32>>,
        explanations_2d: Vec<DaSilvaExplanation>,
        original_points: Vec<PointN>,
        conrod_ids: WidgetId,
    ) -> Scene {
        let dimension_count = original_points.first().unwrap().len();
        let color_map_2d = ColorMap::from_explanations(&explanations_2d, dimension_count);
        let color_map_3d = ColorMap::from_explanations(&explanations_3d, dimension_count);

        Scene {
            dimensionality_mode: DimensionalityMode::ThreeD,
            state_3d: VisualizationState3D::new(points_3d, explanations_3d, color_map_3d),
            state_2d: VisualizationState2D::new(points_2d, explanations_2d, color_map_2d),
            original_points: original_points,
            conrod_ids,
            dirty: false,
        }
    }

    /// Reset the camera view of the current rendering mode
    pub fn reset_camera(&mut self) {
        match self.dimensionality_mode {
            DimensionalityMode::ThreeD => {
                self.state_3d.camera = VisualizationState3D::get_default_camera()
            }
            DimensionalityMode::TwoD => {
                self.state_2d.camera = VisualizationState2D::get_default_camera()
            }
        }
    }

    /// Switch the render mode of the visualization if possible
    /// You can not switch if the 2D view is not present.
    pub fn switch_dimensionality(&mut self) {
        match self.dimensionality_mode {
            // You can always switch to 3D because 3D state is always present
            DimensionalityMode::TwoD => {
                self.dimensionality_mode = DimensionalityMode::ThreeD;
                self.dirty = true;
            }
            // Only switch to 2D if the 2d state is available
            DimensionalityMode::ThreeD => match self.state_2d.initialized {
                false => {
                    println!("Cannot switch to 2D since there is no 2D data loaded");
                    self.dimensionality_mode = DimensionalityMode::ThreeD;
                }
                true => {
                    self.dimensionality_mode = DimensionalityMode::TwoD;
                    self.dirty = true
                }
            },
        }
    }

    /// Switch between rendering the continous and discreet point cloud representation
    pub fn switch_render_mode(&mut self) {
        match self.dimensionality_mode {
            DimensionalityMode::ThreeD => self.state_3d.renderer.switch_rendering_mode(),
            DimensionalityMode::TwoD => self.state_2d.renderer.switch_rendering_mode(),
        }
    }

    /// Get the current rendering mode
    pub fn get_current_render_mode(&self) -> &RenderMode {
        match self.dimensionality_mode {
            DimensionalityMode::ThreeD => &self.state_3d.renderer.render_mode,
            DimensionalityMode::TwoD => &self.state_2d.renderer.render_mode,
        }
    }

    /// Get the color map that is currently in use
    pub fn get_current_color_map(&self) -> &ColorMap {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => &self.state_2d.color_map,
            DimensionalityMode::ThreeD => &self.state_3d.color_map,
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
                WindowEvent::Key(buttons::SWITCH_DIMENSIONALITY, Action::Press, _) => {
                    self.switch_dimensionality();
                }
                WindowEvent::Key(buttons::RESET_VIEW, Action::Press, _) => {
                    println!("Reset render mode");
                    self.reset_camera();
                }
                WindowEvent::Key(buttons::SWITCH_RENDER_MODE, Action::Press, _) => {
                    self.switch_render_mode()
                }
                WindowEvent::Key(buttons::ESC, Action::Release, _)
                | WindowEvent::Key(buttons::QUIT, Action::Release, _)
                | WindowEvent::Close => {
                    window.close();
                }
                _ => (),
            }
        }

        // If the dirty flag is set than the dimensionality mode of the scene
        // and the window do not match, this will fix it.
        // XXX: This is hacky, I should patch this with a proper fix.
        if self.dirty {
            let window_dimensionality = match window.get_rendering_mode() {
                WindowRenderMode::TwoD => DimensionalityMode::TwoD,
                WindowRenderMode::ThreeD => DimensionalityMode::ThreeD,
            };
            if window_dimensionality != self.dimensionality_mode {
                window.switch_rendering_mode()
            };
            self.dirty = false;
        };
    }

    /// Draw the ui overlay given a scene state
    fn draw_overlay(&mut self, window: &mut CustomWindow) {
        // Split out the logic into a separate file to prevent this file
        // from becoming even more bloated
        draw_overlay(self, window);
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

    let conrod_ids = WidgetId::new(window.conrod_ui_mut().widget_id_generator());

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
