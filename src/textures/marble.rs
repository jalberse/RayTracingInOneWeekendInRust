use glam::dvec3;
use noise::{NoiseFn, Perlin, Turbulence};
use rand::random;

use super::texture::Texture;

pub struct Marble {
    noise: Turbulence<Perlin, Perlin>,
    scale: f64,
}

impl Marble {
    pub fn new(scale: f64) -> Marble {
        let perlin = Perlin::new(random::<u32>());
        let turb: Turbulence<_, Perlin> = Turbulence::new(perlin)
            .set_frequency(1.0)
            .set_power(1.0)
            .set_roughness(6);
        Marble { noise: turb, scale }
    }
}

impl Texture for Marble {
    fn value(&self, _u: f64, _v: f64, p: &glam::DVec3) -> glam::DVec3 {
        dvec3(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + f64::sin(self.scale * p.z + 10.0 * self.noise.get(<[f64; 3]>::from(*p))))
    }
}
