use std::sync::Arc;
use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, HittableTrait};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Object to store a Moving Sphere object
pub struct MovingSphere {
    centre0: Vec3,              // Position of the centre of the sphere at time0
    centre1: Vec3,              // Position of the centre of the sphere at time1
    time0: f64,                 // Time of first position
    time1: f64,                 // Time of second position
    radius: f64,                // Radius of the sphere
    material: Arc<Material>,    // Material of the sphere
}

impl MovingSphere {
    /// Constructs a new Moving Sphere object, wrapped in the Hittable enum
    pub fn new(
        centre0: Vec3, centre1: Vec3, time0: f64, time1: f64, radius: f64, material: Arc<Material>
    ) -> Hittable {
        Hittable::MovingSphere(MovingSphere { centre0, centre1, time0, time1, radius, material })
    }

    /// Gets the centre position of the sphere at a certain time
    pub fn centre(&self, time: f64) -> Vec3{
        self.centre0 + ((time - self.time0) / (self.time1 - self.time0))
            * (self.centre1 - self.centre0)
    }
}

impl HittableTrait for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Calculates vector of the ray's origin to the current centre of the sphere
        let oc = ray.origin - self.centre(ray.time);

        // Calculates elements of quadratic equation to solve for the points of intersection
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c  = oc.length_squared() - self.radius * self.radius;

        // Calculates the discriminant of the equation and uses it to detect no intersections
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
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
        let mut hit_record = HitRecord::new(ray.at(root), &self.material, root);
        let outward_normal = (hit_record.point - self.centre(ray.time)) / self.radius;
        hit_record.calculate_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}