use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
/// Object representing an Axis-Aligned Bounding Box for grouping Hittable objects
pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3
}

impl AABB {
    /// Constructs a new AABB
    pub fn new(minimum: Vec3, maximum: Vec3) -> AABB {
        AABB { minimum, maximum }
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let minimum = Vec3::new(
            f64::min(box0.minimum.x, box1.minimum.x),
            f64::min(box0.minimum.y, box1.minimum.y),
            f64::min(box0.minimum.z, box1.minimum.z),
        );

        let maximum = Vec3::new(
            f64::max(box0.maximum.x, box1.maximum.x),
            f64::max(box0.maximum.y, box1.maximum.y),
            f64::max(box0.maximum.z, box1.maximum.z),
        );

        AABB::new(minimum, maximum)
    }

    /// Returns if a ray passed through the bounding box
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            // Calculates t0 and t1 for current dimension
            let inv_d = 1. / ray.direction[a];
            let mut t0 = (self.minimum[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.maximum[a] - ray.origin[a]) * inv_d;

            // Swaps values if the inverse direction is negative
            if inv_d < 0. {
                let temp = t0;
                t0 = t1.clone();
                t1 = temp;
            }

            // Returns false if the ray doesn't overlap with the current dimension
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false
            }
        }

        true
    }
}