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
        look_from: DVec3,
        look_at: DVec3,
        view_up: DVec3,
        vertical_field_of_view: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = f64::to_radians(vertical_field_of_view);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = view_up.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
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
