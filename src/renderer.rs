use std::io::Write;
use std::io::{self, BufWriter};

use glam::Vec3;

use crate::color::Color;

pub struct Renderer {
    image_width: u32,
    image_height: u32,
}

impl Renderer {
    pub fn new(image_width: u32, image_height: u32) -> Renderer {
        Renderer {
            image_width,
            image_height,
        }
    }

    /// Outputs an image to stdout
    pub fn render(&self) -> std::io::Result<()> {
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

                let r = (i as f32) / (self.image_width - 1) as f32;
                let g = (j as f32) / (self.image_height - 1) as f32;
                let b: f32 = 0.25;

                let color = Color::new(Vec3::new(r, g, b));
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
}
