use glam::{vec3, Vec3};

use crate::color::Color;

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

    pub fn ray_color(&self) -> Color {
        // TODO For now, this just returns something which will make a gradient. We will actually do ray tracing here eventually!
        let t = 0.5 * (self.direction.normalize().y + 1.0);
        ((1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)).into()
    }
}
