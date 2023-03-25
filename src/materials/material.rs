use glam::{dvec3, DVec3};

use crate::{hittable::HitRecord, ray::Ray};

pub struct ScatterRecord {
    pub attenuation: DVec3,
    pub ray: Ray,
}

impl ScatterRecord {
    pub fn new(attenuation: DVec3, ray: Ray) -> ScatterRecord {
        ScatterRecord { attenuation, ray }
    }
}

pub trait Material: Send + Sync {
    /// Returns None if the ray is absorbed and not scattered
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;

    fn emit(&self, _u: f64, _v: f64, _point: &DVec3) -> DVec3 {
        dvec3(0.0, 0.0, 0.0)
    }
}
