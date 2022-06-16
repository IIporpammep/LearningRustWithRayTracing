use crate::{
    hit_record::HitRecord,
    ray::Ray,
    vector::{random_on_unit_sphere, Vector},
};

use super::material::Material;

pub struct DiffuseMaterial {
    pub albedo: Vector,
}

impl Material for DiffuseMaterial {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Vector, Ray)> {
        let mut scatter_direction = hit_record.normal + random_on_unit_sphere();
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered_ray: Ray = Ray {
            origin: hit_record.origin,
            direction: scatter_direction,
        };

        Some((self.albedo, scattered_ray))
    }
}
