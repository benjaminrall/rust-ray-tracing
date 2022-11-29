use std::sync::Arc;
use ray_tracing::{INFINITY, random_double};
use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, HittableTrait};
use crate::isotropic::Isotropic;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct ConstantMedium {
    boundary: Arc<Hittable>,
    phase_function: Arc<Material>,
    neg_inv_density: f64
}

impl ConstantMedium {
    pub fn new(boundary: Arc<Hittable>, density: f64, albedo: Arc<Texture>) -> Hittable {
        Hittable::ConstantMedium(ConstantMedium {
            boundary, neg_inv_density: (-1. / density), phase_function: Arc::new(Isotropic::new(albedo))
        })
    }

    pub fn from_colour(boundary: Arc<Hittable>, density: f64, colour: Vec3) -> Hittable {
        Hittable::ConstantMedium(ConstantMedium {
            boundary, neg_inv_density: (-1. / density), phase_function: Arc::new(Isotropic::from_colour(colour))
        })
    }
}

impl HittableTrait for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec1;
        let mut rec2;

        match self.boundary.hit(ray, -INFINITY, INFINITY) {
            None => return None,
            Some(rec) => rec1 = rec
        }

        match self.boundary.hit(ray, rec1.t + 0.0001, INFINITY) {
            None => return None,
            Some(rec) => rec2 = rec
        }

        if rec1.t < t_min { rec1.t = t_min };
        if rec2.t > t_max { rec2.t = t_max };

        if rec1.t >= rec2.t { return None };
        if rec1.t < 0. { rec1.t = 0. };

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary { return None };

        let t = rec1.t + hit_distance / ray_length;
        let mut hit_record = HitRecord::new(ray.at(t), &self.phase_function, 0., 0., t);
        hit_record.normal = Vec3::new(1., 0., 0.);
        hit_record.front_face = true;

        Some(hit_record)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}