use ray_tracing::random_double;
use crate::HitRecord;
use crate::Ray;
use crate::Vec3;

#[derive(Debug)]
pub enum MaterialType {
    None,
    Lambertian(Vec3),
    Metal(Vec3, f64),
    Dielectric(f64),
}

impl MaterialType {
    pub fn scatter(self: &MaterialType, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match self {
            MaterialType::None => false,
            MaterialType::Lambertian(albedo) => {

                let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

                if scatter_direction.near_zero() {
                    scatter_direction = hit_record.normal;
                };

                scattered.set(hit_record.point, scatter_direction);
                attenuation.copy_vector(albedo);
                true
            },
            MaterialType::Metal(albedo, fuzz) => {
                let reflected = Vec3::reflect(ray_in.get_direction().unit(), hit_record.normal);
                scattered.set(hit_record.point, reflected + *fuzz * Vec3::random_in_unit_sphere());
                attenuation.copy_vector(albedo);
                Vec3::dot(&scattered.get_direction(), &hit_record.normal) > 0.0
            },
            MaterialType::Dielectric(ir) => {
                attenuation.copy_vector(&Vec3::one());
                let refraction_ratio = if hit_record.front_face { 1.0 / *ir } else { *ir };

                let unit_direction = ray_in.get_direction().unit();
                let cos_theta = f64::min(Vec3::dot(&-unit_direction, &hit_record.normal), 1.0);
                let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

                let direction ;

                if refraction_ratio * sin_theta > 1.0 || reflectance(cos_theta, refraction_ratio)
                        > random_double(0.0, 1.0) {
                    direction = Vec3::reflect(unit_direction, hit_record.normal);
                } else {
                    direction = Vec3::refract(unit_direction, hit_record.normal, refraction_ratio);
                }


                scattered.set(hit_record.point, direction);
                true
            },
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
}