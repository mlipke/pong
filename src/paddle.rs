pub struct Paddle {
    pub rectangle: [f64; 4],
    pub position: (f64, f64)
}

impl Paddle {
    pub fn move_paddle(&mut self, step: f64) {
        self.position.1 += step;

        if self.position.1 < 0.0 {
            self.position.1 = 0.0;
        }

        if self.position.1 > 320.0 {
            self.position.1 = 320.0;
        }
    }
}
