use crate::ray::Ray;

use glam::DVec3;

pub struct Camera {
    origin: DVec3,
    horizontal: DVec3,
    vertical: DVec3,
    lower_left_corner: DVec3,
}

impl Camera {
    pub fn new(vertical_field_of_view: f64, aspect_ratio: f64) -> Camera {
        let theta = f64::to_radians(vertical_field_of_view);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        let origin = DVec3::ZERO;
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
