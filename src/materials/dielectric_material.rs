use rand::Rng;

use crate::{
    hit_record::HitRecord,
    ray::Ray,
    vector::{dot, Vector},
};

use super::material::Material;

pub struct DielectricMaterial {
    pub refraction_index: f32,
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vector, Ray)> {
        let refraction_ratio = if hit_record.is_front_face {
            // From air to this material.
            1.0 / self.refraction_index
        } else {
            // From this material to air.
            self.refraction_index
        };

        let direction_normalized = ray.direction.normalize();

        let cos_theta = dot(&-direction_normalized, &hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut random = rand::thread_rng();

        let scattered_ray_direction = if refraction_ratio * sin_theta > 1.0 || reflectance(cos_theta, refraction_ratio) > random.gen() {
            direction_normalized.reflect(&hit_record.normal)
        } else {
            direction_normalized.refract(&hit_record.normal, refraction_ratio)
        };

        let scattered_ray: Ray = Ray {
            origin: hit_record.origin,
            direction: scattered_ray_direction,
        };

        Some((
            Vector {
                data: [1.0, 1.0, 1.0],
            },
            scattered_ray,
        ))
    }
}

fn reflectance(cosine: f32, refraction_ratio: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let mut r_0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    r_0 = r_0 * r_0;

    r_0 + (1.0 - r_0) * (1.0 - cosine).powf(5.0)
}