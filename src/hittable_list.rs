use std::sync::Arc;
use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, HittableTrait};
use crate::ray::Ray;

#[derive(Debug)]
/// Stores a list of Hittable objects
pub struct HittableList {
    pub objects: Vec<Arc<Hittable>>
}

impl HittableList {
    /// Generates a new empty list of objects
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    /// Generates a list from a list of objects
    pub fn from_objects(objects: Vec<Hittable>) -> Self {
        let mut list = HittableList::new();
        for obj in objects {
            list.add(obj)
        }
        list
    }

    /// Adds a hittable object to the internal list
    pub fn add(&mut self, object: Hittable) {
        self.objects.push(Arc::new(object));
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

    /// Gets the combined bounding box of all objects in the list
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None
        }

        let mut output_box: Option<AABB> = None;

        for object in &self.objects {
            match object.bounding_box(time0, time1) {
                None => return None,
                Some(returned_box) => {
                    match output_box {
                        None => output_box = Some(returned_box),
                        Some(curr_box) => {
                            output_box = Some(AABB::surrounding_box(&curr_box, &returned_box))
                        }
                    }
                }
            }
        }

        output_box
    }
}