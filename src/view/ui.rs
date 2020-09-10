extern crate kiss3d;
extern crate nalgebra as na;

// Third party
use kiss3d::{
    conrod::{widget, widget_ids, Color, Colorable, Labelable, Positionable, Sizeable, Widget},
    window::CustomWindow,
};

// Internal imports
use crate::view::{color_map::ColorMap, view::Scene, DimensionalityMode};

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct WidgetId {
        // Information widgets
        text_point_count,
        text_dimensionality,
        text_render_mode,
        // Buttons
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
        // Settings panel for the current viewer.
    }
}

const FONT_SIZE: u32 = 12;
const BUTTON_WIDTH: f64 = 144.0f64;
const BUTTON_HEIGHT: f64 = 22.0f64;
const COLOR_PREVIEW_SIZE: f64 = 18.0f64;
/// All the types of event that can happen in the UI.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum UIEvents {
    ResetButtonPress,
    DimensionalitySwitch,
    RenderModeSwitch,
}

/// Draw an overlay in the window of the given scene
pub fn draw_overlay(scene: &mut Scene, window: &mut CustomWindow) {
    // Get a mutable reference to the conrod ui
    let ids = &scene.conrod_ids;
    let mut ui = window.conrod_ui_mut().set_widgets();

    // ######################################################################
    // # Draw the basic information about the current view in the left top. #
    // ######################################################################

    // Display the amount of points
    let num_points_text = format!("Point count: {}", scene.original_points.len());
    widget::Text::new(&num_points_text)
        .font_size(FONT_SIZE)
        .top_left()
        .color(Color::Rgba(0.0, 0.0, 0.0, 1.0))
        .set(ids.text_point_count, &mut ui);

    // Display the current reduction dimensionality
    let dimensionality_text = format!("Reduced to: {}", scene.dimensionality_mode.to_str());
    widget::Text::new(&dimensionality_text)
        .font_size(FONT_SIZE)
        .down_from(ids.text_point_count, 5.0f64)
        .color(Color::Rgba(0.0, 0.0, 0.0, 1.0))
        .set(ids.text_dimensionality, &mut ui);

    // Display the current rendering mode
    let render_mode = scene.get_current_render_mode().to_str();
    let render_mode_text = format!("Render mode: {}", render_mode);
    widget::Text::new(&render_mode_text)
        .font_size(FONT_SIZE)
        .down_from(ids.text_dimensionality, 5.0f64)
        .color(Color::Rgba(0.0, 0.0, 0.0, 1.0))
        .set(ids.text_render_mode, &mut ui);

    // ###########################################
    // # Draw the color legend in the top right #
    // ###########################################

    // Retrieve the color map from the correct state
    let color_map = scene.get_current_color_map();
    // No need to render the color legend if the color map is empty
    if !color_map.is_initialized() {
        widget::Text::new("No color map present\nall points are grey")
            .font_size(FONT_SIZE)
            .top_right()
            .set(ids.text_dim_0, &mut ui);
        return;
    }

    let color = color_map.get_conrod_color(&0usize);
    widget::Canvas::new()
        .top_right_with_margin(5.0f64)
        .w(COLOR_PREVIEW_SIZE)
        .h(COLOR_PREVIEW_SIZE)
        .color(color)
        .set(ids.color_block_0, &mut ui);

    let text = format!(
        "Dimension {}",
        color_map.get_dimension_from_rank(&0usize).unwrap()
    );
    widget::Text::new(&text)
        .font_size(FONT_SIZE)
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
        .take(color_map.dimension_count())
        .enumerate()
    {
        let rank = &index + 1usize;
        // First draw the color preview with the correct color.
        let color = color_map.get_conrod_color(&rank);
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
                format!(
                    "Dimension {}",
                    color_map.get_dimension_from_rank(&rank).unwrap()
                )
            }
        };

        widget::Text::new(&text)
            .font_size(FONT_SIZE)
            .left_from(preview_id, 2.0f64)
            .w_of(ids.text_point_count)
            .set(text_id, &mut ui);
    }

    // ##################################################
    // # Draw all the buttoms and slides on the screen. #
    // ##################################################

    // Create an event queue, the scene cannot be used immutably for the ids and then
    // mutated after. We need to render all the widgets and then we can process and
    // mutate the scene.
    let mut queue: Vec<UIEvents> = Vec::new();

    // Button for switching render mode
    let text = format!(
        "Switch to {}",
        scene.get_current_render_mode().inverse().to_str()
    );
    for _ in widget::Button::new()
        .label(&text)
        .label_font_size(FONT_SIZE)
        .bottom_left_with_margin(5.0f64)
        .w(BUTTON_WIDTH)
        .h(BUTTON_HEIGHT)
        .set(ids.buttom_render_mode, &mut ui)
    {
        queue.push(UIEvents::RenderModeSwitch)
    }

    // Button for reseting the current view
    for _ in widget::Button::new()
        .label("Reset view")
        .label_font_size(FONT_SIZE)
        .up_from(ids.buttom_render_mode, 5.0f64)
        .w(BUTTON_WIDTH)
        .h(BUTTON_HEIGHT)
        .set(ids.button_reset, &mut ui)
    {
        queue.push(UIEvents::ResetButtonPress)
    }

    // Button for switching 2D/3D
    let text = format!("Switch to {}", scene.dimensionality_mode.inverse().to_str());
    for _ in widget::Button::new()
        .label(&text)
        .label_font_size(FONT_SIZE)
        .up_from(ids.button_reset, 5.0f64)
        .w(BUTTON_WIDTH)
        .h(BUTTON_HEIGHT)
        .set(ids.button_dimension_switch, &mut ui)
    {
        queue.push(UIEvents::DimensionalitySwitch)
    }

    // Handle all the enqueued events in order.
    for event in queue {
        match event {
            UIEvents::ResetButtonPress => scene.reset_camera(),
            UIEvents::RenderModeSwitch => scene.switch_render_mode(),
            UIEvents::DimensionalitySwitch => scene.switch_dimensionality(),
        }
    }
}
