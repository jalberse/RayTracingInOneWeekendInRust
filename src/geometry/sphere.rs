use std::{
    f32::consts::PI,
    ops::Neg,
    sync::{Arc, Mutex},
};

use ahash::AHashMap;
use glam::{vec3, DVec3, Vec3};

use crate::{
    aabb::Aabb,
    bvh::BvhId,
    hittable::{HitRecord, Hittable},
    hrpp::Predictor,
    materials::material::Material,
    ray::Ray,
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Sphere {
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
    pub fn get_uv(point: &Vec3) -> (f32, f32) {
        let theta = f32::acos(point.neg().y);
        let phi = f32::atan2(point.neg().z, point.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        _predictors: &Arc<Option<AHashMap<BvhId, Mutex<Predictor>>>>,
    ) -> Option<HitRecord> {
        let direction = DVec3::new(
            ray.direction.x as f64,
            ray.direction.y as f64,
            ray.direction.z as f64,
        );
        let origin = DVec3::new(
            ray.origin.x as f64,
            ray.origin.y as f64,
            ray.origin.z as f64,
        );
        let center = DVec3::new(
            self.center.x as f64,
            self.center.y as f64,
            self.center.z as f64,
        );
        let radius = self.radius as f64;

        let oc = origin - center;
        let a = direction.length_squared();
        let half_b = oc.dot(direction);
        let c = oc.length_squared() - radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant.is_sign_negative() {
            return None;
        }
        let sqrt_discriminant = f64::sqrt(discriminant);
        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min as f64 || (t_max as f64) < root {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min as f64 || (t_max as f64) < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(root as f32);
        let normal = (point - self.center) / self.radius;
        let (u, v) = Sphere::get_uv(&normal);
        Some(HitRecord::new(
            &ray,
            normal,
            t as f32,
            u,
            v,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _time_0: f32, _time_1: f32) -> Option<Aabb> {
        let rad = vec3(self.radius, self.radius, self.radius);
        let bb = Aabb::new(self.center - rad, self.center + rad);
        Some(bb)
    }
}
