use crate::{
    ray::Ray,
    vector::{cross, random_in_unit_disc, Vector},
};

pub struct Camera {
    origin: Vector,
    lower_left_corner: Vector,
    viewport_width: Vector,
    viewport_height: Vector,
    right: Vector,
    up: Vector,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        origin: &Vector,
        target: &Vector,
        up: &Vector,
        aspect_ratio: f32,
        vertical_fov_deg: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Camera {
        // Use Unity-like left hand Y-up, Z-forward coordinate system.
        let theta = vertical_fov_deg.to_radians();
        let h = (theta / 2.0).tan();

        let height: f32 = 2.0 * h;
        let width: f32 = aspect_ratio * height;

        let camera_forward = ((*target) - (*origin)).normalize();
        let camera_right = cross(&up, &camera_forward).normalize();
        let camera_up = -cross(&camera_right, &camera_forward);

        let viewport_width: Vector = focus_distance * width * camera_right;
        let viewport_height: Vector = focus_distance * height * camera_up;

        let lower_left_corner: Vector = (*origin) - viewport_width / 2.0 - viewport_height / 2.0
            + focus_distance * camera_forward;

        return Camera {
            origin: (*origin),
            lower_left_corner: lower_left_corner,
            viewport_width: viewport_width,
            viewport_height: viewport_height,
            right: camera_right,
            up: camera_up,
            lens_radius: aperture / 2.0,
        };
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let random_position_on_lens = self.lens_radius * random_in_unit_disc();
        let offset =
            random_position_on_lens.x() * self.right + random_position_on_lens.y() * self.up;

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + u * self.viewport_width + v * self.viewport_height
                - self.origin
                - offset,
        }
    }
}
