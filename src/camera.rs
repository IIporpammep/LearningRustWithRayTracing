use crate::{ray::Ray, vector::Vector};

pub struct Camera {
    origin: Vector,
    lower_left_corner: Vector,
    viewport_width: Vector,
    viewport_height: Vector,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Camera {
        // Use Unity-like left hand Y-up, Z-forward coordinate system.
        let origin: Vector = Vector {
            data: [0.0, 0.0, 0.0],
        };

        let height: f32 = 2.0;
        let width: f32 = aspect_ratio * height;
        let focal_length: f32 = 1.0;

        let viewport_width: Vector = Vector {
            data: [width, 0.0, 0.0],
        };
        let viewport_height: Vector = Vector {
            data: [0.0, height, 0.0],
        };

        let lower_left_corner: Vector = origin - viewport_width / 2.0 - viewport_height / 2.0
            + Vector {
                data: [0.0, 0.0, focal_length],
            };

        return Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            viewport_width: viewport_width,
            viewport_height: viewport_height,
        };
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.viewport_width + v * self.viewport_height,
        }
    }
}
