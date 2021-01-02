//! Code containing the scene used by the future, this tracks the state of the entire program.

// Build in imports
use std::process::exit;

// Third party imports
use kiss3d::{
    camera::Camera,
    event::{Action, WindowEvent},
    light::Light,
    planar_camera::PlanarCamera,
    post_processing::PostProcessingEffect,
    renderer::{PlanarRenderer, Renderer},
    window::{CustomWindow, ExtendedState, RenderMode as WindowRenderMode},
};

// First party imports
use super::{
    ui::{UIEvents, UIState},
    ExplanationMode,
};
use crate::{
    exp,
    search::{PointContainer, PointContainer2D, PointContainer3D},
    view::{
        ui::draw_overlay, ColorMap, DimensionalityMode, PointRendererInteraction,
        VisualizationState2D, VisualizationState3D, VisualizationStateInteraction,
    },
};

// Easy access to buttons
mod buttons {
    use kiss3d::event::Key;
    // Switch between Discreet and Continuous
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
    // 2D state
    pub state_2d: Option<VisualizationState2D>,
    // 3D state
    pub state_3d: Option<VisualizationState3D>,
    // The state of the current UI which are not relevant for the rest
    // of the program
    pub ui_state: UIState,
    // Used when rendering, set to dirty if the window needs to be synced
    // with the scene
    dirty: bool,
}

impl Scene {
    // Create a new 3D visualization without a initializing the 2D view
    pub fn new(window: &mut CustomWindow) -> Scene {
        Scene {
            dimensionality_mode: DimensionalityMode::ThreeD,
            state_2d: None,
            state_3d: None,
            ui_state: UIState::new(window.conrod_ui_mut()),
            dirty: false,
        }
    }

    /// Load the 3D visualization state using the da silva explanations
    pub fn load_3d(&mut self, points_container: PointContainer3D) {
        self.state_3d = Some(VisualizationState3D::new(points_container));
        self.dimensionality_mode = DimensionalityMode::ThreeD;
        self.dirty = true;
    }

    /// Load the 2D visualization state using the da silva explanations
    pub fn load_2d(&mut self, points_container: PointContainer2D) {
        self.state_2d = Some(VisualizationState2D::new(points_container));
        self.dimensionality_mode = DimensionalityMode::TwoD;
        self.dirty = true;
    }

    /// Check if any data has been passed, we can not work without any reduction data
    pub fn initialized(&self) -> bool {
        !(self.state_2d.is_none() && self.state_3d.is_none())
    }

    pub fn dimension_switch_available(&self) -> bool {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => self.state_3d.is_some(),
            DimensionalityMode::ThreeD => self.state_2d.is_some(),
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

    /// Retrieve the current state
    pub fn current_state(&self) -> &dyn VisualizationStateInteraction {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => match &self.state_2d {
                Some(state) => state,
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
            DimensionalityMode::ThreeD => match &self.state_3d {
                Some(state) => state,
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
        }
    }

    /// Retrieve the current state as mutable
    pub fn current_state_mut(&mut self) -> &mut dyn VisualizationStateInteraction {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => match &mut self.state_2d {
                Some(state) => state,
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
            DimensionalityMode::ThreeD => match &mut self.state_3d {
                Some(state) => state,
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
        }
    }

    /// TODO: Move into VisualizationStateInteraction
    /// Get the point count of the currently used state
    pub fn get_point_count(&self) -> usize {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => match &self.state_2d {
                Some(state) => state.point_container.get_point_count(),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
            DimensionalityMode::ThreeD => match &self.state_3d {
                Some(state) => state.point_container.get_point_count(),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
        }
    }

    pub fn get_dimension_name(&self, index: &usize) -> Option<String> {
        match self.get_explanation_mode() {
            ExplanationMode::DaSilva(_) => match self.dimensionality_mode {
                DimensionalityMode::TwoD => match &self.state_2d {
                    Some(state) => state
                        .point_container
                        .dimension_names
                        .get(*index)
                        .and_then(|v| Some(v.clone())),
                    None => None,
                },
                DimensionalityMode::ThreeD => match &self.state_3d {
                    Some(state) => state
                        .point_container
                        .dimension_names
                        .get(*index)
                        .and_then(|v| Some(v.clone())),
                    None => None,
                },
            },
            ExplanationMode::VanDriel(_) => Some(format!("{} Dimension(s)", (index))),
            ExplanationMode::Normal => None,
            ExplanationMode::None => None,
        }
    }

    /// Disable the shading if the 3D state is available
    pub fn disable_shading(&mut self) {
        match &mut self.state_3d {
            Some(state) => state.disable_shading(),
            None => (),
        };
    }

    fn handle_ui_input(&mut self, ui_events: Vec<UIEvents>) {
        for event in ui_events {
            match event {
                UIEvents::ResetButtonPress => self.reset_camera(),
                UIEvents::RenderModeSwitch => self.switch_render_mode(),
                UIEvents::DimensionalitySwitch => self.switch_dimensionality(),
                // Setting scene options
                UIEvents::SetPointSize(size) => self.set_point_size(size),
                UIEvents::SetBlobSize(size) => self.set_blob_size(size),
                UIEvents::SetGamma(gamma) => self.set_gamma(gamma),
                UIEvents::SetColorBound(min, max) => self.set_color_map_confidence_bounds(min, max),
                UIEvents::SetExplanationMode(mode) => {
                    self.set_explanation_mode(mode);
                }
                // Running the explanation method
                UIEvents::RunExplanationMode(mode, neighborhood, theta) => {
                    self.ui_state.recompute_state.update(neighborhood);
                    self.run_explanation_mode(mode, neighborhood, theta)
                }
                // UI specific
                UIEvents::UpdateUINeighborhood(neighborhood) => {
                    self.ui_state.recompute_state.update(neighborhood)
                }
                UIEvents::UpdateUISwitchNeighborhood => {
                    self.ui_state.recompute_state.switch_neighborhood_type()
                }
                UIEvents::SwitchOpenMenu(v) => self.ui_state.open_menu = v,
                UIEvents::SetTheta(theta) => {
                    self.ui_state.theta = {
                        let t = theta.max(0.0).min(1.0);
                        (t * 200f32) as i32 as f32 / 200f32
                    }
                }
                UIEvents::ToggleConfidenceNormalization => {
                    self.toggle_color_map_confidence_normalization()
                }
                UIEvents::DisableShading => self.disable_shading(),
            }
        }
    }

    // TODO: handle is dirty case?
    fn handle_input(&mut self, window: &mut CustomWindow) {
        // Handle the window events
        for event in window.events().iter() {
            match event.value {
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
        let events = draw_overlay(self, window);
        self.handle_ui_input(events);
    }
}

/// All the interaction steps that can be done on the scene and are propegated to the
/// current state.
impl VisualizationStateInteraction for Scene {
    /// Retrieve the current renderer used to show the points
    fn current_render_mode(&self) -> &dyn PointRendererInteraction {
        self.current_state().current_render_mode()
    }

    /// Retrieve the current renderer used to show the points as mutable
    fn current_render_mode_mut(&mut self) -> &mut dyn PointRendererInteraction {
        self.current_state_mut().current_render_mode_mut()
    }

    /// Reset the camera view of the current rendering mode
    fn reset_camera(&mut self) {
        self.current_state_mut().reset_camera()
    }

    /// Run the explanation for this state and load it for the current state
    fn run_explanation_mode(
        &mut self,
        mode: ExplanationMode,
        neighborhood_size: exp::Neighborhood,
        theta: Option<f32>,
    ) {
        self.current_state_mut()
            .run_explanation_mode(mode, neighborhood_size, theta);
    }

    /// Check if a given explanation mode is already loaded for the state
    fn is_explanation_available(&self, mode: &ExplanationMode) -> bool {
        self.current_state().is_explanation_available(mode)
    }
    /// Get the current explanation mode
    fn get_explanation_mode(&self) -> ExplanationMode {
        self.current_state().get_explanation_mode()
    }
    /// Set the explanation mode of the state to `mode` for the current state
    fn set_explanation_mode(&mut self, mode: ExplanationMode) -> bool {
        self.current_state_mut().set_explanation_mode(mode)
    }

    /// Get the color map that is currently in use
    fn get_current_color_map(&self) -> &ColorMap {
        self.current_state().get_current_color_map()
    }
    /// Set the confidence bounds on the current color map
    fn set_color_map_confidence_bounds(&mut self, min: f32, max: f32) {
        self.current_state_mut()
            .set_color_map_confidence_bounds(min, max)
    }
    /// Toggle the confidence normalization in the current color map
    fn toggle_color_map_confidence_normalization(&mut self) {
        self.current_state_mut()
            .toggle_color_map_confidence_normalization()
    }

    /// Get the point count of the currently used state
    fn get_point_count(&self) -> usize {
        self.current_state().get_point_count()
    }
}

// State will not work for 2D scenes.
impl ExtendedState for Scene {
    // Return the required custom renderer that will be called at each
    // render loop.
    #[allow(clippy::type_complexity)]
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
        self.draw_overlay(&mut window);
        self.handle_input(&mut window);
    }
}

// Main display function
pub fn display(
    point_renderer_2d: Option<PointContainer2D>,
    point_renderer_3d: Option<PointContainer3D>,
) {
    // Create the window and set the background
    const WINDOW_WIDTH: u32 = 1024;
    const WINDOW_HEIGHT: u32 = 768;
    let mut window =
        CustomWindow::new_with_size("Pointctl visualizer", WINDOW_WIDTH, WINDOW_HEIGHT);
    window.set_background_color(1.0, 1.0, 1.0);
    window.set_light(Light::StickToCamera);

    // Create a scene with empty values
    let mut scene = Scene::new(&mut window);

    // Add the 2D points if they were provided
    if let Some(container) = point_renderer_2d {
        scene.load_2d(container);
    }

    // Add the 3D points if they were provided
    if let Some(container) = point_renderer_3d {
        scene.load_3d(container);
    }

    if scene.state_3d.is_none() && scene.state_2d.is_some() {
        window.switch_rendering_mode();
    }
    // Start the render loop.
    window.render_loop(scene)
}
