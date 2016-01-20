use piston::input::*;
use opengl_graphics::GlGraphics;

use ball::Ball;
use paddle::Paddle;

pub struct App {
    pub gl: GlGraphics,
    pub ball: Ball,
    pub left_paddle: Paddle,
    pub right_paddle: Paddle,
    pub state: bool
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

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

    pub fn update(&mut self) {
        if self.state {
            self.ball.update();

            if self.ball.position.0 == 465.0 {
                match self.ball.hit(&self.right_paddle) {
                    Some(angle) => {
                        self.ball.angle = angle;
                        self.ball.reverse();
                    },
                    None => self.ball.update()
                }
            }

            if self.ball.position.0 == 15.0 {
                match self.ball.hit(&self.left_paddle) {
                    Some(angle) => {
                        self.ball.angle = angle;
                        self.ball.reverse();
                    },
                    None => self.ball.update()
                }
            }

            if self.ball.position.0 < 10.0 ||
               self.ball.position.0 > 470.0 {
                   self.ball.position = (240.0, 180.0);
                   self.ball.reference = (240.0, 180.0);
                   self.ball.angle = 0.0;
                   self.toggle_pause();
            }

            if self.ball.position.1 == 0.0 ||
               self.ball.position.1 == 360.0 {
                    self.ball.bounce()
            }
        }
    }

    pub fn key_paddle(&mut self, args: &Button) {
        use piston::input::keyboard::Key;

        match *args {
            Button::Keyboard(Key::Up) => {
                self.right_paddle.move_paddle(-8.0);
            },
            Button::Keyboard(Key::Down) => {
                self.right_paddle.move_paddle(8.0);
            },
            Button::Keyboard(Key::Space) => {
                self.toggle_pause();
            },
            _ => {}
        }
    }

    pub fn mouse_paddle(&mut self, args: &[f64; 2]) {
        self.left_paddle.move_paddle(args[1]);
    }

    pub fn toggle_pause(&mut self) {
        self.state = !self.state;
    }
}
