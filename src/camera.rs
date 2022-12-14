use crate::ray::Ray;

use glam::Vec3;

pub struct Camera {
    viewport_height: f32,
    viewport_width: f32,
    focal_length: f32,
}

impl Camera {
    pub fn new(viewport_height: f32, viewport_width: f32, focal_length: f32) -> Camera {
        Camera {
            viewport_height,
            viewport_width,
            focal_length,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let origin = Vec3::ZERO;
        let horizontal = Vec3::new(self.viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, self.viewport_height, 0.0);
        let lower_left_corner =
            Vec3::ZERO - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, self.focal_length);
        Ray::new(
            origin,
            lower_left_corner + u * horizontal + v * vertical - origin,
        )
    }
}
