use std::ops::Neg;

use glam::Vec3;

use crate::ray::Ray;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, outward_normal: Vec3, t: f32) -> HitRecord {
        let point = ray.at(t);
        let front_face = ray.direction.dot(outward_normal).is_sign_negative();
        let normal = if front_face {
            outward_normal
        } else {
            outward_normal.neg()
        };
        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
    }
}

pub trait hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
