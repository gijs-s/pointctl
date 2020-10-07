extern crate kiss3d;
extern crate nalgebra as na;

// Third party
use kiss3d::{
    conrod::{widget, widget_ids, Color, Colorable, Labelable, Positionable, Sizeable, Widget},
    window::CustomWindow,
};

// Internal imports
use super::scene::Scene;
use crate::{
    exp::Neighborhood,
    view::{ColorMap, DimensionalityMode, ExplanationMode, RenderMode},
};

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct WidgetId {
        // Information widgets
        text_point_count,
        text_dimensionality,
        text_render_mode,
        text_explanation_mode,
        text_error,
        // Buttons controlling the viewer
        button_reset,
        button_dimension_switch,
        buttom_render_mode,
        // Widgets used for the color legend
        canvas_color_legend,
        text_dim_0,
        color_block_0,
        text_dim_1,
        color_block_1,
        text_dim_2,
        color_block_2,
        text_dim_3,
        color_block_3,
        text_dim_4,
        color_block_4,
        text_dim_5,
        color_block_5,
        text_dim_6,
        color_block_6,
        text_dim_7,
        color_block_7,
        text_dim_other,
        color_block_other,
        // Settings panel for the current renderer.
        // - size of the blobs / points
        text_size_slider,
        button_size_reset,
        slider_point_size,
        slider_blob_size,
        // - gamma slider
        text_gamma_slider,
        button_gamma_reset,
        slider_gamma,
        // - Calculate da silva / van driel button
        button_explanation_1,
        button_explanation_2,
        // - Recompute the current metric
        text_recompute,
        button_recompute,
        button_switch_neighborhood_type,
        slider_neighborhood,
    }
}

const FONT_SIZE: u32 = 12;
const FONT_SIZE_SMALL: u32 = FONT_SIZE - 4;
const BUTTON_WIDTH: f64 = 120.0f64;
const BUTTON_HEIGHT: f64 = 18.0f64;
const SLIDER_HEIGHT: f64 = 18.0f64;
const COLOR_PREVIEW_SIZE: f64 = 18.0f64;
const GAMMA_MIN_MAX: (f32, f32) = (1.0, 3.4);
const NEIGHBORHOOD_R_MIN_MAX: (f32, f32) = (0.01, 1.1);
const NEIGHBORHOOD_K_MIN_MAX: (usize, usize) = (10, 250);

/// All the types of event that can happen in the UI.
#[derive(Copy, Clone, Debug, PartialEq)]
enum UIEvents {
    ResetButtonPress,
    DimensionalitySwitch,
    RenderModeSwitch,
    SetPointSize(f32),
    SetBlobSize(f32),
    SetGamma(f32),
    SetExplanationMode(ExplanationMode),
    RunExplanationMode(ExplanationMode, Neighborhood),
    UpdateUINeighborhood(Neighborhood),
    UpdateUISwitchNeighborhood,
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
// Struct that contains data about the UI state that are not relevant to the state itself
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct UIState {
    pub neighborhood_type: NeighborhoodType,
    // Neighborhood size 10...50
    pub k: usize,
    // Radius between 0.0...1.0
    pub r: f32,
}

impl UIState {
    /// Initialize the UI state
    pub fn new() -> Self {
        UIState {
            neighborhood_type: NeighborhoodType::R,
            k: 30,
            r: 0.1,
        }
    }

    /// Get a text representation of the current neigborhood
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

impl Into<Neighborhood> for UIState {
    fn into(self: UIState) -> Neighborhood {
        match self.neighborhood_type {
            NeighborhoodType::K => Neighborhood::K(self.k),
            NeighborhoodType::R => Neighborhood::R(self.r),
        }
    }
}

/// Draw an overlay in the window of the given scene
pub fn draw_overlay(scene: &mut Scene, window: &mut CustomWindow) {
    // Get a mutable reference to the conrod ui
    let ids = &scene.conrod_ids;
    let mut ui = window.conrod_ui_mut().set_widgets();

    // ######################################################################
    // # Draw the basic information about the current view in the left top. #
    // ######################################################################

    // Display the current explanation mode
    let explanation_mode_text = format!(
        "Explanation mode: {}",
        scene.get_explanation_mode().to_str()
    );
    widget::Text::new(&explanation_mode_text)
        .font_size(FONT_SIZE)
        .top_left()
        .color(Color::Rgba(0.0, 0.0, 0.0, 1.0))
        .set(ids.text_explanation_mode, &mut ui);

    // Display the current rendering mode
    let render_mode = scene.get_current_render_mode().to_str();
    let render_mode_text = format!("Render mode: {}", render_mode);
    widget::Text::new(&render_mode_text)
        .font_size(FONT_SIZE)
        .down_from(ids.text_explanation_mode, 5.0f64)
        .color(Color::Rgba(0.0, 0.0, 0.0, 1.0))
        .set(ids.text_render_mode, &mut ui);

    // Display the current reduction dimensionality
    let dimensionality_text = format!("Reduced to: {}", scene.dimensionality_mode.to_str());
    widget::Text::new(&dimensionality_text)
        .font_size(FONT_SIZE)
        .down_from(ids.text_render_mode, 5.0f64)
        .color(Color::Rgba(0.0, 0.0, 0.0, 1.0))
        .set(ids.text_dimensionality, &mut ui);

    // Display the amount of points
    let num_points_text = format!("Point count: {}", scene.original_points.len());
    widget::Text::new(&num_points_text)
        .font_size(FONT_SIZE)
        .down_from(ids.text_dimensionality, 5.0f64)
        .color(Color::Rgba(0.0, 0.0, 0.0, 1.0))
        .set(ids.text_point_count, &mut ui);

    // Draw error if no data is present
    if !scene.initialized() {
        widget::Text::new("No reduction data was loaded, can not display anything\nplease consult 'pointctl --help'")
            .font_size(FONT_SIZE * 3u32)
            .middle()
            .color(Color::Rgba(0.80, 0.33, 0.0, 1.0 ))
            .set(ids.text_error, &mut ui);
        return;
    }

    // ###########################################
    // # Draw the color legend in the top right #
    // ###########################################

    // Retrieve the color map from the correct state
    let color_map = scene.get_current_color_map();
    // No need to render the color legend if the color map is empty
    if !color_map.is_initialized() {
        widget::Text::new("No explanation loaded\nall points are grey")
            .font_size(FONT_SIZE)
            .top_right()
            .set(ids.text_dim_0, &mut ui);
    } else {
        let color = color_map.get_conrod_color_with_gamma(&0usize, scene.get_gamma());
        widget::Canvas::new()
            .top_right_with_margin(5.0f64)
            .w(COLOR_PREVIEW_SIZE)
            .h(COLOR_PREVIEW_SIZE)
            .color(color)
            .set(ids.color_block_0, &mut ui);

        let dim = color_map.get_dimension_from_rank(&0usize).unwrap();
        let text = format!("[{}] {}", dim, scene.get_dimension_name(dim).unwrap());
        widget::Text::new(&text)
            .font_size(FONT_SIZE_SMALL)
            .left_from(ids.color_block_0, 2.0f64)
            .w_of(ids.text_point_count)
            .set(ids.text_dim_0, &mut ui);

        // All the ids used from drawing
        // Here the first entry is the one we offset the current ui element from
        // the second and third are the actual ui element ids.
        let dimensions = vec![
            (ids.color_block_0, ids.color_block_1, ids.text_dim_1),
            (ids.color_block_1, ids.color_block_2, ids.text_dim_2),
            (ids.color_block_2, ids.color_block_3, ids.text_dim_3),
            (ids.color_block_3, ids.color_block_4, ids.text_dim_4),
            (ids.color_block_4, ids.color_block_5, ids.text_dim_5),
            (ids.color_block_5, ids.color_block_6, ids.text_dim_6),
            (ids.color_block_6, ids.color_block_7, ids.text_dim_7),
            (ids.color_block_7, ids.color_block_other, ids.text_dim_other),
        ];

        for (index, &(offset_id, preview_id, text_id)) in dimensions
            .iter()
            // The first dimension is already drawn
            .take(color_map.dimension_count() - 1)
            .enumerate()
        {
            // Enumerate is 0 indexed, so we add 1 to get the correct offset.
            let rank = &index + 1usize;
            // First draw the color preview with the correct color.
            let color = color_map.get_conrod_color_with_gamma(&rank, scene.get_gamma());

            widget::Canvas::new()
                .down_from(offset_id, 3.0)
                .w(COLOR_PREVIEW_SIZE)
                .h(COLOR_PREVIEW_SIZE)
                .color(color)
                .set(preview_id, &mut ui);

            let text = {
                if index == 7usize {
                    "Other dimensions".to_string()
                } else {
                    let dim = color_map.get_dimension_from_rank(&rank).unwrap();
                    format!("[{}] {}", dim, scene.get_dimension_name(dim).unwrap())
                }
            };

            widget::Text::new(&text)
                .font_size(FONT_SIZE_SMALL)
                .left_from(preview_id, 2.0f64)
                .w_of(ids.text_point_count)
                .set(text_id, &mut ui);
        }
    }

    // ##################################################
    // # Draw all the buttoms and slides on the screen. #
    // ##################################################

    // Create an event queue, the scene cannot be used immutably for the ids and then
    // mutated after. We need to render all the widgets and then we can process and
    // mutate the scene.
    let mut queue: Vec<UIEvents> = Vec::new();

    // Add buttons for switching and running explanation modes
    // First gather the possible text and event for each button
    let (text_none, event_none) = (
        "Turn off annotations".to_string(),
        UIEvents::SetExplanationMode(ExplanationMode::None),
    );
    let (text_da_silva, event_da_silva) =
        match scene.is_explanation_available(&ExplanationMode::DaSilva) {
            true => (
                "View to Da Silva".to_string(),
                UIEvents::SetExplanationMode(ExplanationMode::DaSilva),
            ),
            false => (
                "Calculate Da Silva".to_string(),
                UIEvents::RunExplanationMode(ExplanationMode::DaSilva, scene.ui_state.into()),
            ),
        };
    let (text_van_driel, event_van_driel) =
        match scene.is_explanation_available(&ExplanationMode::VanDriel) {
            true => (
                "View Van Driel".to_string(),
                UIEvents::SetExplanationMode(ExplanationMode::VanDriel),
            ),
            false => (
                "Calculate Van Driel".to_string(),
                UIEvents::RunExplanationMode(ExplanationMode::VanDriel, scene.ui_state.into()),
            ),
        };

    // Only show the 2 other options, switching to the mode you are already in does not make sense.
    let (text_1, event_1, text_2, event_2) = match scene.get_explanation_mode() {
        ExplanationMode::None => (
            text_van_driel,
            event_van_driel,
            text_da_silva,
            event_da_silva,
        ),
        ExplanationMode::DaSilva => (text_van_driel, event_van_driel, text_none, event_none),
        ExplanationMode::VanDriel => (text_da_silva, event_da_silva, text_none, event_none),
    };

    for _ in widget::Button::new()
        .label(&text_1)
        .label_font_size(FONT_SIZE_SMALL)
        .bottom_left_with_margin(5.0f64)
        .w(BUTTON_WIDTH)
        .h(BUTTON_HEIGHT)
        .set(ids.button_explanation_1, &mut ui)
    {
        queue.push(event_1)
    }

    for _ in widget::Button::new()
        .label(&text_2)
        .label_font_size(FONT_SIZE_SMALL)
        .up_from(ids.button_explanation_1, 3.0f64)
        .w(BUTTON_WIDTH)
        .h(BUTTON_HEIGHT)
        .set(ids.button_explanation_2, &mut ui)
    {
        queue.push(event_2)
    }

    // Button for switching render mode
    let text = format!(
        "Switch to {}",
        scene.get_current_render_mode().inverse().to_str()
    );
    for _ in widget::Button::new()
        .label(&text)
        .label_font_size(FONT_SIZE_SMALL)
        .up_from(ids.button_explanation_2, 3.0f64)
        .w(BUTTON_WIDTH)
        .h(BUTTON_HEIGHT)
        .set(ids.buttom_render_mode, &mut ui)
    {
        queue.push(UIEvents::RenderModeSwitch)
    }

    // Button for reseting the current view
    for _ in widget::Button::new()
        .label("Reset view")
        .label_font_size(FONT_SIZE_SMALL)
        .up_from(ids.buttom_render_mode, 3.0f64)
        .w(BUTTON_WIDTH)
        .h(BUTTON_HEIGHT)
        .set(ids.button_reset, &mut ui)
    {
        queue.push(UIEvents::ResetButtonPress)
    }

    // Button for switching 2D/3D if available
    if scene.dimension_switch_available() {
        let text = format!("Switch to {}", scene.dimensionality_mode.inverse().to_str());
        for _ in widget::Button::new()
            .label(&text)
            .label_font_size(FONT_SIZE_SMALL)
            .up_from(ids.button_reset, 3.0f64)
            .w(BUTTON_WIDTH)
            .h(BUTTON_HEIGHT)
            .set(ids.button_dimension_switch, &mut ui)
        {
            queue.push(UIEvents::DimensionalitySwitch)
        }
    }

    // Settings for the gamma
    // The gamma slider
    let mut text_slider_value = scene.get_gamma().to_string();
    text_slider_value.truncate(5);

    if let Some(gamma) = widget::Slider::new(scene.get_gamma(), GAMMA_MIN_MAX.0, GAMMA_MIN_MAX.1)
        .label(&text_slider_value)
        .label_font_size(FONT_SIZE - 1)
        .label_color(Color::Rgba(1.0, 0.0, 0.0, 1.0))
        .bottom_right_with_margin(5.0f64)
        .h(SLIDER_HEIGHT)
        .set(ids.slider_gamma, &mut ui)
    {
        queue.push(UIEvents::SetGamma(gamma))
    }

    // Gamma reset button
    for _ in widget::Button::new()
        .label("Reset gamma")
        .label_font_size(FONT_SIZE_SMALL)
        .up_from(ids.slider_gamma, 2.0f64)
        .w_of(ids.slider_gamma)
        .h(BUTTON_HEIGHT - 2f64)
        .set(ids.button_gamma_reset, &mut ui)
    {
        queue.push(UIEvents::SetGamma(scene.get_default_gamma()))
    }

    // Gamma helper text
    widget::Text::new("Set the gamma:")
        .font_size(FONT_SIZE - 2)
        .up_from(ids.button_gamma_reset, 3.0f64)
        .w_of(ids.button_gamma_reset)
        .set(ids.text_gamma_slider, &mut ui);

    // Settings for the point size
    match scene.get_current_render_mode() {
        RenderMode::Discreet => {
            // Point size slider

            let mut text_slider_value = scene.get_point_size().to_string();
            text_slider_value.truncate(5);

            if let Some(point_size) = widget::Slider::new(
                scene.get_point_size(),
                scene.get_default_point_size() / 4f32,
                scene.get_default_point_size() * 4f32,
            )
            .label(&text_slider_value)
            .label_font_size(FONT_SIZE - 1)
            .label_color(Color::Rgba(1.0, 0.0, 0.0, 1.0))
            .h_of(ids.slider_gamma)
            .up_from(ids.text_gamma_slider, 7.0f64)
            .set(ids.slider_point_size, &mut ui)
            {
                queue.push(UIEvents::SetPointSize(point_size))
            }

            // Point size reset button
            for _ in widget::Button::new()
                .label("Reset point size")
                .label_font_size(FONT_SIZE_SMALL)
                .up_from(ids.slider_point_size, 2.0f64)
                .w_of(ids.slider_point_size)
                .h(BUTTON_HEIGHT - 2f64)
                .set(ids.button_size_reset, &mut ui)
            {
                queue.push(UIEvents::SetPointSize(scene.get_default_point_size()))
            }

            // Point size helper text
            widget::Text::new("Set the point size:")
                .font_size(FONT_SIZE - 2)
                .up_from(ids.button_size_reset, 3.0f64)
                .w_of(ids.button_size_reset)
                .set(ids.text_size_slider, &mut ui);
        }
        RenderMode::Continuous => {
            // Create slider to set the blob size
            let mut text_slider_value = scene.get_blob_size().to_string();
            text_slider_value.truncate(5);

            if let Some(blob_size) = widget::Slider::new(
                scene.get_blob_size(),
                scene.get_default_blob_size() / 4f32,
                scene.get_default_blob_size() * 4f32,
            )
            .label(&text_slider_value)
            .label_font_size(FONT_SIZE - 1)
            .label_color(Color::Rgba(1.0, 0.0, 0.0, 1.0))
            .h_of(ids.slider_gamma)
            .up_from(ids.text_gamma_slider, 7.0f64)
            .set(ids.slider_blob_size, &mut ui)
            {
                queue.push(UIEvents::SetBlobSize(blob_size))
            }

            // Blob reset button
            for _ in widget::Button::new()
                .label("Reset size")
                .label_font_size(FONT_SIZE_SMALL)
                .up_from(ids.slider_blob_size, 2.0f64)
                .w_of(ids.slider_point_size)
                .h(BUTTON_HEIGHT - 2f64)
                .set(ids.button_size_reset, &mut ui)
            {
                queue.push(UIEvents::SetBlobSize(scene.get_default_blob_size()))
            }

            // Blob helper text
            widget::Text::new("Set the blob size:")
                .font_size(FONT_SIZE - 2)
                .up_from(ids.button_size_reset, 3.0f64)
                .w_of(ids.button_size_reset)
                .set(ids.text_size_slider, &mut ui);
        }
    };

    // Recomputing the current metric
    // Only show this menu if the explanation mode is not none
    if scene.get_explanation_mode() != ExplanationMode::None {
        // Create the slider and metric switch button
        match scene.ui_state.neighborhood_type {
            NeighborhoodType::R => {
                if let Some(radius_value) = widget::Slider::new(
                    scene.ui_state.r,
                    NEIGHBORHOOD_R_MIN_MAX.0,
                    NEIGHBORHOOD_R_MIN_MAX.1,
                )
                .label(&scene.ui_state.get_neighborhood_text())
                .label_font_size(FONT_SIZE - 1)
                .label_color(Color::Rgba(1.0, 0.0, 0.0, 1.0))
                .h_of(ids.slider_gamma)
                .up_from(ids.text_size_slider, 7.0f64)
                .set(ids.slider_neighborhood, &mut ui)
                {
                    queue.push(UIEvents::UpdateUINeighborhood(Neighborhood::R(
                        radius_value,
                    )))
                }
            }
            NeighborhoodType::K => {
                // Hack: usize sliders are not supported, need to make the slider one for floats and cast to usize every time.
                if let Some(neighborhood_size) = widget::Slider::new(
                    scene.ui_state.k as f32,
                    NEIGHBORHOOD_K_MIN_MAX.0 as f32,
                    NEIGHBORHOOD_K_MIN_MAX.1 as f32,
                )
                .label(&scene.ui_state.get_neighborhood_text())
                .label_font_size(FONT_SIZE - 1)
                .label_color(Color::Rgba(1.0, 0.0, 0.0, 1.0))
                .h_of(ids.slider_gamma)
                .up_from(ids.text_size_slider, 7.0f64)
                .set(ids.slider_neighborhood, &mut ui)
                {
                    queue.push(UIEvents::UpdateUINeighborhood(Neighborhood::K(
                        neighborhood_size as usize,
                    )))
                }
            }
        }

        // Add switch between K and R
        for _ in widget::Button::new()
            .label(&format!(
                "Switch to {}",
                scene.ui_state.neighborhood_type.inverse().to_string()
            ))
            .label_font_size(FONT_SIZE_SMALL)
            .up_from(ids.slider_neighborhood, 2.0f64)
            .w(BUTTON_WIDTH / 2.0)
            .h(BUTTON_HEIGHT - 2f64)
            .set(ids.button_switch_neighborhood_type, &mut ui)
        {
            queue.push(UIEvents::UpdateUISwitchNeighborhood)
        }

        for _ in widget::Button::new()
            .label("Compute")
            .label_font_size(FONT_SIZE_SMALL)
            .right_from(ids.button_switch_neighborhood_type, 2.0f64)
            .w(BUTTON_WIDTH / 2.0)
            .h(BUTTON_HEIGHT - 2f64)
            .set(ids.button_recompute, &mut ui)
        {
            queue.push(UIEvents::RunExplanationMode(
                scene.get_explanation_mode(),
                scene.ui_state.into(),
            ))
        }

        // Recompute text
        widget::Text::new("Recompute current metric:")
            .font_size(FONT_SIZE_SMALL)
            .up_from(ids.button_switch_neighborhood_type, 3.0f64)
            .w_of(ids.button_size_reset)
            .set(ids.text_recompute, &mut ui);
    }

    // Handle all the enqueued events in order.
    for event in queue {
        match event {
            UIEvents::ResetButtonPress => scene.reset_camera(),
            UIEvents::RenderModeSwitch => scene.switch_render_mode(),
            UIEvents::DimensionalitySwitch => scene.switch_dimensionality(),
            // Setting scene options
            UIEvents::SetPointSize(size) => scene.set_point_size(size),
            UIEvents::SetBlobSize(size) => scene.set_blob_size(size),
            UIEvents::SetGamma(gamma) => scene.set_gamma(gamma),
            UIEvents::SetExplanationMode(mode) => scene.set_explanation_mode(mode),
            // Running the explanation method
            UIEvents::RunExplanationMode(mode, neighborhood) => {
                scene.ui_state.update(neighborhood);
                scene.run_explanation_mode(mode, neighborhood)
            }
            // UI specific
            UIEvents::UpdateUINeighborhood(neighborhood) => scene.ui_state.update(neighborhood),
            UIEvents::UpdateUISwitchNeighborhood => scene.ui_state.switch_neighborhood_type(),
        }
    }
}
