use std::sync::Arc;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<Material>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(point: Vec3, material: &Arc<Material>, t: f64) -> Self {
        HitRecord {
            point, t, normal: Vec3::zero(), material: Arc::clone(material), front_face: false
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&ray.get_direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}
