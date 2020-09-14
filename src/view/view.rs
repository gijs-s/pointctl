extern crate kiss3d;
extern crate nalgebra as na;

// Third party
use std::process::exit;

use crate::view::PointRendererInteraction;
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
    exp::da_silva::DaSilvaExplanation,
    util::types::PointN,
    view::{
        color_map::ColorMap,
        point_renderer_2d::PointRenderer2D,
        point_renderer_3d::PointRenderer3D,
        ui::{draw_overlay, WidgetId},
        visualization_state::{VisualizationState2D, VisualizationState3D},
        DimensionalityMode, RenderMode,
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

pub struct Scene {
    // Render mode in which the visualization can be used
    pub dimensionality_mode: DimensionalityMode,
    // Original ND data
    pub original_points: Vec<Vec<f32>>,
    // 2D state
    pub state_2d: Option<VisualizationState2D>,
    // 3D state
    pub state_3d: Option<VisualizationState3D>,
    // Used by conrod to assign widget ids
    pub conrod_ids: WidgetId,
    // Used when rendering, set to dirty if the window needs to be synced
    // with the scene
    dirty: bool,
}

impl Scene {
    // Create a new 3D visualization without a initializing the 2D view
    pub fn new(original_points: Vec<PointN>, conrod_ids: WidgetId) -> Scene {
        Scene {
            dimensionality_mode: DimensionalityMode::ThreeD,
            state_2d: None,
            state_3d: None,
            original_points: original_points,
            conrod_ids,
            dirty: false,
        }
    }

    /// Load the 3D visualization state using the da silva explanations
    pub fn load_3d(&mut self, points: Vec<Point3<f32>>, explanations: Vec<DaSilvaExplanation>) {
        self.state_3d = Some(VisualizationState3D::new(
            points,
            explanations,
            &self.original_points,
        ));
        self.dimensionality_mode = DimensionalityMode::ThreeD;
        self.dirty = true;
    }

    /// Load the 2D visualization state using the da silva explanations
    pub fn load_2d(&mut self, points: Vec<Point2<f32>>, explanations: Vec<DaSilvaExplanation>) {
        self.state_2d = Some(VisualizationState2D::new(
            points,
            explanations,
            &self.original_points,
        ));
        self.dimensionality_mode = DimensionalityMode::TwoD;
        self.dirty = true;
    }

    /// Check if any data has been passed, we can not work without any reduction data
    pub fn initialized(&self) -> bool {
        !(self.state_2d.is_none() && self.state_3d.is_none())
    }

    /// Reset the camera view of the current rendering mode
    pub fn reset_camera(&mut self) {
        match self.dimensionality_mode {
            DimensionalityMode::ThreeD => match &mut self.state_3d {
                Some(state) => state.camera = VisualizationState3D::get_default_camera(),
                None => (),
            },
            DimensionalityMode::TwoD => match &mut self.state_2d {
                Some(state) => state.camera = VisualizationState2D::get_default_camera(),
                None => (),
            },
        }
    }

    /// Switch the render mode of the visualization if possible.
    /// You can not switch if the accompanying state is not available.
    pub fn switch_dimensionality(&mut self) {
        match self.dimensionality_mode {
            // Only switch to 2D if the 2d state is available
            DimensionalityMode::TwoD => match self.state_3d.is_some() {
                true => {
                    self.dimensionality_mode = DimensionalityMode::ThreeD;
                    self.dirty = true;
                }
                false => println!("Cannot switch to 3D since there is no 3D state loaded"),
            },
            // Only switch to 3D if the 3D state is available
            DimensionalityMode::ThreeD => match self.state_2d.is_some() {
                true => {
                    self.dimensionality_mode = DimensionalityMode::TwoD;
                    self.dirty = true
                }
                false => println!("Cannot switch to 2D since there is no 2D state loaded"),
            },
        }
    }

    /// Retrieve the current rendering mode for interaction.
    pub fn current_render_mode(&self) -> Box<&dyn PointRendererInteraction> {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => match &self.state_2d {
                Some(state) => Box::new(&state.renderer),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
            DimensionalityMode::ThreeD => match &self.state_3d {
                Some(state) => Box::new(&state.renderer),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
        }
    }

    /// Retrieve the current rendering mode for interaction.
    pub fn current_render_mode_mut(&mut self) -> Box<&mut dyn PointRendererInteraction> {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => match &mut self.state_2d {
                Some(state) => Box::new(&mut state.renderer),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
            DimensionalityMode::ThreeD => match &mut self.state_3d {
                Some(state) => Box::new(&mut state.renderer),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
        }
    }

    /// Switch between rendering the continous and discreet point cloud representation
    pub fn switch_render_mode(&mut self) {
        self.current_render_mode_mut().switch_render_mode();
    }

    /// Get the current rendering mode
    pub fn get_current_render_mode(&self) -> RenderMode {
        self.current_render_mode().get_current_render_mode()
    }

    /// Get the color map that is currently in use
    pub fn get_current_color_map(&self) -> &ColorMap {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => match &self.state_2d {
                Some(state) => &state.color_map,
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
            DimensionalityMode::ThreeD => match &self.state_3d {
                Some(state) => &state.color_map,
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
        }
    }

    /// Get the current point size
    pub fn get_point_size(&self) -> f32 {
        self.current_render_mode().get_point_size()
    }

    /// Set the point size
    pub fn set_point_size(&mut self, size: f32) {
        self.current_render_mode_mut().set_point_size(size);
    }

    /// Get the current point size
    pub fn get_default_point_size(&self) -> f32 {
        self.current_render_mode().get_default_point_size()
    }

    /// Get the current blob size
    pub fn get_blob_size(&self) -> f32 {
        self.current_render_mode().get_blob_size()
    }

    /// Set the blob size
    pub fn set_blob_size(&mut self, size: f32) {
        self.current_render_mode_mut().set_blob_size(size);
    }

    /// Get the default blob size
    pub fn get_default_blob_size(&self) -> f32 {
        self.current_render_mode().get_default_blob_size()
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
                    if self.initialized() {
                        self.switch_dimensionality();
                    }
                }
                WindowEvent::Key(buttons::RESET_VIEW, Action::Press, _) => {
                    if self.initialized() {
                        self.reset_camera();
                    }
                }
                WindowEvent::Key(buttons::SWITCH_RENDER_MODE, Action::Press, _) => {
                    if self.initialized() {
                        self.switch_render_mode()
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
        let (camera, renderer): (Option<&mut dyn Camera>, Option<&mut dyn Renderer>) =
            match &mut self.state_3d {
                Some(state) => (Some(&mut state.camera), Some(&mut state.renderer)),
                None => (None, None),
            };
        let (planar_camera, planar_renderer): (
            Option<&mut dyn PlanarCamera>,
            Option<&mut dyn PlanarRenderer>,
        ) = match &mut self.state_2d {
            Some(state) => (Some(&mut state.camera), Some(&mut state.renderer)),
            None => (None, None),
        };
        (camera, planar_camera, renderer, planar_renderer, None)
    }

    fn step(&mut self, mut window: &mut CustomWindow) {
        self.handle_input(&mut window);
        self.draw_overlay(&mut window);
    }
}

// Main display function
pub fn display(
    original_points: Vec<Vec<f32>>,
    points_2d: Option<Vec<Point2<f32>>>,
    explanations_2d: Option<Vec<DaSilvaExplanation>>,
    points_3d: Option<Vec<Point3<f32>>>,
    explanations_3d: Option<Vec<DaSilvaExplanation>>,
) {
    // Create the window and set the background
    const WINDOW_WIDTH: u32 = 1024;
    const WINDOW_HEIGHT: u32 = 768;
    let mut window =
        CustomWindow::new_with_size("Pointctl visualizer", WINDOW_WIDTH, WINDOW_HEIGHT);
    window.set_background_color(1.0, 1.0, 1.0);
    window.set_light(Light::StickToCamera);

    // Generate widget ids for the conrod ui to use
    let conrod_ids = WidgetId::new(window.conrod_ui_mut().widget_id_generator());

    // Create a scene with empty values
    let mut scene = Scene::new(original_points, conrod_ids);

    // Add the 2D points if they were provided
    if points_2d.is_some() && explanations_2d.is_some() {
        scene.load_2d(points_2d.unwrap(), explanations_2d.unwrap())
    }

    // Add the 3D points if they were provided
    if points_3d.is_some() && explanations_3d.is_some() {
        scene.load_3d(points_3d.unwrap(), explanations_3d.unwrap())
    }

    // Start the render loop.
    window.render_loop(scene)
}
