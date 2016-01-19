extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

mod color {
    pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
}

pub struct Ball {
    rectangle: [f64; 4],
    position: (f64, f64),
    vector: (f64, f64)
}

impl Ball {
    fn update(&mut self) {
        if self.position.0 >= 460.0 {
            self.vector.0 = -1.0;
        } else if self.position.0 <= 15.0    {
            self.vector.0 = 1.0;
        }

        self.position.0 = self.position.0 + self.vector.0;
        self.position.1 = self.position.1 + self.vector.1;
    }
}

pub struct App {
    gl: GlGraphics,
    ball: Ball
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let paddle: [f64; 4] = [0.0, 0.0, 10.0, 40.0];

        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        let ref ball = self.ball;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(color::BLACK, gl);

            let trans_left = c.transform.trans(5.0, y - 20.0);
            let trans_right = c.transform.trans(465.0, y - 20.0);

            rectangle(color::WHITE, paddle, trans_left, gl);
            rectangle(color::WHITE, paddle, trans_right, gl);

            let trans_ball = c.transform.trans(ball.position.0 - 5.0, ball.position.1 - 5.0);

            rectangle(color::WHITE, ball.rectangle, trans_ball, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.ball.update();
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "pong",
        [480, 360]
    )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        ball: Ball {
            rectangle: [0.0, 0.0, 10.0, 10.0],
            position: (240.0, 180.0),
            vector: (1.0, 0.0)
        }
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }

}
