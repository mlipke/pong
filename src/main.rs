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

        self.position.0 += self.vector.0;
        self.position.1 += self.vector.1;
    }
}

pub struct Paddle {
    rectangle: [f64; 4],
    position: (f64, f64)
}

impl Paddle {
    fn move_paddle(&mut self, step: f64) {
        self.position.1 += step;
    }
}

pub struct App {
    gl: GlGraphics,
    ball: Ball,
    left_paddle: Paddle,
    right_paddle: Paddle
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        let ref ball = self.ball;
        let ref left_paddle = self.left_paddle;
        let ref right_paddle = self.right_paddle;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(color::BLACK, gl);

            let trans_left = c.transform.trans(left_paddle.position.0, left_paddle.position.1);
            let trans_right = c.transform.trans(right_paddle.position.0, right_paddle.position.1);

            rectangle(color::WHITE, left_paddle.rectangle, trans_left, gl);
            rectangle(color::WHITE, right_paddle.rectangle, trans_right, gl);

            let trans_ball = c.transform.trans(ball.position.0 - 5.0, ball.position.1 - 5.0);

            rectangle(color::WHITE, ball.rectangle, trans_ball, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.ball.update();
    }

    fn input(&mut self, args: &Button) {
        use piston::input::keyboard::Key;

        match *args {
            Button::Keyboard(Key::Up) => {
                self.left_paddle.move_paddle(-3.0);
                self.right_paddle.move_paddle(-3.0);
            },
            Button::Keyboard(Key::Down) => {
                self.left_paddle.move_paddle(3.0);
                self.right_paddle.move_paddle(3.0);
            },
            _ => {}
        }
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

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(p) = e.press_args() {
            app.input(&p);
        }
    }

}
