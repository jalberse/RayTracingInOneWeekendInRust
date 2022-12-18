use std::io::Write;
use std::io::{self, BufWriter};

use glam::{dvec3, DVec3};
use rand::random;

use crate::camera::Camera;
use crate::hittable::HittableList;

pub struct Renderer {
    image_width: u32,
    image_height: u32,
}

impl Renderer {
    #[allow(dead_code)]
    pub fn new(image_width: u32, image_height: u32) -> Renderer {
        Renderer {
            image_width,
            image_height,
        }
    }

    pub fn from_aspect_ratio(image_width: u32, aspect_ratio: f64) -> Renderer {
        Renderer {
            image_width,
            image_height: (image_width as f64 / aspect_ratio) as u32,
        }
    }

    /// Outputs an image to stdout
    pub fn render(
        &self,
        camera: &Camera,
        world: &HittableList,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> std::io::Result<()> {
        let stdout = io::stdout();
        let mut buf_writer = io::BufWriter::new(stdout);

        let stderr = io::stderr();
        let mut stderr_buf_writer = io::BufWriter::new(stderr);

        write!(
            buf_writer,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )?;

        for j in (0..self.image_height).rev() {
            write!(stderr_buf_writer, "\rScanlines remaining: {:04}", j)?;
            stderr_buf_writer.flush().unwrap();
            for i in 0..self.image_width {
                let color: DVec3 = {
                    let mut color_accumulator = dvec3(0.0, 0.0, 0.0);
                    for _ in 0..samples_per_pixel {
                        let u = (i as f64 + random::<f64>()) / (self.image_width - 1) as f64;
                        let v = (j as f64 + random::<f64>()) / (self.image_height - 1) as f64;
                        let ray = camera.get_ray(u, v);

                        color_accumulator += ray.ray_color(&world, max_depth);
                    }
                    color_accumulator.into()
                };
                Self::write_color(&mut buf_writer, &color, samples_per_pixel).unwrap();
            }
        }

        write!(stderr_buf_writer, "\nDone.\n")?;

        buf_writer.flush().unwrap();
        stderr_buf_writer.flush().unwrap();

        Ok(())
    }

    fn write_color<T>(
        buf_writer: &mut BufWriter<T>,
        color: &DVec3,
        samples_per_pixel: u32,
    ) -> std::io::Result<()>
    where
        T: std::io::Write,
    {
        // Sqrt is taken for gamma correction
        let scale = 1.0 / samples_per_pixel as f64;
        let r = f64::clamp(f64::sqrt(color.x * scale), 0.0, 0.999);
        let g = f64::clamp(f64::sqrt(color.y * scale), 0.0, 0.999);
        let b = f64::clamp(f64::sqrt(color.z * scale), 0.0, 0.999);

        let ir = (r * 256.0) as u32;
        let ig = (g * 256.0) as u32;
        let ib = (b * 256.0) as u32;

        write!(buf_writer, "{} {} {}\n", ir, ig, ib)?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn aspect_ratio(&self) -> f32 {
        self.image_width as f32 / self.image_height as f32
    }
}
