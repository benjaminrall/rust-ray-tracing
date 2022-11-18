use std::sync::Arc;
use crate::Ray;
use crate::Vec3;
use crate::materials::Material;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<Material>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn empty() -> Self {
        HitRecord { point: Vec3::zero(), normal: Vec3::zero(),
            material: Arc::new(Material::None), t: 0.0, front_face: false}
    }

    pub fn copy_record(&mut self, hit_record: &HitRecord) {
        self.point = hit_record.point;
        self.normal = hit_record.normal;
        self.t = hit_record.t;
        self.front_face = hit_record.front_face;
        self.material = Arc::clone(&hit_record.material);
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&ray.get_direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }

    pub fn set_material(&mut self, material: &Arc<Material>) {
        self.material = Arc::clone(material);
    }
}
