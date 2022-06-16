use crate::{
    hit_record::HitRecord,
    ray::Ray,
    vector::{dot, Vector, random_on_unit_sphere},
};

use super::material::Material;

pub struct MetalMaterial {
    pub albedo: Vector,
    pub fuzziness : f32
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vector, Ray)> {
        let reflected_direction = ray.direction.reflect(&hit_record.normal);

        let scattered_ray: Ray = Ray {
            origin: hit_record.origin,
            direction: reflected_direction + self.fuzziness * random_on_unit_sphere(),
        };

        if dot(&scattered_ray.direction, &hit_record.normal) > 0.0 {
           return Some((self.albedo, scattered_ray))
        }

        None
    }
}
