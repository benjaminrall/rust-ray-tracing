use std::sync::Arc;
use crate::hit_record::HitRecord;
use crate::material::{Material, MaterialTrait};
use crate::ray::Ray;
use crate::solid_colour::SolidColour;
use crate::texture::{Texture, TextureTrait};
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Isotropic {
    albedo: Arc<Texture>
}

impl Isotropic {
    pub fn new(albedo: Arc<Texture>) -> Material {
        Material::Isotropic(Isotropic { albedo })
    }

    pub fn from_colour(colour: Vec3) -> Material {
        Material::Isotropic(Isotropic { albedo: Arc::new(SolidColour::new(colour)) })
    }
}

impl MaterialTrait for Isotropic {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = self.albedo.value(hit_record.u, hit_record.v, &hit_record.point);
        let scattered = Ray::new(hit_record.point, Vec3::random_in_unit_sphere(), ray_in.time);
        Some((attenuation, scattered))
    }
}