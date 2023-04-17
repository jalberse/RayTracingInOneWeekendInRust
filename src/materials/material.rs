use glam::{Vec3, vec3};

use crate::{hittable::HitRecord, ray::Ray};

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub ray: Ray,
}

impl ScatterRecord {
    pub fn new(attenuation: Vec3, ray: Ray) -> ScatterRecord {
        ScatterRecord { attenuation, ray }
    }
}

pub trait Material: Send + Sync {
    /// Returns None if the ray is absorbed and not scattered
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;

    fn emit(&self, _u: f32, _v: f32, _point: &Vec3) -> Vec3 {
        vec3(0.0, 0.0, 0.0)
    }
}
