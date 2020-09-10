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