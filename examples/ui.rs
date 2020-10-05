extern crate kiss3d;
extern crate nalgebra as na;

// Third party
use kiss3d::light::Light;
use kiss3d::window::Window;

// Conrod
use kiss3d::conrod::{widget, widget_ids, Color, Colorable, Widget};

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        text,
        canvas
    }
}

// Main display function
pub fn main() {
    const WINDOW_WIDTH: u32 = 1024;
    const WINDOW_HEIGHT: u32 = 768;
    let mut window = Window::new_with_size("Pointctl visualizer", WINDOW_WIDTH, WINDOW_HEIGHT);
    window.set_background_color(1.0, 1.0, 1.0);
    window.set_light(Light::StickToCamera);
    window.set_point_size(4.);

    let ids = Ids::new(window.conrod_ui_mut().widget_id_generator());

    // Start the render loop, this will _not_ work with 2D scenes yet.
    while window.render() {
        let mut ui = window.conrod_ui_mut().set_widgets();
        // widget::Canvas::new()
        //     // .align_bottom()
        //     .h(100.0)
        //     .set(ids.canvas, &mut ui);

        widget::Text::new("test")
            // .mid_top_of(ids.canvas)
            .color(Color::Rgba(1.0, 0.0, 0.0, 1.0))
            .set(ids.text, &mut ui);
    }
}
