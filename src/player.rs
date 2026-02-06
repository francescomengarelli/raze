use super::math::Vector;
pub struct Player {
    position: Vector,
    direction: Vector,
}

impl Player {
    pub fn new(position: Vector, direction: Vector) -> Self {
        Self {
            position,
            direction,
        }
    }
    pub fn position(&self) -> &Vector {
        &self.position
    }

    pub fn direction(&self) -> &Vector {
        &self.direction
    }
}
