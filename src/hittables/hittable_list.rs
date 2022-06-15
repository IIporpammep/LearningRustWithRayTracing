use crate::hit_record::HitRecord;

use super::hittable::Hittable;

pub struct HittableList {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_something = false;
        let mut hit_record: HitRecord = HitRecord::default();
        let mut closest_t: f32 = t_max;

        for hittable in self.hittables.iter() {
            match hittable.hit(ray, t_min, t_max) {
                Some(hit_result) => {
                    if hit_result.t < closest_t {
                        closest_t = hit_result.t;
                        hit_something = true;
                        hit_record = hit_result;
                    }
                }
                None => (),
            }
        }

        if hit_something {
            return Some(hit_record);
        }
        return None;
    }
}
