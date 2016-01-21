use piston::input::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;

use ball::Ball;
use paddle::Paddle;

use std::path::Path;

struct Resources {
    font: GlyphCache<'static>
}

pub struct App {
    ball: Ball,
    left_paddle: Paddle,
    right_paddle: Paddle,
    state: bool,
    score: (i32, i32),
    resources: Resources
}

impl App {
    pub fn new() -> App {
        App {
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
            },
            score: (0, 0),
            resources: Resources {
                font: GlyphCache::new(Path::new("assets/retro_computer.ttf")).unwrap()
            }
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;

        let ref ball = self.ball;
        let ref left_paddle = self.left_paddle;
        let ref right_paddle = self.right_paddle;
        let ref score = self.score;
        let ref mut resources = self.resources;

        gl.draw(args.viewport(), |c, gl| {
            clear(color::BLACK, gl);

            let trans_left = c.transform.trans(left_paddle.position.0, left_paddle.position.1);
            let trans_right = c.transform.trans(right_paddle.position.0, right_paddle.position.1);

            rectangle(color::WHITE, left_paddle.rectangle, trans_left, gl);
            rectangle(color::WHITE, right_paddle.rectangle, trans_right, gl);

            let trans_ball = c.transform.trans(ball.position.0 - 5.0, ball.position.1 - 5.0);

            rectangle(color::WHITE, ball.rectangle, trans_ball, gl);

            let trans_score_left = c.transform.trans(190.0, 55.0);
            let trans_score_right = c.transform.trans(260.0, 55.0);
            let mut text = Text::new(50);
            text.color = color::WHITE;

            text.draw(&format!("{}", score.0),
                &mut resources.font, &c.draw_state, trans_score_left, gl);

            text.draw(&format!("{}", score.1),
                &mut resources.font, &c.draw_state, trans_score_right, gl);
        });
    }

    pub fn update(&mut self) {
        if self.state {
            self.ball.update();

            if self.ball.position.0 == 465.0 {
                match self.ball.hit(&self.right_paddle) {
                    Some(angle) => {
                        self.ball.angle = angle;
                        self.ball.reverse(true);
                    },
                    None => self.ball.update()
                }
            }

            if self.ball.position.0 == 15.0 {
                match self.ball.hit(&self.left_paddle) {
                    Some(angle) => {
                        self.ball.angle = angle;
                        self.ball.reverse(false);
                    },
                    None => self.ball.update()
                }
            }

            if self.ball.position.0 < 10.0 {
                self.score.1 += 1;
                self.ball.reset();
                self.toggle_pause();
            }

            if self.ball.position.0 > 470.0 {
                self.score.0 +=1;
                self.ball.reset();
                self.toggle_pause();
            }

            if self.ball.position.1 < 5.0 ||
               self.ball.position.1 > 355.0 {
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
