use crate::{ray::Ray, utils};

use glam::DVec3;

pub struct Camera {
    origin: DVec3,
    horizontal: DVec3,
    vertical: DVec3,
    lower_left_corner: DVec3,
    u: DVec3,
    v: DVec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: DVec3,
        look_at: DVec3,
        view_up: DVec3,
        vertical_field_of_view: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = f64::to_radians(vertical_field_of_view);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = view_up.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let random_in_lens = self.lens_radius * utils::random_in_unit_disk();
        let offset = self.u * random_in_lens.x + self.v * random_in_lens.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
