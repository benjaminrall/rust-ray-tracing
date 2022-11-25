use std::sync::Arc;
use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, HittableTrait};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Arc<Material>
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Arc<Material>) -> Hittable {
        Hittable::XYRect(XYRect { x0, x1, y0, y1, k, material })
    }
}

impl HittableTrait for XYRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return None
        }
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None
        }

        let outward_normal = Vec3::new(0., 0., 1.);
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let mut hit_record = HitRecord::new(ray.at(t), &self.material, u, v, t);
        hit_record.calculate_face_normal(ray, outward_normal);

        Some(hit_record)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}