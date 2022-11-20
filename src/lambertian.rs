use crate::hit_record::HitRecord;
use crate::materials::{Material, MaterialTrait};
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Object to represent Lambertian diffuse materials
pub struct Lambertian {
    albedo: Vec3    // Albedo of the material
}

impl Lambertian {
    /// Constructs a new Lambertian object, wrapped in the Material enum
    pub fn new(albedo: Vec3) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }
}

impl MaterialTrait for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(&Vec3, Ray)> {
        // Gets the direction of the scattered ray, which bounces randomly away from the object
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        // Ignores scatter directions near zero to avoid impossible rays
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        };

        // Constructs scattered ray
        let scattered = Ray::new(hit_record.point, scatter_direction, ray_in.time);

        Some((&self.albedo, scattered))
    }
}