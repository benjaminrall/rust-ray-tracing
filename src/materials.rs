use crate::HitRecord;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::dielectric::Dielectric;
use crate::Ray;
use crate::Vec3;

pub trait MaterialTrait {
    fn scatter(
        &self, ray_in: &Ray, hit_record: &HitRecord
    ) -> Option<(&Vec3, Ray)>;
}

pub enum Material {
    None,
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl MaterialTrait for Material {
    fn scatter(
        &self, ray_in: &Ray, hit_record: &HitRecord
    ) -> Option<(&Vec3, Ray)> {
        match self {
            Material::None => None,
            Material::Lambertian(obj) =>
                obj.scatter(ray_in, hit_record),
            Material::Metal(obj) =>
                obj.scatter(ray_in, hit_record),
            Material::Dielectric(obj) =>
                obj.scatter(ray_in, hit_record),
        }
    }
}