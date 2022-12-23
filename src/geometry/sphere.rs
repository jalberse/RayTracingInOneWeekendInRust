use std::{f64::consts::PI, ops::Neg, sync::Arc};

use glam::{dvec3, DVec3};

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    materials::material::Material,
    ray::Ray,
};

pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    /// Returns the `(u, v)` coordinates for a `point` on a unit sphere centered at the origin.
    /// * `u` - returned value \[0,1\] of angle around the Y axis from X=-1.
    /// * `v` - returned value \[0,1\] of angle from Y=-1 to Y=+1.
    /// * Examples:
    ///     * <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    ///     * <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    ///     * <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    pub fn get_uv(point: &DVec3) -> (f64, f64) {
        let theta = f64::acos(point.neg().y);
        let phi = f64::atan2(point.neg().z, point.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
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
        let normal = (point - self.center) / self.radius;
        let (u, v) = Sphere::get_uv(&normal);
        Some(HitRecord::new(&ray, normal, t, u, v, self.material.clone()))
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<Aabb> {
        let rad = dvec3(self.radius, self.radius, self.radius);
        let bb = Aabb::new(self.center - rad, self.center + rad);
        Some(bb)
    }
}
