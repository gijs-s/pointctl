//! having raw gl windows works, 2 kiss3d windows does not. kiss3d does some dark
/// incantations with the framebuffer that seems to corrupt when multiple windows are in use.
extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::window::Window;
use na::Point3;

/// Segfault example
fn main() {
    let (mut window1, mut window2) = (
        Window::new("Kiss3d: window 1"),
        Window::new("Kiss3d: window 2"),
    );
    window1.set_light(Light::StickToCamera);
    window2.set_light(Light::StickToCamera);

    'main: loop {
        if !(window1.render() && window2.render()) {
            break 'main;
        }

        let a = Point3::new(-0.1, -0.1, 0.0);
        let b = Point3::new(0.0, 0.1, 0.0);
        let c = Point3::new(0.1, -0.1, 0.0);

        if !window1.render() {
            window1.draw_line(&a, &b, &Point3::new(1.0, 0.0, 0.0));
            window1.draw_line(&b, &c, &Point3::new(0.0, 1.0, 0.0));
            window1.draw_line(&c, &a, &Point3::new(0.0, 0.0, 1.0));
        }

        if !window2.render() {
            window2.draw_line(&a, &b, &Point3::new(1.0, 0.0, 0.0));
            window2.draw_line(&b, &c, &Point3::new(0.0, 1.0, 0.0));
            window2.draw_line(&c, &a, &Point3::new(0.0, 0.0, 1.0));
        }
    }
}
