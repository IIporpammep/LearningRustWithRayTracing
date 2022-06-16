use super::hittable::Hittable;
use crate::{
    hit_record::{get_face_and_normal_against_ray, HitRecord},
    materials::material::Material,
    ray::Ray,
    vector::{dot, Vector},
};

pub struct Sphere {
    pub centre: Vector,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vector = (*ray).origin - (self.centre);
        let a: f32 = (*ray).direction.squared_length();
        let half_b: f32 = dot(&oc, &(*ray).direction);
        let c: f32 = oc.squared_length() - self.radius * self.radius;

        let discriminant: f32 = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();

        // Find the nearest root of quadratic equation that lies in the acceptable range.
        let mut root: f32 = (-half_b - discriminant_sqrt) / a;
        if root < t_min || root > t_max {
            root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let hit_position: Vector = (*ray).point_at_parameter(root);
        let hit_noraml: Vector = ((hit_position - self.centre) / self.radius).normalize();

        let (face, normal) = get_face_and_normal_against_ray(ray, hit_noraml);

        return Some(HitRecord {
            origin: hit_position,
            normal: normal,
            is_front_face: face,
            t: root,
            material: self.material.as_ref(),
        });
    }
}
