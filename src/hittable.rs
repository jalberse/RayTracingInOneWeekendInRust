use std::{ops::Neg, rc::Rc};

use glam::DVec3;

use crate::{aabb::Aabb, materials::material::Material, ray::Ray};

pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    /// Texture u coordiante
    pub u: f64,
    /// Texture v coordinate
    pub v: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        outward_normal: DVec3,
        t: f64,
        u: f64,
        v: f64,
        material: Rc<dyn Material>,
    ) -> HitRecord {
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
            u,
            v,
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
    /// these values have no effect on the bounding box.
    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<Aabb>;
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

    /// Returns the bounding box encompassing all objects in the HittableList.
    /// Returns None if any object in the list does not have a bounding box (because
    /// it is e.g. an infinite plane)
    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output_box_maybe: Option<Aabb> = None;
        for object in self.objects.iter() {
            if let Some(object_bb) = object.bounding_box(time_0, time_1) {
                // Extend the list's bounding box to include this object
                output_box_maybe = Aabb::union(&output_box_maybe, &Some(object_bb));
            } else {
                // If any object can't be bound, the list can't be bound
                return None;
            }
        }
        output_box_maybe
    }
}
