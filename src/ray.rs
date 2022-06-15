
use crate::vector::Vector;

pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> Vector {
        self.origin + t * self.direction
    }
}
