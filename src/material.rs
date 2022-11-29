use crate::dielectric::Dielectric;
use crate::diffuse_light::DiffuseLight;
use crate::hit_record::HitRecord;
use crate::isotropic::Isotropic;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Trait implemented by all materials
pub trait MaterialTrait {
    /// Calculates scattered ray and colour for a given hit with a material
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;

    /// Returns the emitted colour of a material
    fn emitted(&self, _: f64, _: f64, _: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

#[derive(Debug)]
/// Enum storing each material variation
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Isotropic(Isotropic),
}

/// Calls methods for materials in the Material enum
impl MaterialTrait for Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        match self {
            Material::Lambertian(obj) => obj.scatter(ray_in, hit_record),
            Material::Metal(obj) => obj.scatter(ray_in, hit_record),
            Material::Dielectric(obj) => obj.scatter(ray_in, hit_record),
            Material::DiffuseLight(obj) => obj.scatter(ray_in, hit_record),
            Material::Isotropic(obj) => obj.scatter(ray_in, hit_record),
        }
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        match self {
            Material::Lambertian(obj) => obj.emitted(u, v, p),
            Material::Metal(obj) => obj.emitted(u, v, p),
            Material::Dielectric(obj) => obj.emitted(u, v, p),
            Material::DiffuseLight(obj) => obj.emitted(u, v, p),
            Material::Isotropic(obj) => obj.emitted(u, v, p),
        }
    }
}