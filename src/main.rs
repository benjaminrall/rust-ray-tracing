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
mod aa_rect;
mod aa_box;
mod translate;
mod rotate_y;
mod constant_medium;
mod material;
mod lambertian;
mod metal;
mod dielectric;
mod diffuse_light;
mod isotropic;
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
use crate::aa_rect::{XYRect, XZRect, YZRect};
use crate::aa_box::AABox;
use crate::translate::Translate;
use crate::rotate_y::RotateY;
use crate::constant_medium::ConstantMedium;
use crate::material::MaterialTrait;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::dielectric::Dielectric;
use crate::diffuse_light::DiffuseLight;
use crate::isotropic::Isotropic;
use crate::texture::TextureTrait;
use crate::solid_colour::SolidColour;
use crate::checker_texture::CheckerTexture;
use crate::image_texture::ImageTexture;
use crate::noise_texture::{NoiseTexture, NoiseType};

// Importing other crates
use std::sync::Arc;
use rayon::prelude::*;
use indicatif::{ ProgressBar, ProgressStyle };
use image::{ ImageBuffer, Rgb };

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
fn in_a_weekend_camera(aspect_ratio: f64, background: &mut Vec3) -> Camera {
    let look_from = Vec3::new(13., 2., 3.);
    let look_at = Vec3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    *background =Vec3::new(0.7, 0.8, 1.);

    Camera::new(
        look_from, look_at, up, 20., aperture, dist_to_focus, aspect_ratio, 2., 0., 0.
    )
}

/// Generates scene with bouncing balls and a checkered texture
fn bouncing_balls_scene() -> HittableList {
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

/// Generates camera for bouncing balls scene
fn bouncing_balls_camera(aspect_ratio: f64, background: &mut Vec3) -> Camera {
    let look_from = Vec3::new(13., 2., 3.);
    let look_at = Vec3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    *background = Vec3::new(0.7, 0.8, 1.);

    Camera::new(
        look_from, look_at, up, 20., aperture, dist_to_focus, aspect_ratio, 2., 0., 1.
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
fn two_spheres_camera(aspect_ratio: f64, background: &mut Vec3) -> Camera {
    let look_from = Vec3::new(13., 2., 3.);
    let look_at = Vec3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    *background =Vec3::new(0.7, 0.8, 1.);

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
fn two_perlin_spheres_camera(aspect_ratio: f64, background: &mut Vec3) -> Camera {
    let look_from = Vec3::new(13., 2., 3.);
    let look_at = Vec3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    *background = Vec3::new(0.7, 0.8, 1.);

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
fn earth_camera(aspect_ratio: f64, background: &mut Vec3) -> Camera {
    let look_from = Vec3::new(13., 2., 3.);
    let look_at = Vec3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    *background =Vec3::new(0.7, 0.8, 1.);

    Camera::new(
        look_from, look_at, up, 20., aperture, dist_to_focus, aspect_ratio, 2., 0., 0.
    )
}

/// Generates scene with a simple light
fn simple_light_scene() -> HittableList {
    let mut world = HittableList::new();

    let perlin_texture = Arc::new(NoiseTexture::new(4., NoiseType::Marbled));
    let material = Arc::new(Lambertian::from_texture(
        Arc::clone(&perlin_texture)));
    world.add(Sphere::new(Vec3::new(0., -1000., 0.), 1000., Arc::clone(&material)));
    world.add(Sphere::new(Vec3::new(0., 2., 0.), 2., Arc::clone(&material)));

    let light_mat = Arc::new(DiffuseLight::from_colour(4., 4., 4.));
    world.add(XYRect::new(3., 5., 1., 3., -2., Arc::clone(&light_mat)));
    world.add(Sphere::new(Vec3::new(0., 6.5, 0.), 2., Arc::clone(&light_mat)));

    world
}

/// Generates the camera for the simple light scene
fn simple_light_camera(aspect_ratio: f64, background: &mut Vec3) -> Camera {
    let look_from = Vec3::new(26., 3., 6.);
    let look_at = Vec3::new(0., 2., 0.);
    let up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    *background = Vec3::zero();

    Camera::new(
        look_from, look_at, up, 20., aperture, dist_to_focus, aspect_ratio, 2., 0., 0.
    )
}

/// Generates Cornell Box scene
fn cornell_box_scene() -> HittableList {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_colour(15., 15., 15.));

    world.add(YZRect::new(0., 555., 0., 555., 555., Arc::clone(&green)));
    world.add(YZRect::new(0., 555., 0., 555., 0., Arc::clone(&red)));
    world.add(XZRect::new(213., 343., 227., 332., 554., Arc::clone(&light)));
    world.add(XZRect::new(0., 555., 0., 555., 0., Arc::clone(&white)));
    world.add(XZRect::new(0., 555., 0., 555., 555., Arc::clone(&white)));
    world.add(XYRect::new(0., 555., 0., 555., 555., Arc::clone(&white)));

    let box1 = Arc::new(AABox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 330., 165.), Arc::clone(&white)));
    let box1 = Arc::new(RotateY::new(Arc::clone(&box1), 15.));
    world.add(Translate::new(Arc::clone(&box1), Vec3::new(265., 0., 295.)));

    let box2 = Arc::new(AABox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 165., 165.), Arc::clone(&white)));
    let box2 = Arc::new(RotateY::new(Arc::clone(&box2), -18.));
    world.add(Translate::new(Arc::clone(&box2), Vec3::new(130., 0., 65.)));

    world
}

/// Generates the camera for the Cornell Box scene
fn cornell_box_camera(aspect_ratio: f64, background: &mut Vec3) -> Camera {
    let look_from = Vec3::new(278., 278., -800.);
    let look_at = Vec3::new(278., 278., 0.);
    let up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    *background = Vec3::zero();

    Camera::new(
        look_from, look_at, up, 40., aperture, dist_to_focus, aspect_ratio, 2., 0., 0.
    )
}

/// Generates Cornell Box scene with fog
fn cornell_box_smoke_scene() -> HittableList {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_colour(7., 7., 7.));

    world.add(YZRect::new(0., 555., 0., 555., 555., Arc::clone(&green)));
    world.add(YZRect::new(0., 555., 0., 555., 0., Arc::clone(&red)));
    world.add(XZRect::new(113., 443., 127., 432., 554., Arc::clone(&light)));
    world.add(XZRect::new(0., 555., 0., 555., 0., Arc::clone(&white)));
    world.add(XZRect::new(0., 555., 0., 555., 555., Arc::clone(&white)));
    world.add(XYRect::new(0., 555., 0., 555., 555., Arc::clone(&white)));

    let box1 = Arc::new(AABox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 330., 165.), Arc::clone(&white)));
    let box1 = Arc::new(RotateY::new(Arc::clone(&box1), 15.));
    let box1 = Arc::new(Translate::new(Arc::clone(&box1), Vec3::new(265., 0., 295.)));

    let box2 = Arc::new(AABox::new(Vec3::new(0., 0., 0.), Vec3::new(165., 165., 165.), Arc::clone(&white)));
    let box2 = Arc::new(RotateY::new(Arc::clone(&box2), -18.));
    let box2 = Arc::new(Translate::new(Arc::clone(&box2), Vec3::new(130., 0., 65.)));

    world.add(ConstantMedium::from_colour(Arc::clone(&box1), 0.01, Vec3::zero()));
    world.add(ConstantMedium::from_colour(Arc::clone(&box2), 0.01, Vec3::one()));

    world
}

/// Generates the camera for the Cornell Box scene
fn cornell_box_smoke_camera(aspect_ratio: f64, background: &mut Vec3) -> Camera {
    cornell_box_camera(aspect_ratio, background)
}

/// Generates the final scene of 'Ray Tracing The Next Week'
fn the_next_week_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground = Arc::new(Lambertian::new(Vec3::new(0.48, 0.83, 0.53)));

    let mut boxes1 = HittableList::new();
    const BOXES_PER_SIDE: i32 = 20;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.;
            let x0 = -1000. + i as f64 * w;
            let y0 = 0.;
            let z0 = -1000. + j as f64 * w;
            let x1 = x0 + w;
            let y1 = random_range(1., 101.);
            let z1 = z0 + w;

            boxes1.add(AABox::new(
                Vec3::new(x0, y0, z0), Vec3::new(x1, y1, z1),
                Arc::clone(&ground))
            );
        }
    }

    world.add(BVHNode::from_hittable_list(&boxes1, 0., 1.));

    let light = Arc::new(DiffuseLight::from_colour(7., 7., 7.));
    world.add(XZRect::new(123., 423., 147., 412., 554., Arc::clone(&light)));

    let centre1 = Vec3::new(400., 400., 200.);
    let centre2 = centre1 + Vec3::new(30., 0., 0.);
    let moving_sphere_mat = Arc::new(Lambertian::new(Vec3::new(0.7, 0.3, 0.1)));
    world.add(MovingSphere::new(centre1, centre2, 0., 1., 50., Arc::clone(&moving_sphere_mat)));

    let glass = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Vec3::new(260., 150., 45.), 50., Arc::clone(&glass)));
    world.add(Sphere::new(Vec3::new(0., 150., 145.), 50., Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.))));

    let boundary = Arc::new(Sphere::new(Vec3::new(360., 150., 145.), 70., Arc::clone(&glass)));
    world.add_arc(Arc::clone(&boundary));
    world.add(ConstantMedium::from_colour(Arc::clone(&boundary), 0.2, Vec3::new(0.2, 0.4, 0.9)));
    let boundary = Arc::new(Sphere::new(Vec3::new(0., 0., 0.), 5000., Arc::clone(&glass)));
    world.add(ConstantMedium::from_colour(Arc::clone(&boundary), 0.0001, Vec3::one()));

    let earth_mat = Arc::new(Lambertian::from_texture(Arc::new(ImageTexture::new("earthmap.jpg"))));
    world.add(Sphere::new(Vec3::new(400., 200., 400.), 100., Arc::clone(&earth_mat)));
    let noise_mat = Arc::new(Lambertian::from_texture( Arc::new(NoiseTexture::new(0.1, NoiseType::Marbled))));
    world.add(Sphere::new(Vec3::new(220., 280., 300.), 80., Arc::clone(&noise_mat)));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    for _ in 0..1000 {
        boxes2.add(Sphere::new(Vec3::random(0., 165.), 10., Arc::clone(&white)));
    }

    world.add(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BVHNode::from_hittable_list(&boxes2, 0., 1.)), 15.
        )),
        Vec3::new(-100., 270., 395.))
    );

    HittableList::from_objects(vec![BVHNode::from_hittable_list(&world, 0., 1.)])
}

/// Generates the camera for the 'Ray Tracing The Next Week' scene
fn the_next_week_camera(aspect_ratio: f64, background: &mut Vec3) -> Camera {
    let look_from = Vec3::new(478., 278., -600.);
    let look_at = Vec3::new(278., 278., 0.);
    let up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    *background = Vec3::zero();

    Camera::new(
        look_from, look_at, up, 40., aperture, dist_to_focus, aspect_ratio, 2., 0., 1.
    )
}

/// Gets the colour of a given ray in the world
fn ray_colour(ray: &Ray, background: Vec3, world: &HittableList, depth: i32) -> Vec3 {
    // Stops recursion once past the max depth
    if depth <= 0 {
        return Vec3::zero();
    }

    // Checks if the ray hit anything in the world
    if let Some(hit_record) = world.hit(ray, 0.001, INFINITY) {
        let emitted = hit_record.material.emitted(hit_record.u, hit_record.v, &hit_record.point);

        // If the ray hit anything, return the result of the scattered ray against the
        // hit's material
        match hit_record.material.scatter(ray, &hit_record) {
            Some((attenuation, scattered)) =>
                emitted + attenuation * ray_colour(&scattered, background, world, depth - 1),
            None => emitted
        }
    } else {
        background
    }
}

fn main() {
    // ---- IMAGE SETUP ----
    const ASPECT_RATIO: f64 = 1.;
    const IMAGE_WIDTH: usize = 800;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: i32 = 20000;
    const MAX_DEPTH: i32 = 50;

    // ---- WORLD SETUP ----
    const WORLD_TYPE: usize = 0;
    let world = match WORLD_TYPE {
        1 => in_a_weekend_scene(),
        2 => bouncing_balls_scene(),
        3 => two_spheres_scene(),
        4 => two_perlin_spheres_scene(),
        5 => earth_scene(),
        6 => simple_light_scene(),
        7 => cornell_box_scene(),
        8 => cornell_box_smoke_scene(),
        _ => the_next_week_scene(),
    };

    // ---- CAMERA SETUP ----
    let mut background = Vec3::zero();
    let camera = match WORLD_TYPE {
        1 => in_a_weekend_camera(ASPECT_RATIO, &mut background),
        2 => bouncing_balls_camera(ASPECT_RATIO, &mut background),
        3 => two_spheres_camera(ASPECT_RATIO, &mut background),
        4 => two_perlin_spheres_camera(ASPECT_RATIO, &mut background),
        5 => earth_camera(ASPECT_RATIO, &mut background),
        6 => simple_light_camera(ASPECT_RATIO, &mut background),
        7 => cornell_box_camera(ASPECT_RATIO, &mut background),
        8 => cornell_box_smoke_camera(ASPECT_RATIO, &mut background),
        _ => the_next_week_camera(ASPECT_RATIO, &mut background),
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
            pixel_colour += ray_colour(&r, background, &world, MAX_DEPTH);
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
