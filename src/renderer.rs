use std::io::Write;
use std::io::{self, BufWriter};

use palette::Pixel;
use palette::Srgb;
use rand::random;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::utils::srgb_from_dvec3;

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
                let color: Srgb = {
                    let mut color_accumulator = Srgb::new(0.0, 0.0, 0.0).into_linear();
                    for _ in 0..samples_per_pixel {
                        let u = (i as f64 + random::<f64>()) / (self.image_width - 1) as f64;
                        let v = (j as f64 + random::<f64>()) / (self.image_height - 1) as f64;
                        let ray = camera.get_ray(u, v);

                        color_accumulator +=
                            srgb_from_dvec3(ray.ray_color(&world, max_depth)).into_linear();
                    }
                    color_accumulator = color_accumulator / samples_per_pixel as f32;
                    Srgb::from_linear(color_accumulator)
                };
                Self::write_color(&mut buf_writer, &color).unwrap();
            }
        }

        write!(stderr_buf_writer, "\nDone.\n")?;

        buf_writer.flush().unwrap();
        stderr_buf_writer.flush().unwrap();

        Ok(())
    }

    fn write_color<T>(buf_writer: &mut BufWriter<T>, color: &Srgb) -> std::io::Result<()>
    where
        T: std::io::Write,
    {
        let raw: [u8; 3] = Srgb::into_raw(color.into_format());

        write!(buf_writer, "{} {} {}\n", raw[0], raw[1], raw[2])?;

        Ok(())
    }
}
