use paddle::Paddle;

pub struct Ball {
    pub rectangle: [f64; 4],
    pub position: (f64, f64),
    pub vector: (f64, f64)
}

impl Ball {
    pub fn update(&mut self) {
        self.position.0 += self.vector.0;
        self.position.1 += self.vector.1;
    }

    pub fn hit(&mut self, paddle: &Paddle) -> bool {
        let min = paddle.position.1;
        let max = paddle.position.1 + 40.0;

        if self.position.1 > min && self.position.1 < max {
            return true;
        } else {
            return false;
        }
    }
}
