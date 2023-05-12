use glam::{vec3};
use noise::{NoiseFn, Perlin, Turbulence};
use rand::random;

use super::texture::Texture;

pub struct Marble {
    noise: Turbulence<Perlin, Perlin>,
    scale: f32,
}

impl Marble {
    pub fn new(scale: f32) -> Marble {
        let perlin = Perlin::new(random::<u32>());
        let turb: Turbulence<_, Perlin> = Turbulence::new(perlin)
            .set_frequency(1.0)
            .set_power(1.0)
            .set_roughness(6);
        Marble { noise: turb, scale }
    }
}

impl Texture for Marble {
    fn value(&self, _u: f32, _v: f32, p: &glam::Vec3) -> glam::Vec3 {
        vec3(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + f32::sin(self.scale * p.z + 10.0 * self.noise.get([p.x as f64, p.y as f64, p.z as f64]) as f32))
    }
}
