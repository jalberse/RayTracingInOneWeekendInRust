use glam::{vec3, Vec3};

use crate::{color::Color, sphere::Sphere};

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
        if sphere.hit(&self) {
            return vec3(1.0, 0.0, 0.0).into();
        }
        let t = 0.5 * (self.direction.normalize().y + 1.0);
        ((1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)).into()
    }
}
