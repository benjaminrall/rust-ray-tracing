// Internal module declaration
mod vec3;
mod ray;
mod camera;
mod hit_record;
mod hittable;
mod hittable_list;
mod aabb;
mod bvh_node;
mod sphere;
mod moving_sphere;
mod material;
mod lambertian;
mod metal;
mod dielectric;
mod texture;
mod solid_colour;
mod checker_texture;
mod perlin;
mod noise_texture;
mod image_texture;


// Importing own crate's module behaviour
use ray_tracing::*;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::camera::Camera;

use crate::hittable::HittableTrait;
use crate::hittable_list::HittableList;
use crate::bvh_node::BVHNode;
use crate::aabb::AABB;
use crate::sphere::Sphere;
use crate::moving_sphere::MovingSphere;

use crate::material::MaterialTrait;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::dielectric::Dielectric;

// Importing other crates
use std::sync::Arc;
use rayon::prelude::*;
use indicatif::{ ProgressBar, ProgressStyle };
use image::{ ImageBuffer, Rgb };
use crate::checker_texture::CheckerTexture;
use crate::image_texture::ImageTexture;
use crate::noise_texture::{NoiseTexture, NoiseType};

/// Generates the final scene from 'Ray Tracing in a Weekend'
fn in_a_weekend_scene() -> HittableList {
    // Creates world list
    let mut world = HittableList::new();

    // Sets up ground
    let ground_material = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(Vec3::new(0., -1000., 0.), 1000., Arc::clone(&ground_material)));

    // Generates random spheres
    for a in -11..=11 {
       for b in -11..=11 {
           let choose_mat = random_double();
           let centre = Vec3::new(
               a as f64 + 0.9 * random_double(),
               0.2,
               b as f64 + 0.9 * random_double()
           );
           if (centre - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
               if choose_mat < 0.8 {
                   let albedo = Vec3::random(0., 1.) * Vec3::random(0., 1.);
                   let sphere_material = Arc::new(Lambertian::new(albedo));
                   world.add(Sphere::new(centre, 0.2, Arc::clone(&sphere_material)));
               } else if choose_mat < 0.95 {
                   let albedo = Vec3::random(0.5, 1.);
                   let fuzz = random_range(0.0, 0.5);
                   let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                   world.add(Sphere::new(centre, 0.2, Arc::clone(&sphere_material)));
               } else {
                   let sphere_material = Arc::new(Dielectric::new(1.5));
                   world.add(Sphere::new(centre, 0.2, Arc::clone(&sphere_material)));
               }
           }
        }
    }

    // Adds three main spheres
    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.));
    world.add(Sphere::new(Vec3::new(0., 1., 0.), 1., Arc::clone(&material1)));
    world.add(Sphere::new(Vec3::new(-4., 1., 0.), 1., Arc::clone(&material2)));
    world.add(Sphere::new(Vec3::new(4., 1., 0.), 1., Arc::clone(&material3)));


    HittableList::from_objects(vec![BVHNode::from_hittable_list(&world, 0., 1.)])
}

/// Generates the camera for the 'Ray Tracing in a Weekend' scene
fn in_a_weekend_camera(aspect_ratio: f64) -> Camera {
    let look_from = Vec3::new(13., 2., 3.);
    let look_at = Vec3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;

    Camera::new(
        look_from, look_at, up, 20., aperture, dist_to_focus, aspect_ratio, 2., 0., 0.
    )
}

/// Generates a scene with two checkered spheres
fn two_spheres_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)));
    let material = Arc::new(Lambertian::from_texture(Arc::clone(&checker)));
    world.add(Sphere::new(Vec3::new(0., -10., 0.), 10., Arc::clone(&material)));
    world.add(Sphere::new(Vec3::new(0., 10., 0.), 10., Arc::clone(&material)));

    world
}

/// Generates the camera for the two spheres scene
fn two_spheres_camera(aspect_ratio: f64) -> Camera {
    let look_from = Vec3::new(13., 2., 3.);
    let look_at = Vec3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;

    Camera::new(
        look_from, look_at, up, 20., aperture, dist_to_focus, aspect_ratio, 2., 0., 0.
    )
}

/// Generates a scene with two perlin noise spheres
fn two_perlin_spheres_scene() -> HittableList {
    let mut world = HittableList::new();

    let perlin_texture = Arc::new(NoiseTexture::new(4., NoiseType::Marbled));
    let material = Arc::new(Lambertian::from_texture(
        Arc::clone(&perlin_texture)));
    world.add(Sphere::new(Vec3::new(0., -1000., 0.), 1000., Arc::clone(&material)));
    world.add(Sphere::new(Vec3::new(0., 2., 0.), 2., Arc::clone(&material)));

    world
}

/// Generates the camera for the two perlin spheres scene
fn two_perlin_spheres_camera(aspect_ratio: f64) -> Camera {
    let look_from = Vec3::new(13., 2., 3.);
    let look_at = Vec3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;

    Camera::new(
        look_from, look_at, up, 20., aperture, dist_to_focus, aspect_ratio, 2., 0., 0.
    )
}

/// Generates scene with a sphere using an earth image texture
fn earth_scene() -> HittableList {
    let mut world = HittableList::new();

    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_material = Arc::new(Lambertian::from_texture(Arc::clone(&earth_texture)));
    world.add(Sphere::new(Vec3::zero(), 2., Arc::clone(&earth_material)));

    world
}

/// Generates the camera for the earth scene
fn earth_camera(aspect_ratio: f64) -> Camera {
    let look_from = Vec3::new(13., 2., 3.);
    let look_at = Vec3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;

    Camera::new(
        look_from, look_at, up, 20., aperture, dist_to_focus, aspect_ratio, 2., 0., 0.
    )
}

/// Current working scene
fn working_scene() -> HittableList {
    // Creates world list
    let mut world = HittableList::new();

    // Sets up ground
    let texture = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)));
    let ground_material = Arc::new(Lambertian::from_texture(Arc::clone(&texture)));
    world.add(Sphere::new(Vec3::new(0., -1000., 0.), 1000., Arc::clone(&ground_material)));

    // Generates random spheres
    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat = random_double();
            let centre = Vec3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double()
            );
            if (centre - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random(0., 1.) * Vec3::random(0., 1.);
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let centre2 = centre + Vec3::new(0., random_range(0., 0.5), 0.);
                    world.add(MovingSphere::new(centre, centre2, 0., 1., 0.2, Arc::clone(&sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.5, 1.);
                    let fuzz = random_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Sphere::new(centre, 0.2, Arc::clone(&sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Sphere::new(centre, 0.2, Arc::clone(&sphere_material)));
                }
            }
        }
    }

    // Adds three main spheres
    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.));
    world.add(Sphere::new(Vec3::new(0., 1., 0.), 1., Arc::clone(&material1)));
    world.add(Sphere::new(Vec3::new(-4., 1., 0.), 1., Arc::clone(&material2)));
    world.add(Sphere::new(Vec3::new(4., 1., 0.), 1., Arc::clone(&material3)));

    HittableList::from_objects(vec![BVHNode::from_hittable_list(&world, 0., 1.)])
}

/// Current working camera
fn working_camera(aspect_ratio: f64) -> Camera {
    let look_from = Vec3::new(13., 2., 3.);
    let look_at = Vec3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;

    Camera::new(
        look_from, look_at, up, 20., aperture, dist_to_focus, aspect_ratio, 2., 0., 1.
    )
}

/// Gets the colour of a given ray in the world
fn ray_colour(ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    // Stops recursion once past the max depth
    if depth <= 0 {
        return Vec3::zero();
    }

    // Checks if the ray hit anything in the world
    if let Some(hit_record) = world.hit(ray, 0.001, INFINITY) {
        // If the ray hit anything, return the result of the scattered ray against the
        // hit's material

        return match hit_record.material.scatter(ray, &hit_record) {
            Some((attenuation, scattered)) =>
                attenuation * ray_colour(&scattered, world, depth - 1),
            None => Vec3::zero()
        }
    }

    // If nothing was hit, return the sky gradient for that point
    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.);
    (1. - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.)
}

fn main() {
    // ---- IMAGE SETUP ----
    const ASPECT_RATIO: f64 = 3./2.;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // ---- WORLD SETUP ----
    const WORLD_TYPE: usize = 4;
    let world = match WORLD_TYPE {
        1 => in_a_weekend_scene(),
        2 => two_spheres_scene(),
        3 => two_perlin_spheres_scene(),
        4 => earth_scene(),
        _ => working_scene(),
    };

    // ---- CAMERA SETUP ----
    let camera = match WORLD_TYPE {
        1 => working_camera(ASPECT_RATIO),
        2 => two_spheres_camera(ASPECT_RATIO),
        3 => two_perlin_spheres_camera(ASPECT_RATIO),
        4 => earth_camera(ASPECT_RATIO),
        _ => working_camera(ASPECT_RATIO),
    };

    // ---- RENDERING THE SCENE ----

    // Sets up progress bar
    let progress_bar_style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({eta})")
        .unwrap()
        .progress_chars("#>-");
    let render_progress_bar = ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT) as u64);
    render_progress_bar.set_style(progress_bar_style.clone());

    // Create pixels array to store an RGB value for each pixel in the image
    let mut pixels = vec![Rgb([0 as u8, 0 as u8, 0 as u8]); IMAGE_HEIGHT * IMAGE_WIDTH];

    // Loop through each pixel and calculate their colour in parallel
    pixels.par_iter_mut().enumerate().for_each(|(i, pixel)| {
        let mut pixel_colour = Vec3::zero();

        // Get pixel's x and y position from index
        let y = IMAGE_HEIGHT - 1 - (i / IMAGE_WIDTH);
        let x = i % IMAGE_WIDTH;

        // Calculate the colour at each sample
        for _ in 0..SAMPLES_PER_PIXEL {
            let u = (x as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
            let v = (y as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
            let r = camera.get_ray(u, v);
            pixel_colour += ray_colour(&r, &world, MAX_DEPTH);
        }

        // Averages pixel colour over all samples
        pixel_colour /= SAMPLES_PER_PIXEL as f64;

        // Saves the gamma corrected colour value in the range [0,255] to the pixel array
        *pixel = Rgb([
            (256. * clamp(f64::sqrt(pixel_colour.x), 0., 0.999)) as u8,
            (256. * clamp(f64::sqrt(pixel_colour.y), 0., 0.999)) as u8,
            (256. * clamp(f64::sqrt(pixel_colour.z), 0., 0.999)) as u8,
        ]);

        // Increment progress bar
        render_progress_bar.inc(1);
    });

    // ---- SAVING THE SCENE TO IMAGE ----

    // Sets up progress bar
    let draw_progress_bar = ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT) as u64);
    draw_progress_bar.set_style(progress_bar_style);

    // Loop through each pixel in a new image buffer, and set it to its colour from the pixel array
    let mut buffer = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        *pixel = pixels[(y * IMAGE_WIDTH as u32 + x) as usize];
        draw_progress_bar.inc(1);
    }

    // Attempt to save to 'image.png' file
    match buffer.save("image.png") {
        Err(e) => eprintln!("Error writing to file: {}", e),
        Ok(()) => eprintln!("Image saved successfully.")
    }
}
