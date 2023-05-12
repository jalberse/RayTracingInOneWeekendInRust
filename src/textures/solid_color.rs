use glam::{Vec3, vec3};

use super::texture::Texture;

pub struct SolidColor {
    color: Vec3,
}

impl SolidColor {
    pub fn new(color: Vec3) -> SolidColor {
        SolidColor { color }
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> SolidColor {
        SolidColor {
            color: vec3(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        self.color
    }
}
