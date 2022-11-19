use ray_tracing::degrees_to_radians;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        vertical_fov: f64,
        aperture: f64,
        focus_dist: f64,
        aspect_ratio: f64,
        viewport_height: f64,
    ) -> Self {
        let theta = degrees_to_radians(vertical_fov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = viewport_height * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = Vec3::cross(&up, &w).unit();
        let v = Vec3::cross(&w, &u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = look_from - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Camera {
            origin: look_from, lower_left_corner, horizontal,
            vertical, u, v, lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset, self.lower_left_corner
            + s * self.horizontal + t * self.vertical - self.origin - offset
        )
    }
}