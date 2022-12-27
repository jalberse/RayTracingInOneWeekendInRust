use glam::{dvec3, DVec3};

use crate::{
    hittable::{Hittable, HittableList},
    materials::material::Material,
};

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
    /// The time at which the ray exists
    pub time: f64,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }

    pub fn ray_color(&self, world: &HittableList, depth: u32) -> DVec3 {
        // Ray bounce limit reached; accumulate no further light.
        if depth <= 0 {
            return DVec3::ZERO;
        }

        let hit_record = world.hit(&self, 0.001, f64::INFINITY);
        if let Some(hit_record) = hit_record {
            if let Some(scatter_record) = hit_record.material.scatter(&self, &hit_record) {
                return scatter_record.attenuation * scatter_record.ray.ray_color(world, depth - 1);
            } else {
                return DVec3::ZERO;
            }
        }
        // Background
        let t = 0.5 * (self.direction.normalize().y + 1.0);
        (1.0 - t) * dvec3(1.0, 1.0, 1.0) + t * dvec3(0.5, 0.7, 1.0)
    }
}
