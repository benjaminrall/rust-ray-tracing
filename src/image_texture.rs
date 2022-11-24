use image::{DynamicImage, GenericImageView, ImageResult};
use ray_tracing::clamp;
use crate::texture::{Texture, TextureTrait};
use crate::vec3::Vec3;

const COLOR_SCALE: f64 = 1. / 255.;

#[derive(Debug)]
pub struct ImageTexture {
    data: Option<DynamicImage>,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn new(path: &str) -> Texture {
        let width;
        let height;
        let data = match image::open(path) {
            Ok(i) => {
                width = i.width();
                height = i.height();
                println!("LOADED IMAGE {width} {height}");
                Some(i)
            },
            Err(_) => {
                width = 0;
                height = 0;
                None
            }
        };

        Texture::ImageTexture(ImageTexture { data, width, height })
    }
}

impl TextureTrait for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        if let Some(data) = &self.data {
            let u = clamp(u, 0., 1.);
            let v = 1. - clamp(v, 0., 1.);

            let mut i = (u * self.width as f64) as u32;
            let mut j = (v * self.height as f64) as u32;

            if i >= self.width { i = self.width - 1 }
            if j >= self.height { j = self.height - 1 }

            let pixel = data.get_pixel(i, j);

            Vec3::new(
                COLOR_SCALE * pixel[0] as f64,
                COLOR_SCALE * pixel[1] as f64,
                COLOR_SCALE * pixel[2] as f64
            )
        } else {
            Vec3::new(0., 1., 1.)
        }
    }
}