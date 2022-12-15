use glam::DVec3;

use crate::{hittable::HitRecord, ray::Ray};

use super::lambertian::Lambertian;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
}

pub struct ScatterRecord {
    pub attenuation: DVec3,
    pub ray: Ray,
}

pub trait Scatterable {
    /// Returns None if the ray is absorbed and not scattered
    fn scatter(&self, hit_record: &HitRecord) -> Option<ScatterRecord>;
}

impl Scatterable for Material {
    fn scatter(&self, hit_record: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Material::Lambertian(mat) => mat.scatter(hit_record),
        }
    }
}
