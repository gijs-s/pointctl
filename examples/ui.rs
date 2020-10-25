extern crate nalgebra as na;
extern crate kiss3d;

// Third party
use kiss3d::light::Light;
use kiss3d::window::Window;

// Conrod
use kiss3d::conrod::{widget, widget_ids, Color, Colorable, Widget, Sizeable, Positionable};
use kiss3d::conrod::UiCell;

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        text,
        canvas_label,
        canvas,
        expand_button,
        expand_labe
    }
}

struct State {
    open: bool,
    ids: Ids,
}

// Main display function
pub fn main() {
    const WINDOW_WIDTH: u32 = 1024;
    const WINDOW_HEIGHT: u32 = 768;
    let mut window = Window::new_with_size("Pointctl visualizer", WINDOW_WIDTH, WINDOW_HEIGHT);
    window.set_background_color(1.0, 1.0, 1.0);
    window.set_light(Light::StickToCamera);
    window.set_point_size(4.);

    let mut state = State {
        open: false,
        ids: Ids::new(window.conrod_ui_mut().widget_id_generator())
    };
    // Start the render loop, this will _not_ work with 2D scenes yet.
    while window.render() {
        let ui = Box::from(window.conrod_ui_mut().set_widgets());
        let (_, new_state) = set_colllabsible_area(ui, state);
        state = new_state;
    }
}

fn set_colllabsible_area(mut ui: Box<UiCell>, mut state: State) -> (std::boxed::Box<kiss3d::conrod::UiCell>, State) {
    let (area, status) = widget::CollapsibleArea::new(state.open, "foobar")
        .label_font_size(10u32)
        .bottom_left_with_margin(1.0f64)
        .w(200f64)
        .h(25f64)
        .color(Color::Rgba(0.0, 0.0, 0.0, 0.10))
        .set(state.ids.canvas, &mut ui);

    match status {
        Some(s) => state.open = s.is_open(),
        None => (),
    };

    match area {
        Some(a) => {
            widget::Text::new("test")
                .up_from(a.collapsible_area_id, 5.0f64)
                .color(Color::Rgba(1.0, 0.0, 0.0, 1.0))
                .set(state.ids.text, &mut ui);
        },
        None => (),
    };
    (ui, state)
}