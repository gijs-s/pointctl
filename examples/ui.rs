extern crate kiss3d;
extern crate nalgebra as na;

// Third party
use kiss3d::camera::ArcBall;
use kiss3d::camera::Camera;
use kiss3d::event::{Action, WindowEvent};
use kiss3d::light::Light;
use kiss3d::planar_camera::{PlanarCamera, Sidescroll};
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::renderer::Renderer;
use kiss3d::text::Font;
use kiss3d::window::{State, Window};
// Conrod
use kiss3d::conrod::{widget, widget_ids, Widget};
use na::{Point2, Point3};
use rstar::RTree;

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        text
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
    let mut ui = window.conrod_ui_mut().set_widgets();
    widget::Text::new("test").set(ids.text, &mut ui);

    // Start the render loop, this will _not_ work with 2D scenes yet.
    while window.render() {}
}
