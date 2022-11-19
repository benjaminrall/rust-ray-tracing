use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, HittableTrait};
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Hittable>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Hittable) {
        self.objects.push(object);
    }
}

impl HittableTrait for HittableList {
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