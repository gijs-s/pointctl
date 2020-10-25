use crate::exp::Neighborhood;
use super::widget_ids::{InfoWidgetId, LegendWidgetId, MenuWidgetId};

// Move this into ui_state.rs
pub struct UIState {
    // All the widget ids seperated into different classes
    pub info_widgets: InfoWidgetId,
    pub legend_widgets: LegendWidgetId,
    pub menu_widgets: MenuWidgetId,
    // Enum that denotes which menu is currently being displayed
    pub open_menu: OpenSettingsMenu,
    // State used for the recompute button
    pub recompute_state: RecomputeButtonState,
}

impl UIState {
    pub fn new(ui: &mut kiss3d::conrod::Ui) -> Self {
        UIState {
            info_widgets: InfoWidgetId::new(ui.widget_id_generator()),
            legend_widgets: LegendWidgetId::new(ui.widget_id_generator()),
            menu_widgets: MenuWidgetId::new(ui.widget_id_generator()),
            open_menu: OpenSettingsMenu::ViewerSettings,
            recompute_state: RecomputeButtonState::new(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpenSettingsMenu {
    ViewerSettings,
    ExplanationSettings,
    None
}

pub struct RecomputeButtonState {
    pub neighborhood_type: NeighborhoodType,
    // Neighborhood size 10...50
    pub k: usize,
    // Radius between 0.0...1.0
    pub r: f32,
}

impl RecomputeButtonState {
    pub fn new() -> RecomputeButtonState {
        RecomputeButtonState {
            neighborhood_type: NeighborhoodType::R,
            k: 30,
            r: 0.1,
        }
    }

    /// Get a text representation of the current neighborhood
    pub fn get_neighborhood_text(&self) -> String {
        match self.neighborhood_type {
            NeighborhoodType::R => {
                let mut r_text = self.r.to_string();
                r_text.truncate(5);
                format!("R: {}", r_text)
            }
            NeighborhoodType::K => format!("K: {}", self.k.to_string()),
        }
    }

    pub fn switch_neighborhood_type(&mut self) {
        self.neighborhood_type = self.neighborhood_type.inverse()
    }

    /// Update the current object using a new neighborhood
    pub fn update(&mut self, neighborhood: Neighborhood) {
        match neighborhood {
            Neighborhood::K(k) => self.k = k,
            Neighborhood::R(r) => {
                // Round  the value to 0.005
                let rounded = (r * 200f32) as i32 as f32 / 200f32;
                self.r = rounded
            }
        }
    }
}

impl From<&RecomputeButtonState> for Neighborhood {
    fn from(state: &RecomputeButtonState) -> Neighborhood {
        match state.neighborhood_type {
            NeighborhoodType::K => Neighborhood::K(state.k),
            NeighborhoodType::R => Neighborhood::R(state.r),
        }
    }
}

// Small enum used to denote which neighborhood type is currently being used.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NeighborhoodType {
    K,
    R,
}

impl NeighborhoodType {
    pub fn inverse(self) -> Self {
        match self {
            NeighborhoodType::K => NeighborhoodType::R,
            NeighborhoodType::R => NeighborhoodType::K,
        }
    }
}

impl ToString for NeighborhoodType {
    fn to_string(&self) -> String {
        match self {
            NeighborhoodType::K => "K".to_string(),
            NeighborhoodType::R => "R".to_string(),
        }
    }
}