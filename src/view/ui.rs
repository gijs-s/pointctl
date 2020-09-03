extern crate kiss3d;
extern crate nalgebra as na;

use crate::view::view::Scene;
use kiss3d::{
    conrod::{widget, widget_ids, Color, Colorable, Positionable, Labelable, Sizeable, Widget},
    window::CustomWindow,
};
use super::view::RenderMode;

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct WidgetId {
        button_reset,
        text_point_count,
        text_dimensionality,
        text_render_mode,
    }
}

const FONT_SIZE: u32 = 12;

/// Draw an overlay in the window of the given scene
pub fn draw_overlay(scene: &mut Scene, window: &mut CustomWindow) {
    // let num_points_text = format!("Number of points: {}", scene.original_points.len());

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
