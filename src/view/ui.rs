extern crate kiss3d;
extern crate nalgebra as na;

use crate::view::view::Scene;
use kiss3d::{
    conrod::{widget, widget_ids, Color, Colorable, Positionable, Sizeable, Widget},
    window::CustomWindow,
};

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct WidgetId {
        canvas,
        text_left,
        text_right,
        pane_left,
        pane_right
    }
}

const PANE_ALPHA: f32 = 0.25;
const PANE_WIDTH: kiss3d::conrod::Scalar = 150.0;

/// Draw an overlay in the window of the given scene
pub fn draw_overlay(scene: &mut Scene, window: &mut CustomWindow) {
    // let num_points_text = format!("Number of points: {}", scene.original_points.len());

    let ids = &scene.conrod_ids;
    let mut ui = window.conrod_ui_mut().set_widgets();

    // Create left pane on the main canvas
    widget::Canvas::new()
        // .align_left_of(ids.canvas)
        .mid_left()
        .color(Color::Rgba(0.0, 0.0, 0.0, PANE_ALPHA))
        .w(PANE_WIDTH)
        .set(ids.pane_left, &mut ui);

    // Create right pane on the main canvas
    widget::Canvas::new()
        // .align_right_of(ids.canvas)
        .mid_right()
        .color(Color::Rgba(0.0, 0.0, 0.0, PANE_ALPHA))
        .w_of(ids.pane_left)
        .set(ids.pane_right, &mut ui);

    widget::Text::new("Left test")
        .middle_of(ids.pane_left)
        .color(Color::Rgba(1.0, 0.0, 0.0, 1.0))
        .set(ids.text_left, &mut ui);

    widget::Text::new("Right test")
        .middle_of(ids.pane_right)
        .color(Color::Rgba(1.0, 0.0, 0.0, 1.0))
        .set(ids.text_right, &mut ui);
}
