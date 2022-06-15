use crate::{ray::Ray, hit_record::HitRecord};

pub trait Hittable
{
    fn hit(&self, ray : &Ray, t_min : f32, t_max :f32) -> Option<HitRecord>;
}