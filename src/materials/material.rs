use enum_dispatch::enum_dispatch;
use glam::DVec3;

use crate::{hittable::HitRecord, ray::Ray};

use super::dialectric::Dialectric;
use super::lambertian::Lambertian;
use super::metal::Metal;

pub struct ScatterRecord {
    pub attenuation: DVec3,
    pub ray: Ray,
}

#[enum_dispatch]
pub enum MaterialEnum {
    Lambertian,
    Metal,
    Dialectric,
}

#[enum_dispatch(MaterialEnum)]
pub trait Material: Send + Sync {
    /// Returns None if the ray is absorbed and not scattered
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}
