use super::texture::Texture;

use glam::DVec3;
use image::{io::Reader as ImageReader, ImageBuffer, Rgb};

use std::path::Path;

pub struct ImageTexture {
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl ImageTexture {
    pub fn new(path: &Path) -> ImageTexture {
        // TODO propogate errors
        let image = ImageReader::open(path).unwrap().decode().unwrap().to_rgb8();

        ImageTexture { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &glam::DVec3) -> glam::DVec3 {
        let u = f64::clamp(u, 0.0, 1.0);
        let v = f64::clamp(v, 0.0, 1.0);
        // Flip V to mathc image coordinate system
        let v = 1.0 - v;

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;

        // Clamp integer mapping
        let i = if i >= self.image.width() {
            self.image.width() - 1
        } else {
            i
        };
        let j = if j >= self.image.height() {
            self.image.height() - 1
        } else {
            j
        };

        let pixel = self.image.get_pixel(i, j);

        let color_scale = 1.0 / 255.0;
        DVec3::new(
            pixel.0[0] as f64 * color_scale,
            pixel.0[1] as f64 * color_scale,
            pixel.0[2] as f64 * color_scale,
        )
    }
}
