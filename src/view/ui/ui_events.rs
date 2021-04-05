//! File with an enum that represents all possible actions that can originate from the UI

// Internal imports
use super::ui_state::OpenSettingsMenu;
use crate::{exp::Neighborhood, view::ExplanationMode};

/// All the types of event that can happen in the UI.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UIEvents {
    // reset the camera
    ResetButtonPress,
    // Switch the dimension from 2d to 3d or vise versa
    DimensionalitySwitch,
    // Switch between continuous and discreet rendering modes
    RenderModeSwitch,
    // Set the point size of the current renderer (discreet mode)
    SetPointSize(f32),
    // Set the blob size of the current renderer (continuous mode)
    SetBlobSize(f32),
    // Set the gamma of the current renderer
    SetGamma(f32),
    // Change the color bounds
    SetColorBound(f32, f32),
    // Change the theta selected for the dimensionality based explanation
    SetTheta(f32),
    // Change the shading intensity
    SetShadingIntensity(f32),
    // Disable the shading
    DisableShading,
    // Load an explanation when it is already available
    SetExplanationMode(ExplanationMode),
    // Run an explanation
    RunExplanationMode(ExplanationMode, Neighborhood, Option<f32>),
    // Run / Load a single explanation
    SetSingleExplanationMode(ExplanationMode, Neighborhood),
    // Update the neighborhood in the UI menus
    UpdateUINeighborhood(Neighborhood),
    // Update the neighborhood in the UI menus
    UpdateUISwitchNeighborhood,
    // Change the current settings open in the menu
    SwitchOpenMenu(OpenSettingsMenu),
    // Turn of the normalization in the color confidence encoding
    ToggleConfidenceNormalization,
    // Override the dimension for a rank in the color mapping
    ResetRankOverrides,
    SetRankOverride(usize, usize),
}
