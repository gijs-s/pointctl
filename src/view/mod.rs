//! Module that wraps everything related to the viewer.

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
    visualization_state::{
        VisualizationState2D, VisualizationState3D, VisualizationStateInteraction,
    },
};

use crate::exp::{DaSilvaType, VanDrielType};

// Build in imports
use std::convert::TryFrom;

/// Dimensionality mode used by the program, determines which dimension the current viewer is in.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum DimensionalityMode {
    ThreeD,
    TwoD,
}

impl DimensionalityMode {
    // Convert the current value to a string
    pub fn to_string(&self) -> String {
        match &self {
            DimensionalityMode::TwoD => "2D".to_string(),
            DimensionalityMode::ThreeD => "3D".to_string(),
        }
    }
}

impl DimensionalityMode {
    /// Get the inverse of the current value
    pub fn inverse(self) -> Self {
        match self {
            DimensionalityMode::TwoD => DimensionalityMode::ThreeD,
            DimensionalityMode::ThreeD => DimensionalityMode::TwoD,
        }
    }
}

/// Explanation mode is used to denote which color map is currently being displayed
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ExplanationMode {
    None,
    DaSilva(DaSilvaType),
    VanDriel(VanDrielType),
    Normal,
}

impl ToString for ExplanationMode {
    // Convert the current value to a string
    fn to_string(&self) -> String {
        match &self {
            ExplanationMode::None => "None".to_string(),
            ExplanationMode::DaSilva(t) => format!("Da Silva ({})", t.to_string()),
            ExplanationMode::VanDriel(VanDrielType::MinimalVariance) => {
                "Van Driel (min)".to_string()
            }
            ExplanationMode::VanDriel(VanDrielType::TotalVariance) => {
                "Van Driel (total)".to_string()
            }
            ExplanationMode::Normal => "Normal".to_string(),
        }
    }
}

impl TryFrom<&str> for ExplanationMode {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "silva_euclidean" => Ok(ExplanationMode::DaSilva(DaSilvaType::Euclidean)),
            "silva_variance" => Ok(ExplanationMode::DaSilva(DaSilvaType::Variance)),
            "driel_sum" => Ok(ExplanationMode::VanDriel(VanDrielType::TotalVariance)),
            "driel_min" => Ok(ExplanationMode::VanDriel(VanDrielType::MinimalVariance)),
            "none" => Ok(ExplanationMode::None),
            "normal" => Ok(ExplanationMode::Normal),
            v => Err(format!("Could not create explanation mode from '{}'", v)),
        }
    }
}
