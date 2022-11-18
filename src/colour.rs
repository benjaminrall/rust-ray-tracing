use image::Rgb;
use ray_tracing::clamp;
use crate::vec3::Vec3;

pub fn get_colour(colour: &Vec3, samples_per_pixel: i32) -> Rgb<u8> {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = f64::sqrt(scale * colour.x);
    let g = f64::sqrt(scale * colour.y);
    let b = f64::sqrt(scale * colour.z);

    Rgb([
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    ])
}