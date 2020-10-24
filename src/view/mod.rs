/// Module pertaining to everything that happens in the viewer

// Sub modules
mod color_map;
mod point_renderer;
mod scene;
mod ui;
mod visualization_state;

// Re-export the public facing parts of this module
pub use self::{
    color_map::ColorMap,
    point_renderer::{PointRenderer2D, PointRenderer3D, PointRendererInteraction, RenderMode},
    scene::{display, Scene},
    visualization_state::{VisualizationState2D, VisualizationState3D},
};

//Build in imports
use std::convert::TryFrom;

// Dimensionality mode used by the program, determines which dimension the current viewer is in.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

// Explanation mode is used to denote which color map is currently being displayed
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ExplanationMode {
    None,
    DaSilva,
    VanDriel,
}

impl ExplanationMode {
    // Convert the current value to a string
    pub fn to_str(self) -> String {
        match self {
            ExplanationMode::None => "None".to_string(),
            ExplanationMode::DaSilva => "Da Silva".to_string(),
            ExplanationMode::VanDriel => "Van Driel".to_string(),
        }
    }
}

impl TryFrom<&str> for ExplanationMode {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "silva" => Ok(ExplanationMode::DaSilva),
            "driel" => Ok(ExplanationMode::VanDriel),
            "none" => Ok(ExplanationMode::None),
            v => Err(format!("Could not create explanation mode from '{}'", v))
        }
    }
}
