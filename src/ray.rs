use glam::{vec3, Vec3};

use crate::{color::Color, hittable::hittable, sphere::Sphere};

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

    pub fn ray_color(&self, sphere: &Sphere) -> Color {
        let hit_record = sphere.hit(&self, 0.0, f32::INFINITY);
        if let Some(hit_record) = hit_record {
            return (0.5 * (hit_record.normal + vec3(1.0, 1.0, 1.0))).into();
        }
        // Background
        let t = 0.5 * (self.direction.normalize().y + 1.0);
        ((1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)).into()
    }
}
