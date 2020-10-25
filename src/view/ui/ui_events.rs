// Internal imports
use crate::{
    exp::Neighborhood,
    view::ExplanationMode,
};
use super::ui_state::OpenSettingsMenu;

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
    SetExplanationMode(ExplanationMode),
    RunExplanationMode(ExplanationMode, Neighborhood),
    UpdateUINeighborhood(Neighborhood),
    UpdateUISwitchNeighborhood,
    SwitchOpenMenu(OpenSettingsMenu),
}