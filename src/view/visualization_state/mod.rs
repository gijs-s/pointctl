//! Module containing the current state of the 2D or 3D renderer.
mod state_2d;
mod state_3d;

pub use self::{state_2d::VisualizationState2D, state_3d::VisualizationState3D};

/// First party imports
use crate::{exp::Neighborhood, search::UIPointData, view::{ColorMap, ExplanationMode, PointRendererInteraction, RenderMode}};

/// Common functions used to interact with the scene
pub trait VisualizationStateInteraction {
    /// Retrieve the current renderer used to show the points
    fn current_render_mode(&self) -> &dyn PointRendererInteraction;

    /// Retrieve the current renderer used to show the points as mutable
    fn current_render_mode_mut(&mut self) -> &mut dyn PointRendererInteraction;

    /// Reset the camera view of the current rendering mode
    fn reset_camera(&mut self);

    /// Run the explanation for this state and load it.
    fn run_explanation_mode(
        &mut self,
        mode: ExplanationMode,
        neighborhood_size: Neighborhood,
        theta: Option<f32>,
    );

    /// Check if a given explanation mode is already loaded for the state
    fn is_explanation_available(&self, mode: &ExplanationMode) -> bool;
    /// Get the current explanation mode
    fn get_explanation_mode(&self) -> ExplanationMode;
    /// Set the explanation mode of the state to `mode`
    fn set_explanation_mode(&mut self, mode: ExplanationMode) -> bool;

    /// Get the color map that is currently in use
    fn get_current_color_map(&self) -> &ColorMap;
    /// Set the confidence bounds on the current color map
    fn set_color_map_confidence_bounds(&mut self, min: f32, max: f32);
    /// Toggle the confidence normalization in the current color map
    fn toggle_color_map_confidence_normalization(&mut self);

    /// Set an override to from rank to a dimension
    fn set_rank_dimension_override(&mut self, rank: usize, dimension: usize);
    /// reset the overrides
    fn reset_rank_overrides(&mut self);

    /// Get the point count of the state
    fn get_point_count(&self) -> usize;

    /// Scale the current camera step size
    fn scale_camera_step(&mut self, scale: f32);

    // Get closed point to cursor position
    fn get_point_tooltip(&self, cursor_x: f32, cursor_y: f32, window_size: na::Vector2<f32>) -> Option<UIPointData>;
}

impl<T> PointRendererInteraction for T
where
    T: VisualizationStateInteraction,
{
    /// Switch between rendering the continous and discreet point cloud representation
    fn switch_render_mode(&mut self) {
        self.current_render_mode_mut().switch_render_mode();
    }

    /// Get the current rendering mode
    fn get_current_render_mode(&self) -> RenderMode {
        self.current_render_mode().get_current_render_mode()
    }

    /// Get the gamma which will be used to next render loop
    fn get_gamma(&self) -> f32 {
        self.current_render_mode().get_gamma()
    }

    /// Set the gamma which will be used to next render loop
    fn set_gamma(&mut self, gamma: f32) {
        self.current_render_mode_mut().set_gamma(gamma);
    }

    /// Reset the gamma
    fn reset_gamma(&mut self) {
        self.current_render_mode_mut().reset_blob_size()
    }

    /// Get the default gamma value
    fn get_default_gamma(&self) -> f32 {
        self.current_render_mode().get_default_gamma()
    }

    /// Get the current point size
    fn get_point_size(&self) -> f32 {
        self.current_render_mode().get_point_size()
    }

    /// Set the point size
    fn set_point_size(&mut self, size: f32) {
        self.current_render_mode_mut().set_point_size(size);
    }

    /// Reset the point size back to its initial value
    fn reset_point_size(&mut self) {
        self.current_render_mode_mut().reset_point_size();
    }

    /// Get the current point size
    fn get_default_point_size(&self) -> f32 {
        self.current_render_mode().get_default_point_size()
    }

    /// Get the current blob size
    fn get_blob_size(&self) -> f32 {
        self.current_render_mode().get_blob_size()
    }

    /// Set the blob size
    fn set_blob_size(&mut self, size: f32) {
        self.current_render_mode_mut().set_blob_size(size);
    }

    /// Reset the blob size
    fn reset_blob_size(&mut self) {
        self.current_render_mode_mut().reset_blob_size();
    }

    /// Get the default blob size
    fn get_default_blob_size(&self) -> f32 {
        self.current_render_mode().get_default_blob_size()
    }

    /// Get the shading intensity
    fn get_shading_intensity(&self) -> f32 {
        self.current_render_mode().get_shading_intensity()
    }

    /// Set the shading intensity
    fn set_shading_intensity(&mut self, intensity: f32) {
        self.current_render_mode_mut()
            .set_shading_intensity(intensity);
    }

    /// Get the default shading intensity
    fn get_default_shading_intensity(&self) -> f32 {
        self.current_render_mode().get_default_shading_intensity()
    }
}
