use crate::vec3::Vec3;

/// Object to store information about a 3D line ray
pub struct Ray {
    origin: Vec3,   // Origin of the ray
    direction: Vec3 // Direction of the ray
}

impl Ray {
    /// Constructs a new ray with a given origin and direction
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    /// Gets the point on the ray at a given time 't'
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    /// Returns the ray's origin
    pub fn get_origin(&self) -> Vec3 {
        self.origin
    }

    /// Returns the ray's direction
    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }

    /// Returns the direction of a reflected ray through a normal vector
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(&v, &n) * n
    }

    /// Returns the direction of a refracted ray
    pub fn refract(uv: Vec3, n: Vec3, refractive_index_ratio: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(&-uv, &n), 1.0);
        let r_out_perpendicular = refractive_index_ratio * (uv + cos_theta * n);
        let r_out_parallel = -f64::sqrt(f64::abs(
            1.0 - r_out_perpendicular.length_squared())) * n;
        r_out_perpendicular + r_out_parallel
    }
}