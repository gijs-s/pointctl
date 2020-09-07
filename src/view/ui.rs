extern crate kiss3d;
extern crate nalgebra as na;

use super::view::RenderMode;
use crate::view::color_map::ColorMap;
use crate::view::view::Scene;
use kiss3d::{
    conrod::{widget, widget_ids, Color, Colorable, Labelable, Positionable, Sizeable, Widget},
    window::CustomWindow,
};

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct WidgetId {
        button_reset,
        text_point_count,
        text_dimensionality,
        text_render_mode,
        // Widgets used for the colour legend
        canvas_colour_legend,
        text_dim_0,
        colour_block_0,
        text_dim_1,
        colour_block_1,
        text_dim_2,
        colour_block_2,
        text_dim_3,
        colour_block_3,
        text_dim_4,
        colour_block_4,
        text_dim_5,
        colour_block_5,
        text_dim_6,
        colour_block_6,
        text_dim_7,
        colour_block_7,
        text_dim_other,
        colour_block_other,
    }
}

const FONT_SIZE: u32 = 12;
const COLOR_PREVIEW_SIZE: f64 = 18.0f64;

/// Draw an overlay in the window of the given scene
pub fn draw_overlay(mut scene: &mut Scene, window: &mut CustomWindow) {
    // General info in the top left
    draw_basic_info(&mut scene, window);

    // Draw buttons and sliders in the bottom left
    draw_buttons(&mut scene, window);

    // Draw the color legend in the top right
    draw_colour_legend(&mut scene, window);
}

/// Draw the basic information about the current view in the left top.
fn draw_basic_info(scene: &mut Scene, window: &mut CustomWindow) {
    // Get a mutable reference to the conrod ui
    let ids = &scene.conrod_ids;
    let mut ui = window.conrod_ui_mut().set_widgets();

    // Display the amount of points
    let num_points_text = format!("Point count: {}", scene.original_points.len());
    widget::Text::new(&num_points_text)
        .font_size(FONT_SIZE)
        .top_left()
        .color(Color::Rgba(0.0, 0.0, 0.0, 1.0))
        .set(ids.text_point_count, &mut ui);

    // Display the current reduction dimensionality
    let dimensionality_text = format!("Reduced to: {}", scene.render_mode.to_str());
    widget::Text::new(&dimensionality_text)
        .font_size(FONT_SIZE)
        .down_from(ids.text_point_count, 5.0f64)
        .color(Color::Rgba(0.0, 0.0, 0.0, 1.0))
        .set(ids.text_dimensionality, &mut ui);

    // Display the current rendering mode
    let render_mode = match scene.render_mode {
        RenderMode::ThreeD => "Discreet".to_string(),
        RenderMode::TwoD => scene.state_2d.renderer.render_mode.to_str(),
    };
    let render_mode_text = format!("Render mode: {}", render_mode);
    widget::Text::new(&render_mode_text)
        .font_size(FONT_SIZE)
        .down_from(ids.text_dimensionality, 5.0f64)
        .color(Color::Rgba(0.0, 0.0, 0.0, 1.0))
        .set(ids.text_render_mode, &mut ui);
}

/// Draw all the buttoms and slides on the screen.
fn draw_buttons(scene: &mut Scene, window: &mut CustomWindow) {
    // Get a mutable reference to the conrod ui
    let ids = &scene.conrod_ids;
    let mut ui = window.conrod_ui_mut().set_widgets();

    // Button for reseting the current view
    for _press in widget::Button::new()
        .label("Reset view")
        .label_font_size(FONT_SIZE)
        .bottom_left()
        .w_of(ids.text_point_count)
        .set(ids.button_reset, &mut ui)
    {
        scene.reset_camera();
    }
}

/// Draw the colour legend in the top right
fn draw_colour_legend(scene: &mut Scene, window: &mut CustomWindow) {
    // Get a mutable reference to the conrod ui
    let ids = &scene.conrod_ids;
    let mut ui = window.conrod_ui_mut().set_widgets();

    let color_map = match scene.render_mode {
        RenderMode::TwoD => &scene.state_2d.color_map,
        RenderMode::ThreeD => &scene.state_3d.color_map,
    };

    // No need to render the color legend if the color map is empty
    if !color_map.is_initialized() {
        widget::Text::new("No color map present\nall points are grey")
            .font_size(FONT_SIZE)
            .top_right()
            .set(ids.text_dim_0, &mut ui);
        return;
    }

    let color = ColorMap::to_conrod_color(&color_map.rank_to_color(&0usize));
    widget::Canvas::new()
        .top_right_with_margin(5.0f64)
        .w(COLOR_PREVIEW_SIZE)
        .h(COLOR_PREVIEW_SIZE)
        .color(color)
        .set(ids.colour_block_0, &mut ui);

    let text = format!(
        "Dimension {}",
        color_map.get_dimension_from_rank(&0usize).unwrap()
    );
    widget::Text::new(&text)
        .font_size(FONT_SIZE)
        .left_from(ids.colour_block_0, 2.0f64)
        .w_of(ids.text_point_count)
        .set(ids.text_dim_0, &mut ui);

    // All the ids used from drawing
    // Here the first entry is the one we offset the current ui element from
    // the second and third are the actual ui element ids.
    let dimensions = vec![
        (ids.colour_block_0, ids.colour_block_1, ids.text_dim_1),
        (ids.colour_block_1, ids.colour_block_2, ids.text_dim_2),
        (ids.colour_block_2, ids.colour_block_3, ids.text_dim_3),
        (ids.colour_block_3, ids.colour_block_4, ids.text_dim_4),
        (ids.colour_block_4, ids.colour_block_5, ids.text_dim_5),
        (ids.colour_block_5, ids.colour_block_6, ids.text_dim_6),
        (ids.colour_block_6, ids.colour_block_7, ids.text_dim_7),
        (
            ids.colour_block_7,
            ids.colour_block_other,
            ids.text_dim_other,
        ),
    ];

    for (index, &(offset_id, preview_id, text_id)) in dimensions
        .iter()
        .take(color_map.dimension_count())
        .enumerate()
    {
        let rank = &index + 1usize;
        // First draw the color preview with the correct color.
        let color = ColorMap::to_conrod_color(&color_map.rank_to_color(&rank));
        widget::Canvas::new()
            .down_from(offset_id, 3.0)
            .w(COLOR_PREVIEW_SIZE)
            .h(COLOR_PREVIEW_SIZE)
            .color(color)
            .set(preview_id, &mut ui);

        let text = format!(
            "Dimension {}",
            color_map.get_dimension_from_rank(&rank).unwrap()
        );

        widget::Text::new(&text)
            .font_size(FONT_SIZE)
            .left_from(preview_id, 2.0f64)
            .w_of(ids.text_point_count)
            .set(text_id, &mut ui);
    }
}
