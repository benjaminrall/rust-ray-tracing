use ray_tracing::{degrees_to_radians, random_range};
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Object to represent the Camera in a scene
pub struct Camera {
    origin: Vec3,               // Origin of the camera's rays
    lower_left_corner: Vec3,    // Lower left corner of the camera's image
    horizontal: Vec3,           // Horizontal distance for each image pixel
    vertical: Vec3,             // Vertical distance for each image pixel
    u: Vec3,                    // Direction of relative x axis in camera's plane
    v: Vec3,                    // Direction of relative y axis in camera's plane
    lens_radius: f64,           // Radius of the camera's lens
    min_time: f64,              // Shutter open time
    max_time: f64,              // Shutter close time
}

impl Camera {
    /// Returns a new Camera object
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vertical_up: Vec3,
        vertical_fov: f64,
        aperture: f64,
        focus_dist: f64,
        aspect_ratio: f64,
        viewport_height: f64,
        min_time: f64,
        max_time: f64
    ) -> Self {
        // Calculates viewport width and height given a field of view angle in degrees
        let theta = degrees_to_radians(vertical_fov);
        let h = f64::tan(theta / 2.);
        let viewport_height = viewport_height * h;
        let viewport_width = aspect_ratio * viewport_height;

        // Calculates direction of relative x, y, and z axis in the camera's plane
        let w = (look_from - look_at).unit();
        let u = Vec3::cross(&vertical_up, &w).unit();
        let v = Vec3::cross(&w, &u);

        // Calculates lower left corner of image and the
        // horizontal and vertical component of each pixel
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = look_from - horizontal / 2. - vertical / 2. - focus_dist * w;

        // Returns fully constructed Camera
        Camera {
            origin: look_from, lower_left_corner, horizontal,
            vertical, u, v, lens_radius: aperture / 2., min_time, max_time
        }
    }

    /// Creates a ray which goes from the camera to a pixel at the given (x, y) position
    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        // Calculates a random offset in the camera's relative plane to cause depth of field effect
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        // Returns fully constructed Ray
        Ray::new(
            self.origin + offset, self.lower_left_corner
            + x * self.horizontal + y * self.vertical - self.origin - offset,
            random_range(self.min_time, self.max_time)
        )
    }
}