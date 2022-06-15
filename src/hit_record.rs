use crate::{
    ray::Ray,
    vector::{dot, Vector},
};

#[derive(Default)]
pub struct HitRecord {
    pub origin: Vector,
    // It's always against the cast ray.
    pub normal: Vector,
    pub t: f32,
    pub is_front_face: bool,
}

pub fn get_face_and_normal_against_ray(ray: &Ray, outward_normal: Vector) -> (bool, Vector) {
    let is_front_face = dot(&(*ray).direction, &outward_normal) < 0.0;
    let normal = if is_front_face {
        outward_normal
    } else {
        -outward_normal
    };

    return (is_front_face, normal);
}
