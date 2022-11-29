use std::sync::Arc;
use ray_tracing::{degrees_to_radians, INFINITY};
use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, HittableTrait};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct RotateY {
    object: Arc<Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(object: Arc<Hittable>, angle: f64) -> Hittable {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box(0., 1.);

        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    if let Some(b) = &bbox {
                        let x = i as f64 * b.maximum.x + (1. - i as f64) * b.minimum.x;
                        let y = j as f64 * b.maximum.y + (1. - j as f64) * b.minimum.y;
                        let z = k as f64 * b.maximum.z + (1. - k as f64) * b.minimum.z;

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(new_x, y, new_z);

                        for c in 0..3 {
                            min[c] = f64::min(min[c], tester[c]);
                            max[c] = f64::max(max[c], tester[c]);
                        }
                    }
                }
            }
        }

        let bbox = match bbox{
            None => None,
            Some(_) => Some(AABB::new(min, max))
        };

        Hittable::RotateY(RotateY {object, cos_theta, sin_theta, bbox})
    }

    fn rotate_ray_vec(&self, v: &Vec3) -> Vec3 {
        Vec3::new(
            self.cos_theta * v.x - self.sin_theta * v.z, v.y,
            self.sin_theta * v.x + self.cos_theta * v.z
        )
    }

    fn rotate_record_vec(&self, v: &Vec3) -> Vec3 {
        Vec3::new(
            self.cos_theta * v.x + self.sin_theta * v.z, v.y,
            -self.sin_theta * v.x + self.cos_theta * v.z
        )
    }
}

impl HittableTrait for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let rotated_ray = Ray::new(
            self.rotate_ray_vec(&ray.origin), self.rotate_ray_vec(&ray.direction), ray.time
        );

        match self.object.hit(&rotated_ray, t_min, t_max) {
            None => None,
            Some(rec) => {
                let mut hit_record = HitRecord::new(
                    self.rotate_record_vec(&rec.point), &rec.material, rec.u, rec.v, rec.t
                );
                hit_record.calculate_face_normal(&rotated_ray, self.rotate_record_vec(&rec.normal));

                Some(hit_record)
            }
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        match &self.bbox {
            None => None,
            Some(b) => {
                Some(AABB::new(b.minimum, b.maximum))
            }
        }
    }
}