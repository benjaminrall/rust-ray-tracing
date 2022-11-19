use std::sync::Arc;
use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, HittableTrait};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    centre: Vec3,
    radius: f64,
    material: Arc<Material>,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f64, material: Arc<Material>) -> Hittable {
        Hittable::Sphere(Sphere { centre, radius, material })
    }
}

impl HittableTrait for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.get_origin() - self.centre;
        let a = ray.get_direction().length_squared();
        let half_b = Vec3::dot(&oc, &ray.get_direction());
        let c  = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = f64::sqrt(discriminant);

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut hit_record = HitRecord::new(ray.at(root), &self.material, root);
        let outward_normal = (hit_record.point - self.centre) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}