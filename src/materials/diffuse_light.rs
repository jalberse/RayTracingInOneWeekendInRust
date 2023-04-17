use std::sync::Arc;

use glam::Vec3;

use crate::textures::{solid_color::SolidColor, texture::Texture};

use super::material::Material;

pub struct DiffuseLight {
    emission_texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emission_texture: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { emission_texture }
    }

    pub fn from_color(color: Vec3) -> DiffuseLight {
        DiffuseLight {
            emission_texture: Arc::new(SolidColor::new(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _ray: &crate::ray::Ray,
        _hit_record: &crate::hittable::HitRecord,
    ) -> Option<super::material::ScatterRecord> {
        None
    }

    fn emit(&self, u: f32, v: f32, point: &Vec3) -> Vec3 {
        self.emission_texture.value(u, v, point)
    }
}
