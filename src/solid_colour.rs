use crate::texture::{Texture, TextureTrait};
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct SolidColour {
    colour_value: Vec3
}

impl SolidColour {
    pub fn new(colour_value: Vec3) -> Texture {
        Texture::SolidColour(SolidColour { colour_value })
    }
}

impl TextureTrait for SolidColour {
    fn value(&self, _: f64, _: f64, _: &Vec3) -> Vec3 {
        self.colour_value
    }
}