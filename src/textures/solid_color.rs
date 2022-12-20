use glam::{dvec3, DVec3};

use super::texture::Texture;

pub struct SolidColor {
    color: DVec3,
}

impl SolidColor {
    pub fn new(color: DVec3) -> SolidColor {
        SolidColor { color }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {
            color: dvec3(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &DVec3) -> DVec3 {
        self.color
    }
}
