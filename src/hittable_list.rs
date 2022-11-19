use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, HittableTrait};
use crate::ray::Ray;

/// Stores a list of Hittable objects
pub struct HittableList {
    objects: Vec<Hittable>
}

impl HittableList {
    /// Generates a new empty list of objects
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    /// Adds a hittable object to the internal list
    pub fn add(&mut self, object: Hittable) {
        self.objects.push(object);
    }
}

impl HittableTrait for HittableList {
    /// Checks if any object in the list is hit by a given ray
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}