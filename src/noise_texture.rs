use crate::perlin::Perlin;
use crate::texture::{Texture, TextureTrait};
use crate::vec3::Vec3;

#[derive(Debug)]
pub enum NoiseType {
    Random,
    Turbulence,
    Marbled
}

#[derive(Debug)]
pub struct NoiseTexture {
    noise: Perlin,
    noise_type: NoiseType,
    scale: f64
}

impl NoiseTexture {
    pub fn new(scale: f64, noise_type: NoiseType) -> Texture {
        Texture::NoiseTexture(NoiseTexture { noise: Perlin::new(), noise_type, scale })
    }
}

impl TextureTrait for NoiseTexture {
    fn value(&self, _: f64, _: f64, p: &Vec3) -> Vec3 {
        match self.noise_type {
            NoiseType::Random =>
                Vec3::one() * 0.5 * (1. + self.noise.noise(&(self.scale * *p))),
            NoiseType::Turbulence =>
                Vec3::one() * self.noise.turbulence(&(self.scale * *p), 7),
            NoiseType::Marbled =>
                Vec3::one() * 0.5 * (1. + (self.scale * p.z + 10. * self.noise.turbulence(p, 7)).sin())
        }
    }
}