extern crate kiss3d;
extern crate nalgebra as na;

use super::view::RenderMode;
use crate::view::ui::widget::UiCell;
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

/// Draw an overlay in the window of the given scene
pub fn draw_overlay(scene: &mut Scene, window: &mut CustomWindow) {
    // let num_points_text = format!("Number of points: {}", scene.original_points.len());

    let ids = &scene.conrod_ids;
    let mut ui = window.conrod_ui_mut().set_widgets();

    // General info in the top left
    draw_basic_info(&mut ui, &ids, &mut scene, &mut window);

    // Draw buttons and sliders in the bottom left
    draw_buttons(&mut ui, &ids, &mut scene, &mut window);

    // Draw the color legend in the top right
    draw_colour_legend(&mut ui, &ids, &mut scene, &mut window);
}

/// Draw the basic information about the current view in the left top.
fn draw_basic_info(ui: &mut UiCell, ids: &WidgetId, scene: &mut Scene, window: &mut CustomWindow) {
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
fn draw_buttons(ui: &mut UiCell, ids: &WidgetId, scene: &mut Scene, window: &mut CustomWindow) {
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
fn draw_colour_legend(
    ui: &mut UiCell,
    ids: &WidgetId,
    scene: &mut Scene,
    window: &mut CustomWindow,
) {
    for rank in 1usize..7usize {
        // Draw the colour legend top down
        // widget::Text::new("Other dimesnions: ")
        //     .font_size(FONT_SIZE)
        //     .bottom_right()
        //     .w_of(ids.text_point_count)
        //     .set(ids.text_dim_other);
    }
}
