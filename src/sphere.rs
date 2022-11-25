use std::sync::Arc;
use ray_tracing::PI;
use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, HittableTrait};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
/// Object to store a Sphere object
pub struct Sphere {
    centre: Vec3,               // Position of the centre of the sphere
    radius: f64,                // Radius of the sphere
    material: Arc<Material>,    // Material of the sphere
}

impl Sphere {
    /// Constructs a new Sphere object, wrapped in the Hittable enum
    pub fn new(centre: Vec3, radius: f64, material: Arc<Material>) -> Hittable {
        Hittable::Sphere(Sphere { centre, radius, material })
    }

    pub fn get_sphere_uv(p: Vec3) -> (f64, f64) {
        let theta = f64::acos(-p.y);
        let phi = f64::atan2(-p.z, p.x) + PI;

        (phi / (2. * PI), theta / PI)
    }
}

impl HittableTrait for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Calculates vector of the ray's origin to the centre of the sphere
        let oc = ray.origin - self.centre;

        // Calculates elements of quadratic equation to solve for the points of intersection
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c  = oc.length_squared() - self.radius * self.radius;

        // Calculates the discriminant of the equation and uses it to detect no intersections
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrt_d = f64::sqrt(discriminant);

        // Finds the smallest root within the range [t_min,t_max] or returns None if none exist
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        // Creates a new hit record for the interaction and returns it
        let record_point = ray.at(root);
        let outward_normal = (record_point - self.centre) / self.radius;
        let (u, v) = Sphere::get_sphere_uv(outward_normal);
        let mut hit_record = HitRecord::new(record_point, &self.material,
                                            u, v, root);
        hit_record.calculate_face_normal(ray, outward_normal);

        Some(hit_record)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        let r_vec = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(self.centre - r_vec, self.centre + r_vec))
    }
}