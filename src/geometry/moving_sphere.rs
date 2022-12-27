use std::sync::Arc;

use glam::{dvec3, DVec3};

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    materials::material::MaterialEnum,
};

use super::sphere::Sphere;

/// A sphere which moves in a linear fashion from `center_start` at `time_start` to
/// `center_end` at `time_end`. Movement continues outside those those times as well;
/// these fields just define the velocity and position of the sphere via those two points in time.
pub struct MovingSphere {
    center_start: DVec3,
    center_end: DVec3,
    time_start: f64,
    time_end: f64,
    radius: f64,
    pub material: Arc<MaterialEnum>,
}

impl MovingSphere {
    pub fn new(
        center_start: DVec3,
        center_end: DVec3,
        time_start: f64,
        time_end: f64,
        radius: f64,
        material: Arc<MaterialEnum>,
    ) -> MovingSphere {
        MovingSphere {
            center_start,
            center_end,
            time_start,
            time_end,
            radius,
            material,
        }
    }

    fn center(&self, time: f64) -> DVec3 {
        self.center_start
            + ((time - self.time_start) / (self.time_end - self.time_start))
                * (self.center_end - self.center_start)
    }
}

impl Hittable for MovingSphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant.is_sign_negative() {
            return None;
        }
        let sqrt_discriminant = f64::sqrt(discriminant);
        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(root);
        let normal = (point - self.center(ray.time)) / self.radius;
        let (u, v) = Sphere::get_uv(&normal);
        Some(HitRecord::new(&ray, normal, t, u, v, self.material.clone()))
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<Aabb> {
        // Note that this assumes a linear movement from the start and end position;
        // a parametric implementation wouldn't necessarily have its extent bounded like this.
        let rad = dvec3(self.radius, self.radius, self.radius);
        let start_box = Aabb::new(self.center(time_0) - rad, self.center(time_0) + rad);
        let end_box = Aabb::new(self.center(time_0) - rad, self.center(time_1) + rad);
        Aabb::union(&Some(start_box), &Some(end_box))
    }
}
