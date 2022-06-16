use crate::hit_record::HitRecord;

use super::hittable::Hittable;

pub struct HittableList {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_t: f32 = t_max;

        for hittable in self.hittables.iter() {
            match hittable.hit(ray, t_min, t_max) {
                Some(hit_result) => {
                    if hit_result.t < closest_t {
                        closest_t = hit_result.t;
                        hit_record = Some(hit_result);
                    }
                }
                None => (),
            }
        }
        return hit_record;
    }
}
