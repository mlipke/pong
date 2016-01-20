pub struct Ball {
    pub rectangle: [f64; 4],
    pub position: (f64, f64),
    pub vector: (f64, f64)
}

impl Ball {
    pub fn update(&mut self) {
        if self.position.0 >= 460.0 {
            self.vector.0 = -1.0;
        } else if self.position.0 <= 15.0    {
            self.vector.0 = 1.0;
        }

        self.position.0 += self.vector.0;
        self.position.1 += self.vector.1;
    }
}
