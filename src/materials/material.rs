use glam::DVec3;

use crate::{hittable::HitRecord, ray::Ray};

pub struct ScatterRecord {
    pub attenuation: DVec3,
    pub ray: Ray,
}

pub trait Material {
    /// Returns None if the ray is absorbed and not scattered
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}
