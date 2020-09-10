pub mod color_map;
pub mod view;

mod marcos;

mod texture_creation;
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
