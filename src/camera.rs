use crate::{
    ray::Ray,
    vector::{cross, Vector},
};

pub struct Camera {
    origin: Vector,
    lower_left_corner: Vector,
    viewport_width: Vector,
    viewport_height: Vector,
}

impl Camera {
    pub fn new(
        origin: &Vector,
        target: &Vector,
        up: &Vector,
        aspect_ratio: f32,
        vertical_fov_deg: f32,
    ) -> Camera {
        // Use Unity-like left hand Y-up, Z-forward coordinate system.
        let theta = vertical_fov_deg.to_radians();
        let h = (theta / 2.0).tan();

        let height: f32 = 2.0 * h;
        let width: f32 = aspect_ratio * height;

        let camera_forward = ((*target) - (*origin)).normalize();
        let camera_right = cross(&up, &camera_forward).normalize();
        let camera_up = - cross(&camera_right, &camera_forward);

        let viewport_width: Vector = width * camera_right;
        let viewport_height: Vector = height * camera_up;

        let lower_left_corner: Vector =
            (*origin) - viewport_width / 2.0 - viewport_height / 2.0 + camera_forward;

        return Camera {
            origin: (*origin),
            lower_left_corner: lower_left_corner,
            viewport_width: viewport_width,
            viewport_height: viewport_height,
        };
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.viewport_width + v * self.viewport_height
                - self.origin,
        }
    }
}
