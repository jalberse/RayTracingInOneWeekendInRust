use std::sync::Arc;

use glam::DVec3;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    textures::{solid_color::SolidColor, texture::Texture},
    utils,
};

use super::{
    material::{Material, ScatterRecord},
    utils::random_unit_vector,
};

#[derive(Clone)]
pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo }
    }

    pub fn from_color(albedo: DVec3) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = hit_record.normal + random_unit_vector();
        // Catch degenerate scatter directions
        let scatter_direction = if utils::near_zero(&scatter_direction) {
            hit_record.normal
        } else {
            scatter_direction
        };
        let scattered = Ray::new(hit_record.point, scatter_direction, ray.time);

        let attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, &hit_record.point);
        Some(ScatterRecord {
            ray: scattered,
            attenuation,
        })
    }
}
