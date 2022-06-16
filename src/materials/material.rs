use crate::{hit_record::HitRecord, ray::Ray, vector::Vector};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vector, Ray)>;
}
