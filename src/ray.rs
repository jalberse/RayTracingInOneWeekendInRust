use glam::{vec3, Vec3};
use rand::Rng;

use crate::hittable::{Hittable, HittableList};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn ray_color<T>(&self, world: &HittableList<T>, depth: u32) -> Vec3
    where
        T: Hittable,
    {
        // Ray bounce limit reached; accumulate no further light.
        if depth <= 0 {
            return Vec3::ZERO;
        }

        let hit_record = world.hit(&self, 0.0, f32::INFINITY);
        if let Some(hit_record) = hit_record {
            let reflection_target =
                hit_record.point + hit_record.normal + Self::random_in_unit_sphere();
            let reflection_ray = Ray::new(hit_record.point, reflection_target - hit_record.point);
            return 0.5 * reflection_ray.ray_color(world, depth - 1);
        }
        // Background
        let t = 0.5 * (self.direction.normalize().y + 1.0);
        (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
    }

    fn random_in_unit_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();

        loop {
            let vec = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            );
            if vec.length_squared() < 1.0 {
                return vec;
            }
        }
    }
}
