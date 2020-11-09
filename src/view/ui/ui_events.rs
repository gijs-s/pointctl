//! File with an enum that represents all possible actions that can originate from the UI

// Internal imports
use super::ui_state::OpenSettingsMenu;
use crate::{exp::Neighborhood, view::ExplanationMode};

/// All the types of event that can happen in the UI.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UIEvents {
    ResetButtonPress,
    DimensionalitySwitch,
    RenderModeSwitch,
    SetPointSize(f32),
    SetBlobSize(f32),
    SetGamma(f32),
    SetColorBound(f32, f32),
    SetTheta(f32),
    SetExplanationMode(ExplanationMode),
    RunExplanationMode(ExplanationMode, Neighborhood, Option<f32>),
    UpdateUINeighborhood(Neighborhood),
    UpdateUISwitchNeighborhood,
    SwitchOpenMenu(OpenSettingsMenu),
    ToggleConfidenceNormalization,
}
