use crate::aabb::AABB;
use crate::bvh_node::BVHNode;
use crate::hit_record::HitRecord;
use crate::moving_sphere::MovingSphere;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::aa_rect::XYRect;

/// Trait implemented by all hittable objects
pub trait HittableTrait {
    /// Checks if the object is hit by a ray and if so returns the hit record
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    /// Gets the bounding box of the object
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}

#[derive(Debug)]
/// Enum storing each hittable object variation
pub enum Hittable {
    BVHNode(BVHNode),
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    XYRect(XYRect),
}

/// Calls methods for objects in the Hittable enum
impl HittableTrait for Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Hittable::BVHNode(obj) => obj.hit(ray, t_min, t_max),
            Hittable::Sphere(obj) => obj.hit(ray, t_min, t_max),
            Hittable::MovingSphere(obj) => obj.hit(ray, t_min, t_max),
            Hittable::XYRect(obj) => obj.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        match self {
            Hittable::BVHNode(obj) => obj.bounding_box(time0, time1),
            Hittable::Sphere(obj) => obj.bounding_box(time0, time1),
            Hittable::MovingSphere(obj) => obj.bounding_box(time0, time1),
            Hittable::XYRect(obj) => obj.bounding_box(time0, time1),
        }
    }
}