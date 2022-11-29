use std::sync::Arc;
use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, HittableTrait};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Translate {
    object: Arc<Hittable>,
    offset: Vec3
}

impl Translate {
    pub fn new(object: Arc<Hittable>, offset: Vec3) -> Hittable {
        Hittable::Translate(Translate { object, offset })
    }
}

impl HittableTrait for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        match self.object.hit(&moved_ray, t_min, t_max) {
            None => None,
            Some(mut rec) => {
                let mut hit_record = HitRecord::new(
                    rec.point + self.offset, &rec.material, rec.u, rec.v, rec.t
                );
                hit_record.calculate_face_normal(&moved_ray, rec.normal);

                Some(hit_record)
            }
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        match self.object.bounding_box(time0, time1) {
            None => None,
            Some(output_box) => {
                Some(AABB::new(
                    output_box.minimum + self.offset,
                    output_box.maximum + self.offset
                ))
            }
        }
    }
}