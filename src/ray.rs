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
        let t = sphere.hit(&self);
        if t.is_sign_positive() {
            let normal = (self.at(t) - sphere.center).normalize();
            return (0.5 * vec3(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0)).into();
        }
        // Background
        let t = 0.5 * (self.direction.normalize().y + 1.0);
        ((1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)).into()
    }
}
