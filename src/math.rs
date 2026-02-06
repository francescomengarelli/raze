pub struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn normalized(&self) -> Self {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            return Self { x: 0.0, y: 0.0 };
        }

        return Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
        };
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
