use std::sync::Arc;

use glam::DVec3;

use crate::{
    ray::Ray,
    textures::{solid_color::SolidColor, texture::Texture},
};

use super::{
    material::{Material, ScatterRecord},
    utils::random_in_unit_sphere,
};

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Isotropic {
        Isotropic { albedo }
    }

    pub fn from_color(color: DVec3) -> Isotropic {
        Isotropic {
            albedo: Arc::new(SolidColor::new(color)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        ray: &crate::ray::Ray,
        hit_record: &crate::hittable::HitRecord,
    ) -> Option<super::material::ScatterRecord> {
        let scattered = Ray::new(hit_record.point, random_in_unit_sphere(), ray.time);
        let attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, &hit_record.point);
        Some(ScatterRecord::new(attenuation, scattered))
    }
}
