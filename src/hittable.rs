use std::ops::Neg;

use glam::DVec3;

use crate::{materials::material::Material, ray::Ray};

pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new(ray: &Ray, outward_normal: DVec3, t: f64, material: Material) -> HitRecord {
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
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList<T>
where
    T: Hittable,
{
    objects: Vec<Box<T>>,
}

impl<T> HittableList<T>
where
    T: Hittable,
{
    #[allow(dead_code)]
    pub fn new() -> HittableList<T> {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn from_vec(objects: Vec<Box<T>>) -> HittableList<T> {
        HittableList { objects }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, object: Box<T>) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.objects
            .iter()
            .fold(None, |closest_yet, object| -> Option<HitRecord> {
                let closest_t = if let Some(closest) = &closest_yet {
                    closest.t
                } else {
                    t_max
                };
                if let Some(hit) = object.hit(&ray, t_min, closest_t) {
                    Some(hit)
                } else {
                    closest_yet
                }
            })
    }
}
