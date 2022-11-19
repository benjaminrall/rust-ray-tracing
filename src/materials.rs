use crate::dielectric::Dielectric;
use crate::hit_record::HitRecord;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait MaterialTrait {
    fn scatter(
        &self, ray_in: &Ray, hit_record: &HitRecord
    ) -> Option<(&Vec3, Ray)>;
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl MaterialTrait for Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(&Vec3, Ray)> {
        match self {
            Material::Lambertian(obj) => obj.scatter(ray_in, hit_record),
            Material::Metal(obj) => obj.scatter(ray_in, hit_record),
            Material::Dielectric(obj) => obj.scatter(ray_in, hit_record),
        }
    }
}