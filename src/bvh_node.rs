use std::cmp::Ordering;
use std::sync::Arc;
use ray_tracing::random_int;
use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, HittableTrait};
use crate::hittable_list::HittableList;
use crate::ray::Ray;

#[derive(Debug)]
/// Object to represent a node in a Bounding Volume Hierarchy
pub struct BVHNode {
    left: Arc<Hittable>,
    right: Arc<Hittable>,
    bounding_box: AABB
}

impl BVHNode {
    /// Constructs a BVH Tree from a Hittable List
    pub fn from_hittable_list(list: &HittableList, time0: f64, time1: f64) -> Hittable {
        BVHNode::new(&list.objects, 0, list.objects.len(), time0, time1)
    }

    /// Constructs a new BVH Tree from a list of Hittable objects
    pub fn new(
        src_objects: &Vec<Arc<Hittable>>, start: usize, end: usize, time0: f64, time1: f64,
    ) -> Hittable {
        let mut objects: Vec<Arc<Hittable>> = src_objects.clone();

        let axis = random_int(0, 2);
        let comparator = match axis {
            0 => BVHNode::box_x_compare,
            1 => BVHNode::box_y_compare,
            2 => BVHNode::box_z_compare,
            _ => panic!("Invalid axis for comparator.")
        };

        let object_span = end - start;

        let left;
        let right;
        let bounding_box;

        match object_span {
            1 => {
                left = Arc::clone(&objects[start]);
                right = Arc::clone(&objects[start]);
            },
            2 => {
                if comparator(&objects[start], &objects[start + 1]) {
                    left = Arc::clone(&objects[start]);
                    right = Arc::clone(&objects[start + 1]);
                } else {
                    left = Arc::clone(&objects[start + 1]);
                    right = Arc::clone(&objects[start]);
                }
            },
            _ => {
                objects.sort_by(|a, b|
                    if !comparator(a, b) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                );

                let mid = start + object_span / 2;
                left = Arc::new(BVHNode::new(&objects, start, mid, time0, time1));
                right = Arc::new(BVHNode::new(&objects, mid, end, time0, time1));
            }
        }

        let mut box_left;
        let mut box_right;

        match left.bounding_box(time0, time1) {
            Some(b) => box_left = b,
            None => panic!("No bounding box in BVHNode constructor.")
        }

        match right.bounding_box(time0, time1) {
            Some(b) => box_right = b,
            None => panic!("No bounding box in BVHNode constructor.")
        }

        bounding_box = AABB::surrounding_box(&box_left, &box_right);

        Hittable::BVHNode(BVHNode { left, right, bounding_box })
    }

    fn box_compare(a: &Arc<Hittable>, b: &Arc<Hittable>, axis: usize) -> bool {
        let box_a;
        let box_b;

        match a.bounding_box(0., 0.) {
            Some(b) => box_a = b,
            None => panic!("No bounding box in BVHNode constructor.")
        }

        match b.bounding_box(0., 0.) {
            Some(b) => box_b = b,
            None => panic!("No bounding box in BVHNode constructor.")
        }

        box_a.minimum[axis] < box_b.minimum[axis]
    }

    fn box_x_compare(a: &Arc<Hittable>, b: &Arc<Hittable>) -> bool {
        BVHNode::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<Hittable>, b: &Arc<Hittable>) -> bool {
        BVHNode::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<Hittable>, b: &Arc<Hittable>) -> bool {
        BVHNode::box_compare(a, b, 2)
    }
}

impl HittableTrait for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None
        }

        let left_hit = self.left.hit(ray, t_min, t_max);

        let mut t_max = t_max;
        if let Some(h) = &left_hit {
            t_max = h.t;
        }

        let right_hit = self.right.hit(ray, t_min, t_max);

        if let Some(_) = &right_hit {
            return right_hit
        }

        return left_hit
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(AABB::new(self.bounding_box.minimum, self.bounding_box.maximum))
    }
}