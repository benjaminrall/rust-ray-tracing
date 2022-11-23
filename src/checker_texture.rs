use std::sync::Arc;
use crate::solid_colour::SolidColour;
use crate::texture::{Texture, TextureTrait};
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct CheckerTexture {
    even: Arc<Texture>,
    odd: Arc<Texture>
}

impl CheckerTexture {
    pub fn new(even_colour: Vec3, odd_colour: Vec3) -> Texture {
        let even = Arc::new(SolidColour::new(even_colour));
        let odd = Arc::new(SolidColour::new(odd_colour));
        Texture::CheckerTexture(CheckerTexture { even, odd })
    }
}

impl TextureTrait for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10. * p.x).sin() * (10. * p.y).sin() * (10. * p.z).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}