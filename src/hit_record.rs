use std::sync::Arc;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Object to store a record of a ray hit
pub struct HitRecord {
    pub point: Vec3,                // Point of the hit
    pub normal: Vec3,               // Normal of the face of the object hit
    pub material: Arc<Material>,    // Material of the object hit
    pub t: f64,                     // Time in the ray's lifetime that the hit occurred
    pub front_face: bool            // Stores if the hit was on an outward face of the object hit
}

impl HitRecord {
    /// Constructs a new Hit Record
    pub fn new(point: Vec3, material: &Arc<Material>, t: f64) -> Self {
        HitRecord {
            point, t, normal: Vec3::zero(), material: Arc::clone(material), front_face: false
        }
    }

    /// Calculates the normal of the hit record
    pub fn calculate_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&ray.get_direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}
