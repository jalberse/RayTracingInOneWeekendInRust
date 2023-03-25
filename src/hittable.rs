use std::{ops::Neg, sync::Arc};

use glam::DVec3;
use rand::Rng;

use crate::{
    aabb::Aabb, materials::isotropic::Isotropic, materials::material::Material, ray::Ray,
    textures::texture::Texture,
};

pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    /// Texture u coordiante
    pub u: f64,
    /// Texture v coordinate
    pub v: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        outward_normal: DVec3,
        t: f64,
        u: f64,
        v: f64,
        material: Arc<dyn Material>,
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

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: DVec3) {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
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
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
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

/// A volume with constant density.
/// The boundary must be convex.
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(
        boundary: Arc<dyn Hittable>,
        density: f64,
        texture: Arc<dyn Texture>,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary,
            phase_function: Arc::new(Isotropic::new(texture)),
            neg_inv_density: -1.0 / density,
        }
    }

    pub fn new_with_color(
        boundary: Arc<dyn Hittable>,
        density: f64,
        color: DVec3,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary,
            phase_function: Arc::new(Isotropic::from_color(color)),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit1 = self.boundary.hit(ray, f64::NEG_INFINITY, f64::INFINITY)?;
        let mut hit2 = self.boundary.hit(ray, hit1.t + 0.0001, f64::INFINITY)?;

        if hit1.t < t_min {
            hit1.t = t_min
        }
        if hit2.t > t_max {
            hit2.t = t_max
        }

        if hit1.t >= hit2.t {
            return None;
        }

        if hit1.t < 0.0 {
            hit1.t = 0.0
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
        let mut rng = rand::thread_rng();
        let hit_distance = self.neg_inv_density * f64::ln(rng.gen());

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let out_rec_time = hit1.t + hit_distance / ray_length;
        let out_rec_point = ray.at(out_rec_time);

        let out_hit_record = HitRecord {
            point: out_rec_point,
            normal: DVec3::X, // Arbitrary
            t: out_rec_time,
            // Using either the first or second hit record's UVs doesn't make physical sense, nor
            // would using an interpolated U/V - we are working with volumes, not on the boundary.
            // We expect the material/texture to determine attenuation via the HitRecord.point.
            // So, we use 0 here.
            u: 0.0,
            v: 0.0,
            front_face: true, // Arbitrary
            material: self.phase_function.clone(),
        };

        Some(out_hit_record)
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time_0, time_1)
    }
}
