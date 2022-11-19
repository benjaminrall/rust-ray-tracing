use ray_tracing::random_double;
use crate::hit_record::HitRecord;
use crate::materials::{Material, MaterialTrait};
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Object to represent Dielectric materials such as water and glass
pub struct Dielectric {
    albedo: Vec3,           // Albedo of the dielectric
    refraction_index: f64   // Refraction index of the material
}

impl Dielectric {
    /// Constructs a new Dielectric object, wrapped in the Material enum
    pub fn new(refraction_index: f64) -> Material {
        Material::Dielectric(Dielectric { albedo: Vec3::one(), refraction_index })
    }

    /// Uses Schlick's approximation for calculating reflectance of a dielectric material
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
    }
}

impl MaterialTrait for Dielectric {
    /// Calculates scattered ray and colour for a given hit with a dielectric object
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(&Vec3, Ray)> {
        // Calculates refraction ratio depending on if the front face of the object was hit
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        // Calculates unit direction of the ray in and appropriate trig values
        let unit_direction = ray_in.get_direction().unit();
        let cos_theta = f64::min(Vec3::dot(&-unit_direction, &hit_record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        // Determines if a ray should be reflected or refracted, and gets the resulting direction
        let direction;
        if refraction_ratio * sin_theta > 1.0 || reflectance(cos_theta, refraction_ratio) > random_double() {
            direction = Vec3::reflect(unit_direction, hit_record.normal);
        } else {
            direction = Vec3::refract(unit_direction, hit_record.normal, refraction_ratio);
        }

        // Constructs the scattered ray
        let scattered = Ray::new(hit_record.point, direction);

        Some((&self.albedo, scattered))
    }
}