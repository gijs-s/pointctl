pub mod color_map;
pub mod view;

mod marcos;

mod texture_creation;
mod visualization_state;
mod point_renderer_2d;
mod point_renderer_3d;
mod ui;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RenderMode {
    Discreet,
    Continuous,
}

impl RenderMode {
    pub fn to_str(self) -> String {
        match self {
            RenderMode::Discreet => "Discreet".to_string(),
            RenderMode::Continuous => "Continous".to_string(),
        }
    }

    /// Get the inverse of the current value
    pub fn inverse(self) -> Self {
        match self {
            RenderMode::Discreet => RenderMode::Continuous,
            RenderMode::Continuous => RenderMode::Discreet,
        }
    }
}

// Dimensionality mode used by the program, determines which dimension the current viewer is in.
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

pub trait PointRendererInteraction {
    /// Switch between continous and discreet rendering
    fn switch_render_mode(&mut self);
    /// Retrieve current rendering mode
    fn get_current_render_mode(&self) -> RenderMode;

    /// Get the gamma which will be used to next render loop
    fn get_gamma(&self) -> f32;
    /// Set the gamma which will be used to next render loop
    fn set_gamma(&mut self, gamma: f32);

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

}