// Third party
use kiss3d::{
    conrod::{widget, UiCell, Color, Colorable, Labelable, Positionable, Sizeable, Widget},
    window::CustomWindow,
};

use std::boxed::Box;

// Internal imports
use crate::{exp::Neighborhood, view::{ExplanationMode, RenderMode, Scene}};
use super::{ui_events::UIEvents, ui_state::{OpenSettingsMenu, NeighborhoodType}};

// Font sizes
const FONT_SIZE: u32 = 12;
const FONT_SIZE_SMALL: u32 = FONT_SIZE - 4;
// Button sizes
const BUTTON_WIDTH: f64 = 160f64;
const BUTTON_HEIGHT: f64 = 22f64;
const MENU_BUTTON_WIDTH: f64 = 160f64;
// Slider settings
const SLIDER_WIDTH: f64 = 150f64;
const SLIDER_HEIGHT: f64 = 18f64;
// Bounds on sliders
const GAMMA_MIN_MAX: (f32, f32) = (1.0, 3.4);
const NEIGHBORHOOD_R_MIN_MAX: (f32, f32) = (0.01, 1.1);
const NEIGHBORHOOD_K_MIN_MAX: (usize, usize) = (10, 250);
// Misc
const SIDE_MARGIN: f64 = 3.0f64;
const COLOR_PREVIEW_SIZE: f64 = 18f64;

pub fn draw_overlay(scene: &mut Scene, window: &mut CustomWindow) -> Vec<UIEvents> {
    // Get the ui cell used when placing widgets and box it
    let ui = Box::from(window.conrod_ui_mut().set_widgets());

    // Add the basic info on the top left
    let ui = draw_info_text(ui, &scene);
    // Add an early out when no points are loaded
    if !scene.initialized() { return Vec::new(); }

    // Render the current legends
    let ui = draw_legends(ui, &scene);

    // Render the menus and buttons, these can produce UI events each which
    // we will handle after rendering everything.
    let event_queue: Vec<UIEvents> = Vec::new();
    let (ui, event_queue) = draw_bottom_menu(ui, event_queue, &scene);

    // draw the left menu buttons
    let (ui, event_queue) =  draw_left_general_menu(ui, event_queue, &scene);

    // draw the correct right menu
    let (_ui, event_queue) = match scene.ui_state.open_menu {
        // No-op, menu closed so we do not draw anything
        OpenSettingsMenu::None => (ui, event_queue),
        OpenSettingsMenu::ViewerSettings => draw_right_viewer_settings_menu(ui, event_queue, &scene),
        OpenSettingsMenu::ExplanationSettings => draw_right_explanation_settings_menu(ui, event_queue, &scene),
    };

    // Return the event queue at the end, this will be handled in the scene
    event_queue
}

/// Draw the basic information about the current view in the left top.
pub fn draw_info_text<'a>(mut ui: Box<UiCell<'a>>, scene: &Scene) -> Box<UiCell<'a>> {
    let info_ids = &scene.ui_state.info_widgets;

    // Display the current explanation mode
    let explanation_mode_text = format!(
        "Explanation mode: {}",
        scene.get_explanation_mode().to_str()
    );
    widget::Text::new(&explanation_mode_text)
        .font_size(FONT_SIZE)
        .top_left()
        .color(Color::Rgba(0.0, 0.0, 0.0, 1.0))
        .set(info_ids.text_explanation_mode, &mut ui);

    // Display the current rendering mode
    let render_mode = scene.get_current_render_mode().to_str();
    let render_mode_text = format!("Render mode: {}", render_mode);
    widget::Text::new(&render_mode_text)
        .font_size(FONT_SIZE)
        .down_from(info_ids.text_explanation_mode, 5.0f64)
        .color(Color::Rgba(0.0, 0.0, 0.0, 1.0))
        .set(info_ids.text_render_mode, &mut ui);

    // Display the current reduction dimensionality
    let dimensionality_text = format!("Reduced to: {}", scene.dimensionality_mode.to_str());
    widget::Text::new(&dimensionality_text)
        .font_size(FONT_SIZE)
        .down_from(info_ids.text_render_mode, 5.0f64)
        .color(Color::Rgba(0.0, 0.0, 0.0, 1.0))
        .set(info_ids.text_dimensionality, &mut ui);

    // Display the amount of points
    // TODO get point count function in the scene
    let num_points_text = format!("Point count: {}", scene.get_point_count());
    widget::Text::new(&num_points_text)
        .font_size(FONT_SIZE)
        .down_from(info_ids.text_dimensionality, 5.0f64)
        .color(Color::Rgba(0.0, 0.0, 0.0, 1.0))
        .set(info_ids.text_point_count, &mut ui);

    // Draw error if no data is present
    if !scene.initialized() {
        widget::Text::new("No reduction data was loaded, can not display anything\nplease consult 'pointctl --help'")
            .font_size(FONT_SIZE * 3u32)
            .middle()
            .color(Color::Rgba(0.80, 0.33, 0.0, 1.0 ))
            .set(info_ids.text_error, &mut ui);
    }
    ui
}

/// Draw the color legend in the top right
pub fn draw_legends<'a>(mut ui: Box<UiCell<'a>>, scene: &Scene) -> Box<UiCell<'a>> {
    let legend_ids = &scene.ui_state.legend_widgets;
    // Retrieve the color map currently in use
    let color_map = scene.get_current_color_map();

    // No need to render the color legend if the color map is empty
    if !color_map.is_initialized() {
        widget::Text::new("No explanation loaded\nall points are grey")
            .font_size(FONT_SIZE)
            .top_right()
            .set(legend_ids.text_dim_0, &mut ui);
    } else {
        let color = color_map.get_conrod_color_with_gamma(&0usize, scene.get_gamma());
        widget::Canvas::new()
            .top_right_with_margin(5.0f64)
            .w(COLOR_PREVIEW_SIZE)
            .h(COLOR_PREVIEW_SIZE)
            .color(color)
            .set(legend_ids.color_block_0, &mut ui);

        let dim = color_map.get_dimension_from_rank(&0usize).unwrap();
        let text = format!("[{}] {}", dim, scene.get_dimension_name(dim).unwrap());
        widget::Text::new(&text)
            .font_size(FONT_SIZE_SMALL)
            .left_from(legend_ids.color_block_0, 2.0f64)
            .w(BUTTON_WIDTH)
            .set(legend_ids.text_dim_0, &mut ui);

        // All the legend_ids used from drawing
        // Here the first entry is the one we offset the current ui element from
        // the second and third are the actual ui element legend_ids.
        let dimensions = vec![
            (legend_ids.color_block_0, legend_ids.color_block_1, legend_ids.text_dim_1),
            (legend_ids.color_block_1, legend_ids.color_block_2, legend_ids.text_dim_2),
            (legend_ids.color_block_2, legend_ids.color_block_3, legend_ids.text_dim_3),
            (legend_ids.color_block_3, legend_ids.color_block_4, legend_ids.text_dim_4),
            (legend_ids.color_block_4, legend_ids.color_block_5, legend_ids.text_dim_5),
            (legend_ids.color_block_5, legend_ids.color_block_6, legend_ids.text_dim_6),
            (legend_ids.color_block_6, legend_ids.color_block_7, legend_ids.text_dim_7),
            (legend_ids.color_block_7, legend_ids.color_block_other, legend_ids.text_dim_other),
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
                .w(BUTTON_WIDTH)
                .set(text_id, &mut ui);
        }
    };
    ui
}

/// Draw the menu buttons at the bottom of the display
fn draw_bottom_menu<'a>(mut ui: Box<UiCell<'a>>, mut event_queue: Vec<UIEvents>, scene: &Scene) -> (Box<UiCell<'a>>, Vec<UIEvents>) {
    let menu_ids = &scene.ui_state.menu_widgets;

    // Draw the explanation settings menu expand button
    if let (_, Some(status)) = widget::CollapsibleArea::new(scene.ui_state.open_menu == OpenSettingsMenu::ExplanationSettings, "Explanation settings")
        .label_font_size(FONT_SIZE)
        .bottom_right_with_margin(SIDE_MARGIN)
        .w(MENU_BUTTON_WIDTH)
        .h(BUTTON_HEIGHT)
        .color(Color::Rgba(1.0, 1.0, 1.0, 1.00))
        .set(menu_ids.explanation_settings_menu_toggle, &mut ui) {
            event_queue.push(UIEvents::SwitchOpenMenu(match status.is_open() {
                true => OpenSettingsMenu::ExplanationSettings,
                false => OpenSettingsMenu::None,
            }));
        }

    // Draw the computation switch menu expand button
    if let (_, Some(status)) = widget::CollapsibleArea::new(scene.ui_state.open_menu == OpenSettingsMenu::ViewerSettings, "Viewer settings")
        .label_font_size(FONT_SIZE)
        .left_from(menu_ids.explanation_settings_menu_toggle, 1.0f64)
        .w(MENU_BUTTON_WIDTH)
        .h(BUTTON_HEIGHT)
        .color(Color::Rgba(1.0, 1.0, 1.0, 1.00))
        .set(menu_ids.viewer_settings_menu_toggle, &mut ui) {
            event_queue.push(UIEvents::SwitchOpenMenu(match status.is_open() {
                true => OpenSettingsMenu::ViewerSettings,
                false => OpenSettingsMenu::None,
            }));
        }


    (ui, event_queue)
}

/// Draw the general menu above the left menu bar
fn draw_left_general_menu<'a>(mut ui: Box<UiCell<'a>>, mut event_queue: Vec<UIEvents>, scene: &Scene) -> (Box<UiCell<'a>>, Vec<UIEvents>) {
    let menu_ids = &scene.ui_state.menu_widgets;

    // Button for switching 2D/3D if available
    if scene.dimension_switch_available() {
        let text = format!("Switch to {}", scene.dimensionality_mode.inverse().to_str());
        for _ in widget::Button::new()
            .label(&text)
            .label_font_size(FONT_SIZE - 2)
            .bottom_left_with_margin(SIDE_MARGIN)
            .w(BUTTON_WIDTH)
            .h(BUTTON_HEIGHT - 2f64)
            .set(menu_ids.button_dimension_switch, &mut ui)
        {
            event_queue.push(UIEvents::DimensionalitySwitch)
        }
    }

    // Button for reseting the current view
    for _ in widget::Button::new()
        .label("Reset view")
        .label_font_size(FONT_SIZE - 2)
        .up_from(menu_ids.button_dimension_switch, 3.0f64)
        .w(BUTTON_WIDTH)
        .h(BUTTON_HEIGHT - 2f64)
        .set(menu_ids.button_reset, &mut ui)
    {
        event_queue.push(UIEvents::ResetButtonPress)
    }

    (ui, event_queue)
}

/// Draw the settings menu for the current explanation above the right menu bar
fn draw_right_explanation_settings_menu<'a>(mut ui: Box<UiCell<'a>>, mut event_queue: Vec<UIEvents>, scene: &Scene) -> (Box<UiCell<'a>>, Vec<UIEvents>) {
    let menu_ids = &scene.ui_state.menu_widgets;

    // Get the text and correct event if for turning off the explanation
    let (text_none, event_none) = (
        "Turn off annotations".to_string(),
        UIEvents::SetExplanationMode(ExplanationMode::None),
    );

    // Get the text and correct event for running the da silva explanation
    let (text_da_silva, event_da_silva) =
        match scene.is_explanation_available(&ExplanationMode::DaSilva) {
            true => (
                "View to Da Silva".to_string(),
                UIEvents::SetExplanationMode(ExplanationMode::DaSilva),
            ),
            false => (
                "Calculate Da Silva".to_string(),
                UIEvents::RunExplanationMode(ExplanationMode::DaSilva, Neighborhood::from(&scene.ui_state.recompute_state)),
            ),
        };

    // Get the text and correct event for running the da van driel explanation
    let (text_van_driel, event_van_driel) =
        match scene.is_explanation_available(&ExplanationMode::VanDriel) {
            true => (
                "View Van Driel".to_string(),
                UIEvents::SetExplanationMode(ExplanationMode::VanDriel),
            ),
            false => (
                "Calculate Van Driel".to_string(),
                UIEvents::RunExplanationMode(ExplanationMode::VanDriel, Neighborhood::from(&scene.ui_state.recompute_state)),
            ),
        };

    // Only show the 2 other options, switching to the mode you are already
    // in does not make sense.
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
        .label_font_size(FONT_SIZE - 2)
        .up_from(menu_ids.explanation_settings_menu_toggle, 5.0f64)
        .w(BUTTON_WIDTH)
        .h(BUTTON_HEIGHT - 2f64)
        .set(menu_ids.button_explanation_1, &mut ui)
    {
        event_queue.push(event_1)
    }

    for _ in widget::Button::new()
        .label(&text_2)
        .label_font_size(FONT_SIZE - 2)
        .up_from(menu_ids.button_explanation_1, 3.0f64)
        .w(BUTTON_WIDTH)
        .h(BUTTON_HEIGHT - 2f64)
        .set(menu_ids.button_explanation_2, &mut ui)
    {
        event_queue.push(event_2)
    }

    // Create the slider and metric switch button
    match scene.ui_state.recompute_state.neighborhood_type {
        NeighborhoodType::R => {
            if let Some(radius_value) = widget::Slider::new(
                scene.ui_state.recompute_state.r,
                NEIGHBORHOOD_R_MIN_MAX.0,
                NEIGHBORHOOD_R_MIN_MAX.1,
            )
            .label(&scene.ui_state.recompute_state.get_neighborhood_text())
            .label_font_size(FONT_SIZE - 1)
            .label_color(Color::Rgba(1.0, 0.0, 0.0, 1.0))
            .w(SLIDER_WIDTH)
            .h(SLIDER_HEIGHT)
            .up_from(menu_ids.button_explanation_2, 7.0f64)
            .set(menu_ids.slider_neighborhood, &mut ui)
            {
                event_queue.push(UIEvents::UpdateUINeighborhood(Neighborhood::R(
                    radius_value,
                )))
            }
        }
        NeighborhoodType::K => {
            // Hack: usize sliders are not supported, need to make the slider one for floats and cast to usize every time.
            if let Some(neighborhood_size) = widget::Slider::new(
                scene.ui_state.recompute_state.k as f32,
                NEIGHBORHOOD_K_MIN_MAX.0 as f32,
                NEIGHBORHOOD_K_MIN_MAX.1 as f32,
            )
            .label(&scene.ui_state.recompute_state.get_neighborhood_text())
            .label_font_size(FONT_SIZE - 1)
            .label_color(Color::Rgba(1.0, 0.0, 0.0, 1.0))
            .up_from(menu_ids.button_explanation_2, 7.0f64)
            .w(SLIDER_WIDTH)
            .h(SLIDER_HEIGHT)
            .set(menu_ids.slider_neighborhood, &mut ui)
            {
                event_queue.push(UIEvents::UpdateUINeighborhood(Neighborhood::K(
                    neighborhood_size as usize,
                )))
            }
        }
    }

    // Add switch between K and R
    for _ in widget::Button::new()
        .label(&format!(
            "Switch to {}",
            scene.ui_state.recompute_state.neighborhood_type.inverse().to_string()
        ))
        .label_font_size(FONT_SIZE_SMALL)
        .up_from(menu_ids.slider_neighborhood, 2.0f64)
        .w(if scene.get_explanation_mode() != ExplanationMode::None {BUTTON_WIDTH / 2.0} else { BUTTON_WIDTH })
        .h(BUTTON_HEIGHT - 2f64)
        .set(menu_ids.button_switch_neighborhood_type, &mut ui)
    {
        event_queue.push(UIEvents::UpdateUISwitchNeighborhood)
    }

    // Allow recomputing the current metric if a explanation mode is set
    if scene.get_explanation_mode() != ExplanationMode::None {
        for _ in widget::Button::new()
        .label("Compute")
        .label_font_size(FONT_SIZE_SMALL)
        .right_from(menu_ids.button_switch_neighborhood_type, 2.0f64)
        .w(BUTTON_WIDTH / 2.0)
        .h(BUTTON_HEIGHT - 2f64)
        .set(menu_ids.button_recompute, &mut ui)
        {
            event_queue.push(UIEvents::RunExplanationMode(
                scene.get_explanation_mode(),
                Neighborhood::from(&scene.ui_state.recompute_state),
            ))
        }
    }

    // Recompute text
    let t = if scene.get_explanation_mode() != ExplanationMode::None {
        "Recompute current metric:"
    } else {
        "Set neighborhood:"
    };
    widget::Text::new(t)
        .font_size(FONT_SIZE_SMALL)
        .up_from(menu_ids.button_switch_neighborhood_type, 3.0f64)
        .w(SLIDER_WIDTH)
        .set(menu_ids.text_recompute, &mut ui);

    (ui, event_queue)
}

/// Draw the settings menu for the current viewer above the right menu bar
fn draw_right_viewer_settings_menu<'a>(mut ui: Box<UiCell<'a>>, mut event_queue: Vec<UIEvents>, scene: &Scene) -> (Box<UiCell<'a>>, Vec<UIEvents>) {
    let menu_ids = &scene.ui_state.menu_widgets;

    // Button for switching render mode
    let text = format!(
        "Switch to {}",
        scene.get_current_render_mode().inverse().to_str()
    );
    for _ in widget::Button::new()
        .label(&text)
        .label_font_size(FONT_SIZE - 2)
        .up_from(menu_ids.explanation_settings_menu_toggle, 3.0f64)
        .w(BUTTON_WIDTH)
        .h(BUTTON_HEIGHT - 2f64)
        .set(menu_ids.buttom_render_mode, &mut ui)
    {
        event_queue.push(UIEvents::RenderModeSwitch)
    }

    // Settings for the gamma
    let mut text_slider_value = scene.get_gamma().to_string();
    text_slider_value.truncate(5);

    // Create slider for the gamma
    if let Some(gamma) = widget::Slider::new(scene.get_gamma(), GAMMA_MIN_MAX.0, GAMMA_MIN_MAX.1)
        .label(&text_slider_value)
        .label_font_size(FONT_SIZE - 1)
        .label_color(Color::Rgba(1.0, 0.0, 0.0, 1.0))
        .up_from(menu_ids.buttom_render_mode, 5.0f64)
        .w(SLIDER_WIDTH)
        .h(SLIDER_HEIGHT)
        .set(menu_ids.slider_gamma, &mut ui)
    {
        event_queue.push(UIEvents::SetGamma(gamma))
    }

    // Gamma reset button
    for _ in widget::Button::new()
        .label("Reset gamma")
        .label_font_size(FONT_SIZE - 2)
        .up_from(menu_ids.slider_gamma, 2.0f64)
        .w_of(menu_ids.slider_gamma)
        .h(BUTTON_HEIGHT - 2f64)
        .set(menu_ids.button_gamma_reset, &mut ui)
    {
        event_queue.push(UIEvents::SetGamma(scene.get_default_gamma()))
    }

    // Gamma helper text
    widget::Text::new("Set the gamma:")
        .font_size(FONT_SIZE - 2)
        .up_from(menu_ids.button_gamma_reset, 3.0f64)
        .w_of(menu_ids.button_gamma_reset)
        .set(menu_ids.text_gamma_slider, &mut ui);

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
            .h_of(menu_ids.slider_gamma)
            .up_from(menu_ids.text_gamma_slider, 7.0f64)
            .set(menu_ids.slider_point_size, &mut ui)
            {
                event_queue.push(UIEvents::SetPointSize(point_size))
            }

            // Point size reset button
            for _ in widget::Button::new()
                .label("Reset point size")
                .label_font_size(FONT_SIZE - 2)
                .up_from(menu_ids.slider_point_size, 2.0f64)
                .w_of(menu_ids.slider_point_size)
                .h(BUTTON_HEIGHT - 2f64)
                .set(menu_ids.button_size_reset, &mut ui)
            {
                event_queue.push(UIEvents::SetPointSize(scene.get_default_point_size()))
            }

            // Point size helper text
            widget::Text::new("Set the point size:")
                .font_size(FONT_SIZE - 2)
                .up_from(menu_ids.button_size_reset, 3.0f64)
                .w_of(menu_ids.button_size_reset)
                .set(menu_ids.text_size_slider, &mut ui);
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
            .h_of(menu_ids.slider_gamma)
            .up_from(menu_ids.text_gamma_slider, 7.0f64)
            .set(menu_ids.slider_blob_size, &mut ui)
            {
                event_queue.push(UIEvents::SetBlobSize(blob_size))
            }

            // Blob reset button
            for _ in widget::Button::new()
                .label("Reset size")
                .label_font_size(FONT_SIZE_SMALL)
                .up_from(menu_ids.slider_blob_size, 2.0f64)
                .w_of(menu_ids.slider_point_size)
                .h(BUTTON_HEIGHT - 2f64)
                .set(menu_ids.button_size_reset, &mut ui)
            {
                event_queue.push(UIEvents::SetBlobSize(scene.get_default_blob_size()))
            }

            // Blob helper text
            widget::Text::new("Set the blob size:")
                .font_size(FONT_SIZE - 2)
                .up_from(menu_ids.button_size_reset, 3.0f64)
                .w_of(menu_ids.button_size_reset)
                .set(menu_ids.text_size_slider, &mut ui);
        }
    };

    if scene.get_explanation_mode() != ExplanationMode::None {
        // Slider allowing you to modify the confidence bounds set to the current color map
        let color_map = scene.get_current_color_map();
        let (static_min, static_max) = color_map.get_static_confidence_bounds();
        let (current_min, current_max) = color_map.get_actual_confidence_bounds();
        // Create text for in the slider
        let mut min_text = current_min.to_string();
        let mut max_text = current_max.to_string();
        min_text.truncate(5);
        max_text.truncate(5);
        // create the range slider
        for (edge, value) in widget::RangeSlider::new(
            current_min,
            current_max,
            static_min - static_max * 0.05,
            static_max + static_max * 0.05,
        )
            .label(format!("{} - {}", min_text, max_text).as_str())
            .label_font_size(FONT_SIZE - 1)
            .label_color(Color::Rgba(1.0, 0.0, 0.0, 1.0))
            .up_from(menu_ids.text_size_slider, 7.0f64)
            .h(SLIDER_HEIGHT)
            .set(menu_ids.slider_color_normalization, &mut ui)
            {
                match edge {
                    widget::range_slider::Edge::Start => {
                        event_queue.push(UIEvents::SetColorBound(value, current_max))
                    }
                    widget::range_slider::Edge::End => {
                        event_queue.push(UIEvents::SetColorBound(current_min, value))
                    }
                }
            }

        // Confidence bounds helper text
        widget::Text::new("Confidence bounds:")
            .font_size(FONT_SIZE_SMALL)
            .up_from(menu_ids.slider_color_normalization, 3.0f64)
            .w_of(menu_ids.button_size_reset)
            .set(menu_ids.text_color_normalization_bounds, &mut ui);
    }

    (ui, event_queue)
}