use std::sync::{Arc, Mutex};

use ahash::AHashMap;
use glam::Vec3;

use crate::{
    bvh::BvhId,
    hittable::{Hittable, HittableList},
    hrpp::Predictor,
};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    /// The time at which the ray exists
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn ray_color(
        &self,
        world: &HittableList,
        depth: u32,
        background: Vec3,
        predictors: &Arc<Option<Mutex<AHashMap<BvhId, Predictor>>>>,
    ) -> Vec3 {
        // Ray bounce limit reached; accumulate no further light.
        if depth <= 0 {
            return Vec3::ZERO;
        }

        let hit_record = world.hit(&self, 0.001, f32::INFINITY, &predictors);
        if let Some(hit_record) = hit_record {
            let emitted = hit_record
                .material
                .emit(hit_record.u, hit_record.v, &hit_record.point);

            if let Some(scatter_record) = hit_record.material.scatter(&self, &hit_record) {
                emitted
                    + scatter_record.attenuation
                        * scatter_record
                            .ray
                            .ray_color(world, depth - 1, background, &predictors)
            } else {
                emitted
            }
        } else {
            background
        }
    }
}
