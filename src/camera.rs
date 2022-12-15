use crate::ray::Ray;

use glam::DVec3;

pub struct Camera {
    origin: DVec3,
    horizontal: DVec3,
    vertical: DVec3,
    lower_left_corner: DVec3,
}

impl Camera {
    pub fn new(
        origin: DVec3,
        viewport_height: f64,
        viewport_width: f64,
        focal_length: f64,
    ) -> Camera {
        let horizontal = DVec3::new(viewport_width, 0.0, 0.0);
        let vertical = DVec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            DVec3::ZERO - horizontal / 2.0 - vertical / 2.0 - DVec3::new(0.0, 0.0, focal_length);
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
