use crate::hit_record::HitRecord;
use crate::sphere::Sphere;
use crate::Ray;

pub trait HittableTrait {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub enum Hittable {
    Sphere(Sphere)
}

impl HittableTrait for Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        match self {
            Hittable::Sphere(obj) => obj.hit(ray, t_min, t_max, hit_record)
        }
    }
}