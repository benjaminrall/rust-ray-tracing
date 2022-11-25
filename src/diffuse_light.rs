use std::sync::Arc;
use crate::hit_record::HitRecord;
use crate::material::{Material, MaterialTrait};
use crate::ray::Ray;
use crate::solid_colour::SolidColour;
use crate::texture::{Texture, TextureTrait};
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct DiffuseLight {
    emit: Arc<Texture>
}

impl DiffuseLight {
    pub fn new(emit: Arc<Texture>) -> Material {
        Material::DiffuseLight(DiffuseLight { emit })
    }

    pub fn from_colour(r: f64, g: f64, b: f64) -> Material {
        Material::DiffuseLight(DiffuseLight { emit: Arc::new(SolidColour::new(Vec3::new(r, g, b)))})
    }
}

impl MaterialTrait for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}