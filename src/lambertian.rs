use crate::hit_record::HitRecord;
use crate::materials::{Material, MaterialTrait};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Material {
        Material::Lambertian(Lambertian { albedo })
    }
}

impl MaterialTrait for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<(&Vec3, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        };

        let scattered = Ray::new(hit_record.point, scatter_direction);

        Some((&self.albedo, scattered))
    }
}