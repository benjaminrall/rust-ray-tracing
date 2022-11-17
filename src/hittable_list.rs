use crate::{HitRecord, Hittable};
use crate::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Sync>>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: impl Hittable + Sync + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                hit_record.copy_record(&temp_record);
            }
        }

        hit_anything
    }
}