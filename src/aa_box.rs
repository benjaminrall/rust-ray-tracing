use std::sync::Arc;
use crate::aa_rect::{XYRect, XZRect, YZRect};
use crate::aabb::AABB;
use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, HittableTrait};
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct AABox {
    box_min: Vec3,
    box_max: Vec3,
    sides: HittableList,
}

impl AABox {
    pub fn new(box_min: Vec3, box_max: Vec3, material: Arc<Material>) -> Hittable {
        let mut sides = HittableList::new();

        sides.add(XYRect::new(box_min.x, box_max.x, box_min.y, box_max.y, box_min.z, Arc::clone(&material)));
        sides.add(XYRect::new(box_min.x, box_max.x, box_min.y, box_max.y, box_max.z, Arc::clone(&material)));

        sides.add(XZRect::new(box_min.x, box_max.x, box_min.z, box_max.z, box_min.y, Arc::clone(&material)));
        sides.add(XZRect::new(box_min.x, box_max.x, box_min.z, box_max.z, box_max.y, Arc::clone(&material)));

        sides.add(YZRect::new(box_min.y, box_max.y, box_min.z, box_max.z, box_min.x, Arc::clone(&material)));
        sides.add(YZRect::new(box_min.y, box_max.y, box_min.z, box_max.z, box_max.x, Arc::clone(&material)));

        Hittable::AABox(AABox { box_min, box_max, sides })
    }
}

impl HittableTrait for AABox {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }
}