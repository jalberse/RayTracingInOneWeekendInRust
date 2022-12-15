use std::io::Write;
use std::io::{self, BufWriter};

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{Hittable, HittableList};

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

    pub fn from_aspect_ratio(image_width: u32, aspect_ratio: f32) -> Renderer {
        Renderer {
            image_width,
            image_height: (image_width as f32 / aspect_ratio) as u32,
        }
    }

    /// Outputs an image to stdout
    pub fn render<T>(&self, camera: &Camera, world: &HittableList<T>) -> std::io::Result<()>
    where
        T: Hittable,
    {
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
            for i in 0..self.image_width {
                write!(stderr_buf_writer, "\rScanlines remaining: {}", j)?;

                let u = i as f32 / (self.image_width - 1) as f32;
                let v = j as f32 / (self.image_height - 1) as f32;
                let ray = camera.get_ray(u, v);

                let color = ray.ray_color(&world);
                Self::write_color(&mut buf_writer, &color).unwrap();
            }
        }

        write!(stderr_buf_writer, "\nDone.\n")?;

        buf_writer.flush().unwrap();
        stderr_buf_writer.flush().unwrap();

        Ok(())
    }

    fn write_color<T>(buf_writer: &mut BufWriter<T>, color: &Color) -> std::io::Result<()>
    where
        T: std::io::Write,
    {
        let ir = (color.as_vec().x * 255.999) as u32;
        let ig = (color.as_vec().y * 255.999) as u32;
        let ib = (color.as_vec().z * 255.999) as u32;

        write!(buf_writer, "{} {} {}\n", ir, ig, ib)?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn aspect_ratio(&self) -> f32 {
        self.image_width as f32 / self.image_height as f32
    }
}
