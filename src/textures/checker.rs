use std::sync::Arc;

use glam::DVec3;

use super::{solid_color::SolidColor, texture::Texture};

pub struct Checker {
    scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl Checker {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Checker {
        Checker { scale, even, odd }
    }

    pub fn from_color(scale: f64, even_color: DVec3, odd_color: DVec3) -> Checker {
        Checker {
            scale,
            even: Arc::new(SolidColor::new(even_color)),
            odd: Arc::new(SolidColor::new(odd_color)),
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: &glam::DVec3) -> glam::DVec3 {
        let sines =
            f64::sin(self.scale * p.x) * f64::sin(self.scale * p.y) * f64::sin(self.scale * p.z);
        if sines.is_sign_negative() {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
