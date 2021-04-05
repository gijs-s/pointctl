/// Submodule containing the custom kiss3d renderers
mod marcos;
mod point_renderer_2d;
mod point_renderer_3d;
mod texture_creation;

pub use self::{point_renderer_2d::PointRenderer2D, point_renderer_3d::PointRenderer3D};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum RenderMode {
    Discreet,
    Continuous,
}

impl ToString for RenderMode {
    fn to_string(&self) -> String {
        match self {
            RenderMode::Discreet => "Discreet".to_string(),
            RenderMode::Continuous => "Continous".to_string(),
        }
    }
}

impl RenderMode {
    /// Get the inverse of the current value
    pub fn inverse(self) -> Self {
        match self {
            RenderMode::Discreet => RenderMode::Continuous,
            RenderMode::Continuous => RenderMode::Discreet,
        }
    }
}

pub trait PointRendererInteraction {
    /// Switch between continous and discreet rendering
    fn switch_render_mode(&mut self);
    /// Retrieve current rendering mode
    fn get_current_render_mode(&self) -> RenderMode;

    /// Get the gamma which will be used to next render loop
    fn get_gamma(&self) -> f32;
    /// Set the gamma which will be used to next render loop
    fn set_gamma(&mut self, gamma: f32);
    /// Reset the gamma
    fn reset_gamma(&mut self);
    /// Retrieve the default gamma value, alsmost always 2.2
    fn get_default_gamma(&self) -> f32 {
        2.0f32
    }

    /// Get/set the point size used in the discreet rendering
    fn get_point_size(&self) -> f32;
    /// Set the point size used in the discreet rendering
    fn set_point_size(&mut self, size: f32);
    /// Reset the point size back to its initial value
    fn reset_point_size(&mut self);
    /// Retrieve the point size's initial value
    fn get_default_point_size(&self) -> f32;

    /// Get the blob size used in the continous rendering
    fn get_blob_size(&self) -> f32;
    /// Set the blob size used in the continous rendering
    fn set_blob_size(&mut self, size: f32);
    /// Reset the blob size back to its initial value
    fn reset_blob_size(&mut self);
    /// Retrieve the blob size's initial value
    fn get_default_blob_size(&self) -> f32;

    /// Get the shading intensity
    fn get_shading_intensity(&self) -> f32 {
        self.get_default_shading_intensity()
    }
    /// Set the shading intensity
    fn set_shading_intensity(&mut self, _intensity: f32) {}
    /// Get the default shading intensity
    fn get_default_shading_intensity(&self) -> f32 {
        1.0f32
    }
}
