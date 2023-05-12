use crate::{ray::Ray, utils};

use glam::Vec3;
use rand::{thread_rng, Rng};

pub struct Camera {
    /// The lens is centered on the origin
    origin: Vec3,
    /// Horizontal vector in the focus plane.
    /// Magnitude is the width of the visible portion of the focus plane.
    horizontal: Vec3,
    /// Vertical vector in the focus plane.
    /// Magnitude is the width of the visible portion of the focus plane.
    vertical: Vec3,
    /// Lower left corner visible in the focus plane.
    lower_left_corner: Vec3,
    /// A "horizontal" vector in the plane of the lens
    u: Vec3,
    /// A "vertical" vector in the plane of the lens
    v: Vec3,
    /// Radius of the lens disk
    lens_radius: f32,
    /// Shutter open time
    time_start: f32,
    /// Shutter close time
    time_end: f32,
}

impl Camera {
    /// Creates a new camera
    ///
    /// * `look_from` - Camera will look from this point.
    /// * `look_at` - Camera will look at this point.
    /// * `view_up` - Orients the camera. Typically "world up" (0.0, 1.0, 0.0).
    /// * `vertical_field_of_view` - FOV is set via the vertical FOV. This also determines
    /// the horizontal FOV according to the aspect ratio.
    /// * `aspect ratio` - The aspect ratio of the camera.
    /// * `aperture` - Twice the lens radius. Larger apertures are more blurry further
    /// from the focus plane. Smaller apertures are more in focus. An aperture of 0.0 is
    /// perfectly in focus at all distances from the focus plane.
    /// * `focus_dist` - The distance to the focus plane.
    /// * `time_start` - Shutter open time.
    /// * `time_end` - Shutter close time.
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vertical_field_of_view: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        time_start: f32,
        time_end: f32,
    ) -> Camera {
        let theta = f32::to_radians(vertical_field_of_view);
        let h = f32::tan(theta / 2.0);
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
            time_start,
            time_end,
        }
    }

    /// Gets a ray from the camera from a random location on the lens,
    /// at a random time while the shutter is open, towards `s` and `t`.
    ///
    /// `s` is the horizontal fraction of the camera's view and `t` is the vertical fraction.
    /// `s` and `t` are expected to be roughly within (0..1), but it's expected
    /// to have slightly larger or smaller values to achieve various effects.
    /// For example, `s == 0.5` and `t == 0.5` will return a ray towards the
    /// point on the focus plane which is at the center of the camera's view.
    /// If `s == 0.0` and `t == 0.0`, the ray will go towards the top left
    /// point on the focus plane that's within the camera's view.
    /// Typical use will involve getting this ray's corresponding pixel's coordinates
    /// and dividing those by the width and height of your image to get `s` and `t` respectively,
    /// with some randomness introduced for anti-aliasing.
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let random_in_lens = self.lens_radius * utils::random_in_unit_disk();
        let offset = self.u * random_in_lens.x + self.v * random_in_lens.y;

        let mut rng = thread_rng();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            rng.gen_range(self.time_start..=self.time_end),
        )
    }
}
