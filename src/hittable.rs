use std::{ops::Neg, rc::Rc};

use glam::DVec3;

use crate::{aabb::Aabb, materials::material::Material, ray::Ray};

pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(ray: &Ray, outward_normal: DVec3, t: f64, material: Rc<dyn Material>) -> HitRecord {
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

    /// Returns the bounding box of the hittable object. If the object has no bounding box
    /// (because it is an infinite plane, for example), None is returned.
    ///
    /// # Arguments
    ///
    /// * `time_0`, `time_1` - If the object moves, the bounding box will encompass its
    /// full range of motion between `time_0` and `time_1`. If the object does not move,
    /// thes have no effect on the bounding box.
    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<Aabb>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
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
