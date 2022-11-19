use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::sphere::Sphere;

/// Trait implemented by all hittable objects
pub trait HittableTrait {
    /// Checks if an object is hit by a ray and if so returns the hit record
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

/// Enum storing each hittable object variation
pub enum Hittable {
    Sphere(Sphere)
}

/// Calls methods for objects in the Hittable enum
impl HittableTrait for Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(obj) => obj.hit(ray, t_min, t_max)
        }
    }
}