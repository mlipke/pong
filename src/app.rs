use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use ball::Ball;
use paddle::Paddle;

pub struct App {
    pub gl: GlGraphics,
    pub ball: Ball,
    pub left_paddle: Paddle,
    pub right_paddle: Paddle
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
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

    pub fn update(&mut self, args: &UpdateArgs) {
        self.ball.update();
    }

    pub fn input(&mut self, args: &Button) {
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
