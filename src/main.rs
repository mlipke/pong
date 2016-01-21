extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

mod app;
mod ball;
mod paddle;

use app::App;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Pong",
        [480, 360]
    )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut app = App::new();
    let mut gl = GlGraphics::new(opengl);

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, &mut gl);
        }

        if let Some(_) = e.update_args() {
            app.update();
        }

        if let Some(p) = e.press_args() {
            app.key_paddle(&p);
        }

        if let Some(m) = e.mouse_scroll_args() {
            app.mouse_paddle(&m);
        }
    }

}
