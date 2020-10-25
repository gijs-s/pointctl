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
        ui::draw_overlay, ColorMap, DimensionalityMode, PointRendererInteraction, RenderMode,
        VisualizationState2D, VisualizationState3D,
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

    /// Check if a given explanation mode is already loaded for the current state
    pub fn is_explanation_available(&self, mode: &ExplanationMode) -> bool {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => match &self.state_2d {
                Some(state) => state.is_explanation_available(&mode),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
            DimensionalityMode::ThreeD => match &self.state_3d {
                Some(state) => state.is_explanation_available(&mode),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
        }
    }

    pub fn get_explanation_mode(&self) -> ExplanationMode {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => match &self.state_2d {
                Some(state) => state.get_explanation_mode(),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
            DimensionalityMode::ThreeD => match &self.state_3d {
                Some(state) => state.get_explanation_mode(),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
        }
    }

    pub fn set_explanation_mode(&mut self, mode: ExplanationMode) {
        let success = match self.dimensionality_mode {
            DimensionalityMode::TwoD => match &mut self.state_2d {
                Some(state) => state.set_explanation_mode(mode),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
            DimensionalityMode::ThreeD => match &mut self.state_3d {
                Some(state) => state.set_explanation_mode(mode),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
        };
        if !success {
            println!("Switching to rendering mode failed")
        };
    }

    /// TODO: Move running the explanation mode into the state
    pub fn run_explanation_mode(
        &mut self,
        mode: ExplanationMode,
        neighborhood: exp::Neighborhood,
        theta: Option<f32>,
    ) {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => match &mut self.state_2d {
                Some(state) => state.run_explanation_mode(mode, neighborhood, theta),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
            DimensionalityMode::ThreeD => match &mut self.state_3d {
                Some(state) => state.run_explanation_mode(mode, neighborhood, theta),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
        }
    }

    /// Retrieve the current rendering mode for interaction.
    pub fn current_render_mode(&self) -> &dyn PointRendererInteraction {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => match &self.state_2d {
                Some(state) => &state.renderer,
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
            DimensionalityMode::ThreeD => match &self.state_3d {
                Some(state) => &state.renderer,
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
        }
    }

    /// Retrieve the current rendering mode for interaction.
    pub fn current_render_mode_mut(&mut self) -> &mut dyn PointRendererInteraction {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => match &mut self.state_2d {
                Some(state) => &mut state.renderer,
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
            DimensionalityMode::ThreeD => match &mut self.state_3d {
                Some(state) => &mut state.renderer,
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
        }
    }

    /// Get the color map that is currently in use
    pub fn get_current_color_map(&self) -> &ColorMap {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => match &self.state_2d {
                Some(state) => &state.get_color_map(),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
            DimensionalityMode::ThreeD => match &self.state_3d {
                Some(state) => &state.get_color_map(),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
        }
    }

    /// Set the confidence bounds on the current color map
    pub fn set_color_map_confidence_bounds(&mut self, min: f32, max: f32) {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => match &mut self.state_2d {
                Some(state) => state.set_color_confidence_bounds(min, max),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
            DimensionalityMode::ThreeD => match &mut self.state_3d {
                Some(state) => state.set_color_confidence_bounds(min, max),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
        }
    }

    /// Toggle the confidence normalization in the current color map
    pub fn toggle_color_map_confidence_normalization(&mut self) {
        match self.dimensionality_mode {
            DimensionalityMode::TwoD => match &mut self.state_2d {
                Some(state) => state.toggle_color_confidence_normalization(),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
            DimensionalityMode::ThreeD => match &mut self.state_3d {
                Some(state) => state.toggle_color_confidence_normalization(),
                None => {
                    eprint!("There is no state available for the Dimensionality the scene is set to, this should not be possible");
                    exit(41);
                }
            },
        }
    }

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
            ExplanationMode::DaSilva => match self.dimensionality_mode {
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
            ExplanationMode::VanDriel => Some(format!("{} Dimension(s)", (index + 1))),
            ExplanationMode::None => None,
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

    /// Get the gamma which will be used to next render loop
    pub fn get_gamma(&self) -> f32 {
        self.current_render_mode().get_gamma()
    }

    /// Set the gamma which will be used to next render loop
    pub fn set_gamma(&mut self, gamma: f32) {
        self.current_render_mode_mut().set_gamma(gamma);
    }

    /// Get the default gamma value
    pub fn get_default_gamma(&self) -> f32 {
        self.current_render_mode().get_default_gamma()
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
                UIEvents::SetExplanationMode(mode) => self.set_explanation_mode(mode),
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
                UIEvents::ToggleConfidenceNormalization => self.toggle_color_map_confidence_normalization()
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
