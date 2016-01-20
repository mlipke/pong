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
use ball::Ball;
use paddle::Paddle;

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

    let mut app = App {
        gl: GlGraphics::new(opengl),
        state: false,
        ball: Ball {
            rectangle: [0.0, 0.0, 10.0, 10.0],
            position: (240.0, 180.0),
            angle: 0.0,
            reference: (240.0, 180.0),
            direction: 1.0
        },
        left_paddle: Paddle {
            rectangle: [0.0, 0.0, 10.0, 40.0],
            position: (5.0, 160.0)
        },
        right_paddle: Paddle {
            rectangle: [0.0, 0.0, 10.0, 40.0],
            position: (465.0, 160.0)
        }
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
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
