use std::ops::Neg;

use glam::{dvec3, DVec3};
use rand::Rng;

use crate::hittable::{Hittable, HittableList};

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }

    pub fn ray_color<T>(&self, world: &HittableList<T>, depth: u32) -> DVec3
    where
        T: Hittable,
    {
        // Ray bounce limit reached; accumulate no further light.
        if depth <= 0 {
            return DVec3::ZERO;
        }

        let hit_record = world.hit(&self, 0.001, f64::INFINITY);
        if let Some(hit_record) = hit_record {
            let reflection_target =
                hit_record.point + hit_record.normal + Self::random_on_unit_sphere();
            let reflection_ray = Ray::new(hit_record.point, reflection_target - hit_record.point);
            return 0.5 * reflection_ray.ray_color(world, depth - 1);
        }
        // Background
        let t = 0.5 * (self.direction.normalize().y + 1.0);
        (1.0 - t) * dvec3(1.0, 1.0, 1.0) + t * dvec3(0.5, 0.7, 1.0)
    }

    /// Useful for faux-lambertian diffuse shading
    fn random_in_unit_sphere() -> DVec3 {
        let mut rng = rand::thread_rng();

        loop {
            let vec = DVec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            );
            if vec.length_squared() < 1.0 {
                return vec;
            }
        }
    }

    /// Useful for lambertian diffuse shading
    fn random_on_unit_sphere() -> DVec3 {
        Self::random_in_unit_sphere().normalize()
    }

    /// Useful as an alternative diffuse shading approach compared to random_on_unit_sphere()
    #[allow(dead_code)]
    fn random_in_hemisphere(normal: &DVec3) -> DVec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(*normal).is_sign_positive() {
            in_unit_sphere
        } else {
            in_unit_sphere.neg()
        }
    }
}
