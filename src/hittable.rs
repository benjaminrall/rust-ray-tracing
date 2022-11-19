use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::sphere::Sphere;

pub trait HittableTrait {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub enum Hittable {
    Sphere(Sphere)
}

impl HittableTrait for Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(obj) => obj.hit(ray, t_min, t_max)
        }
    }
}