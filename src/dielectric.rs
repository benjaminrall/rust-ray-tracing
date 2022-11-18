use ray_tracing::random_double;
use ray_tracing::reflectance;
use crate::hit_record::HitRecord;
use crate::materials::{Material, MaterialTrait};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Dielectric {
    albedo: Vec3,
    refraction_index: f64
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Material {
        Material::Dielectric(Dielectric { albedo: Vec3::one(), refraction_index })
    }
}

impl MaterialTrait for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(&Vec3, Ray)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.get_direction().unit();
        let cos_theta = f64::min(Vec3::dot(&-unit_direction, &hit_record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let direction;

        if refraction_ratio * sin_theta > 1.0 || reflectance(cos_theta, refraction_ratio)
            > random_double() {
            direction = Vec3::reflect(unit_direction, hit_record.normal);
        } else {
            direction = Vec3::refract(unit_direction, hit_record.normal, refraction_ratio);
        }

        let scattered = Ray::new(hit_record.point, direction);

        Some((&self.albedo, scattered))
    }
}