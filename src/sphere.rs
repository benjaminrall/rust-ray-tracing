use std::sync::Arc;
use crate::{HitRecord, Hittable};
use crate::material::MaterialType;
use crate::Ray;
use crate::Vec3;

pub struct Sphere {
    centre: Vec3,
    radius: f64,
    material: Arc<MaterialType>,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f64, material: Arc<MaterialType>) -> Self {
        Sphere { centre, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = ray.get_origin() - self.centre;
        let a = ray.get_direction().length_squared();
        let half_b = Vec3::dot(&oc, &ray.get_direction());
        let c  = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = f64::sqrt(discriminant);

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(root);
        let outward_normal = (hit_record.point - self.centre) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);
        hit_record.set_material(&self.material);

        true
    }
}