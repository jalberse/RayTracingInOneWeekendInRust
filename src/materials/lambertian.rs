use glam::DVec3;

use crate::{hittable::HitRecord, ray::Ray, utils};

use super::{
    material::{Material, ScatterRecord},
    utils::random_unit_vector,
};

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: DVec3,
}

impl Lambertian {
    pub fn new(albedo: DVec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = hit_record.normal + random_unit_vector();
        // Catch degenerate scatter directions
        let scatter_direction = if utils::near_zero(&scatter_direction) {
            hit_record.normal
        } else {
            scatter_direction
        };
        let scattered = Ray::new(hit_record.point, scatter_direction, 0.0);

        let attenuation = self.albedo;
        Some(ScatterRecord {
            ray: scattered,
            attenuation,
        })
    }
}
