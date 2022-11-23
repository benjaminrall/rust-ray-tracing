use crate::checker_texture::CheckerTexture;
use crate::solid_colour::SolidColour;
use crate::vec3::Vec3;

/// Trait implemented by all textures
pub trait TextureTrait {
    /// Calculates value of the texture for some UV surface coordinates
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

#[derive(Debug)]
/// Enum storing each texture variation
pub enum Texture {
    SolidColour(SolidColour),
    CheckerTexture(CheckerTexture),
}

/// Calls methods for materials in the Texture enum
impl TextureTrait for Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        match self {
            Texture::SolidColour(obj) => obj.value(u, v, p),
            Texture::CheckerTexture(obj) => obj.value(u, v, p)
        }
    }
}

