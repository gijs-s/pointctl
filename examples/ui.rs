extern crate kiss3d;

// Third party
use kiss3d::window::Window;

// Conrod
use kiss3d::conrod::{widget, widget_ids, Color, Colorable, Positionable, Sizeable, Widget};

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        canvas,
        text_left,
        text_right,
        pane_left,
        pane_right
    }
}

const PANE_WIDTH: kiss3d::conrod::Scalar = 150.0;


// Main display function
pub fn main() {
    const WINDOW_WIDTH: u32 = 1024;
    const WINDOW_HEIGHT: u32 = 768;
    let mut window =
        Window::new_with_size("Pointctl UI testing program", WINDOW_WIDTH, WINDOW_HEIGHT);
    window.set_background_color(1.0, 1.0, 1.0);

    let ids = Ids::new(window.conrod_ui_mut().widget_id_generator());

    while window.render() {
        let mut ui = window.conrod_ui_mut().set_widgets();

        // Create the complete canvas
        widget::Canvas::new()
            .color(Color::Rgba(0.0, 0.0, 0.0, 0.0))
            .set(ids.canvas, &mut ui);

        // Create left pane on the main canvas
        widget::Canvas::new()
            .align_left_of(ids.canvas)
            .color(Color::Rgba(0.0, 0.0, 0.0, 0.5))
            .w(PANE_WIDTH)
            .set(ids.pane_left, &mut ui);

        // Create right pane on the main canvas
        widget::Canvas::new()
            .align_right_of(ids.canvas)
            .color(Color::Rgba(0.0, 0.0, 0.0, 0.5))
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
}
