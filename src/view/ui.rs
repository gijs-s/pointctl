extern crate kiss3d;
extern crate nalgebra as na;

use crate::view::view::Scene;
use kiss3d::{
    conrod::{widget, widget_ids, Color, Colorable, Positionable, Sizeable, Widget},
    window::CustomWindow,
};
use super::view::RenderMode;

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct WidgetId {
        text_point_count,
        text_dimensionality,
        text_render_mode,
        pane_left,
        pane_right
    }
}

const PANE_ALPHA: f32 = 0.10;
const FONT_SIZE: u32 = 12;
const PANE_WIDTH: kiss3d::conrod::Scalar = 20.0;

/// Draw an overlay in the window of the given scene
pub fn draw_overlay(scene: &mut Scene, window: &mut CustomWindow) {
    // let num_points_text = format!("Number of points: {}", scene.original_points.len());

    let ids = &scene.conrod_ids;
    let mut ui = window.conrod_ui_mut().set_widgets();

    // Create left pane on the main canvas
    widget::Canvas::new()
        .mid_left()
        .color(Color::Rgba(0.0, 0.0, 0.0, PANE_ALPHA))
        .w(PANE_WIDTH)
        .set(ids.pane_left, &mut ui);

    // Create right pane on the main canvas
    widget::Canvas::new()
        .mid_right()
        .color(Color::Rgba(0.0, 0.0, 0.0, PANE_ALPHA))
        .w_of(ids.pane_left)
        .set(ids.pane_right, &mut ui);

    // Display the amount of points
    let num_points_text = format!("Point count: {}", scene.original_points.len());
    widget::Text::new(&num_points_text)
        .font_size(FONT_SIZE)
        .top_left_of(ids.pane_left)
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
