use crate::hit_record::HitRecord;
use crate::materials::{Material, MaterialTrait};
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Object to represent Metal materials
pub struct Metal {
    albedo: Vec3,   // Albedo of the material
    fuzz: f64       // Controls how fuzzy a reflection is
}

impl Metal {
    /// Constructs a new Metal object, wrapped in the Material enum
    pub fn new(albedo: Vec3, fuzz: f64) -> Material {
        Material::Metal(Metal { albedo, fuzz })
    }
}

impl MaterialTrait for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(&Vec3, Ray)> {
        // Gets the reflected direction and constructs the scattered ray with a certain fuzz
        let reflected = Ray::reflect(ray_in.direction.unit(), hit_record.normal);
        let scattered = Ray::new(
            hit_record.point, reflected + self.fuzz * Vec3::random_in_unit_sphere(), ray_in.time
        );

        // Excludes the ray if it's reflected towards the object
        if Vec3::dot(&scattered.direction, &hit_record.normal) > 0. {
            Some((&self.albedo, scattered))
        } else {
            None
        }
    }
}