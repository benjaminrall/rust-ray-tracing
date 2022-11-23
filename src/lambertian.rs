use std::sync::Arc;
use crate::hit_record::HitRecord;
use crate::material::{Material, MaterialTrait};
use crate::ray::Ray;
use crate::solid_colour::SolidColour;
use crate::texture::{Texture, TextureTrait};
use crate::vec3::Vec3;

#[derive(Debug)]
/// Object to represent Lambertian diffuse materials
pub struct Lambertian {
    albedo: Arc<Texture>    // Albedo of the material
}

impl Lambertian {
    /// Constructs a new Lambertian object from a given colour, wrapped in the Material enum
    pub fn new(colour: Vec3) -> Material {
        Material::Lambertian(Lambertian { albedo: Arc::new(SolidColour::new(colour)) })
    }

    /// Constructs a new Lambertian object from a given texture, wrapped in the Material enum
    pub fn from_texture(albedo: Arc<Texture>) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }
}

impl MaterialTrait for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        // Gets the direction of the scattered ray, which bounces randomly away from the object
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        // Ignores scatter directions near zero to avoid impossible rays
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        };

        // Constructs scattered ray
        let scattered = Ray::new(hit_record.point, scatter_direction, ray_in.time);

        Some((self.albedo.value(hit_record.u, hit_record.v, &hit_record.point), scattered))
    }
}