use std::sync::Arc;

use glam::Vec3;

use super::{solid_color::SolidColor, texture::Texture};

pub struct Checker {
    scale: f32,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl Checker {
    pub fn new(scale: f32, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Checker {
        Checker { scale, even, odd }
    }

    pub fn from_color(scale: f32, even_color: Vec3, odd_color: Vec3) -> Checker {
        Checker {
            scale,
            even: Arc::new(SolidColor::new(even_color)),
            odd: Arc::new(SolidColor::new(odd_color)),
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: f32, v: f32, p: &glam::Vec3) -> glam::Vec3 {
        let sines =
            f32::sin(self.scale * p.x) * f32::sin(self.scale * p.y) * f32::sin(self.scale * p.z);
        if sines.is_sign_negative() {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
