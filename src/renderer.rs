use std::io::Write;
use std::io::{self, BufWriter};

use palette::Pixel;
use palette::Srgb;
use rand::random;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::utils::srgb_from_dvec3;

pub struct Renderer {
    image_width: usize,
    image_height: usize,
}

impl Renderer {
    #[allow(dead_code)]
    pub fn new(image_width: usize, image_height: usize) -> Renderer {
        Renderer {
            image_width,
            image_height,
        }
    }

    pub fn from_aspect_ratio(image_width: usize, aspect_ratio: f64) -> Renderer {
        Renderer {
            image_width,
            image_height: (image_width as f64 / aspect_ratio) as usize,
        }
    }

    /// Outputs an image to stdout
    pub fn render(
        &self,
        camera: &Camera,
        world: &HittableList,
        samples_per_pixel: u32,
        max_depth: u32,
        tile_width: usize,
        tile_height: usize,
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

        let tiles = Tile::tile(self.image_width, self.image_height, tile_width, tile_height);
        let mut colors = ImageColors::new(self.image_width, self.image_height);
        let mut tiles_ctr = tiles.len();
        for tile in tiles {
            write!(stderr_buf_writer, "\rTiles remaining: {:04}", tiles_ctr)?;
            stderr_buf_writer.flush().unwrap();
            tiles_ctr -= 1;
            for i in 0..tile.width {
                for j in 0..tile.height {
                    let pixel_coords = tile.get_pixel_coordinates(i, j);
                    let color =
                        self.get_color(&pixel_coords, samples_per_pixel, world, max_depth, camera);
                    colors.set_color(&pixel_coords, color);
                }
            }
        }

        for y in (0..self.image_height).rev() {
            for x in 0..self.image_width {
                Self::write_color(&mut buf_writer, colors.get_color(x, y)).unwrap();
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

    fn get_color(
        &self,
        pixel_coords: &PixelCoordinates,
        samples_per_pixel: u32,
        world: &HittableList,
        max_depth: u32,
        camera: &Camera,
    ) -> Srgb {
        let mut color_accumulator = Srgb::new(0.0, 0.0, 0.0).into_linear();
        for _ in 0..samples_per_pixel {
            let u = (pixel_coords.x as f64 + random::<f64>()) / (self.image_width - 1) as f64;
            let v = (pixel_coords.y as f64 + random::<f64>()) / (self.image_height - 1) as f64;
            let ray = camera.get_ray(u, v);

            color_accumulator += srgb_from_dvec3(ray.ray_color(&world, max_depth)).into_linear();
        }
        color_accumulator = color_accumulator / samples_per_pixel as f32;
        Srgb::from_linear(color_accumulator)
    }
}

/// Stores the color of each pixel in the image.
struct ImageColors {
    /// Matrix of colors in the image, flattened row-major.
    colors: Vec<Srgb>,
    image_width: usize,
}

impl ImageColors {
    pub fn new(image_width: usize, image_height: usize) -> ImageColors {
        ImageColors {
            colors: vec![Srgb::new(0.0, 0.0, 0.0); image_width * image_height],
            image_width,
        }
    }

    pub fn set_color(&mut self, coords: &PixelCoordinates, color: Srgb) {
        let idx = self.get_idx(coords.x, coords.y);
        self.colors[idx] = color;
    }

    pub fn get_color(&self, x: usize, y: usize) -> &Srgb {
        &self.colors[self.get_idx(x, y)]
    }

    fn get_idx(&self, x: usize, y: usize) -> usize {
        y * self.image_width + x
    }
}

struct PixelCoordinates {
    pub x: usize,
    pub y: usize,
}

struct Tile {
    /// Width of the tile, in pixels.
    width: usize,
    /// Height of the tile, in pixels.
    height: usize,
    /// The first pixel X coordinate of this tile in the full image.
    x_coord_start: usize,
    /// The first pixel Y coordinate of this tile in the full image.
    y_coord_start: usize,
}

impl Tile {
    pub fn new(width: usize, height: usize, x_coord_start: usize, y_coord_start: usize) -> Tile {
        Tile {
            width,
            height,
            x_coord_start,
            y_coord_start,
        }
    }

    /// Returns a list of Tiles covering the image.
    ///
    /// The tiles are returned in a flattened Vec in row-major order.
    /// If the image cannot be perfectly divided by the tile width or height,
    /// then smaller tiles are created to fill the remainder of the image width or height.
    /// It's recommended to pick a tiling size that fits into the image resolution well.
    /// Note that 8x8 is a reasonable tile size and 8 evenly divides common resolution
    /// sizes like 1920, 1080, 720, etc.
    ///
    /// * `image_width` - Width of the image to be tiled, in pixels.
    /// * `image_height` - Height of the image to be tiles, in pixels.
    /// * `tile_width` - Width of each tile, in pixels.
    /// * `tile_height` - Height of each tile, in pixels.
    pub fn tile(
        image_width: usize,
        image_height: usize,
        tile_width: usize,
        tile_height: usize,
    ) -> Vec<Tile> {
        let num_horizontal_tiles = image_width / tile_width;
        let remainder_horizontal_pixels = image_width % tile_width;
        let num_vertical_tiles = image_height / tile_height;
        let remainder_vertical_pixels = image_height % tile_height;

        let mut tiles = Vec::with_capacity(
            (num_horizontal_tiles * num_vertical_tiles)
                .try_into()
                .unwrap(),
        );

        for tile_y in 0..num_vertical_tiles {
            for tile_x in 0..num_horizontal_tiles {
                tiles.push(Tile::new(
                    tile_width,
                    tile_height,
                    tile_x * tile_width,
                    tile_y * tile_height,
                ));
            }
            // Add the rightmost row if necessary
            if remainder_horizontal_pixels > 0 {
                tiles.push(Tile::new(
                    remainder_horizontal_pixels,
                    tile_height,
                    num_horizontal_tiles * tile_width,
                    tile_y * tile_height,
                ));
            }
        }
        // Add the bottom row if necessary
        if remainder_vertical_pixels > 0 {
            for tile_x in 0..num_horizontal_tiles {
                tiles.push(Tile::new(
                    tile_width,
                    remainder_vertical_pixels,
                    tile_x * tile_width,
                    num_vertical_tiles * tile_height,
                ));
            }
        }
        // Add the bottom-most, right-most Tile if necessary
        if remainder_horizontal_pixels > 0 && remainder_vertical_pixels > 0 {
            tiles.push(Tile::new(
                remainder_horizontal_pixels,
                remainder_vertical_pixels,
                num_horizontal_tiles * tile_width,
                num_vertical_tiles * tile_height,
            ));
        }

        tiles
    }

    /// Given the `x`, `y` coordinate within this tile, get the corresponding
    /// pixel coordinate in the full image.
    pub fn get_pixel_coordinates(&self, x: usize, y: usize) -> PixelCoordinates {
        assert!(x < self.width);
        assert!(y < self.height);
        PixelCoordinates {
            x: self.x_coord_start + x,
            y: self.y_coord_start + y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tile;

    #[test]
    fn tile_perfect_tiling() {
        let image_width = 300;
        let image_height = 30;
        let tile_width = 100;
        let tile_height = 10;
        let tiles = Tile::tile(image_width, image_height, tile_width, tile_height);

        assert!(tiles.len() == 9);

        assert!(tiles[0].width == tile_width);
        assert!(tiles[0].height == tile_height);
        assert!(tiles[0].x_coord_start == 0);
        assert!(tiles[0].y_coord_start == 0);

        assert!(tiles[1].width == tile_width);
        assert!(tiles[1].height == tile_height);
        assert!(tiles[1].x_coord_start == tile_width);
        assert!(tiles[1].y_coord_start == 0);

        assert!(tiles[3].x_coord_start == 0);
        assert!(tiles[3].y_coord_start == tile_height);

        assert!(tiles.last().unwrap().width == tile_width);
        assert!(tiles.last().unwrap().height == tile_height);
        assert!(tiles.last().unwrap().x_coord_start == 200);
        assert!(tiles.last().unwrap().y_coord_start == 20);
    }

    #[test]
    fn tile_imperfect_tiling() {
        let image_width = 310;
        let image_height = 31;
        let tile_width = 100;
        let tile_height = 10;
        let tiles = Tile::tile(image_width, image_height, tile_width, tile_height);

        assert!(tiles.len() == 16);

        assert!(tiles[0].width == tile_width);
        assert!(tiles[0].height == tile_height);
        assert!(tiles[0].x_coord_start == 0);
        assert!(tiles[0].y_coord_start == 0);

        assert!(tiles[4].x_coord_start == 0);
        assert!(tiles[4].y_coord_start == tile_height);
        assert!(tiles[4].width == tile_width);
        assert!(tiles[4].height == tile_height);

        // Top right - Width remainder tile
        assert!(tiles[3].x_coord_start == 300);
        assert!(tiles[3].y_coord_start == 0);
        assert!(tiles[3].width == 10);
        assert!(tiles[3].height == tile_height);

        // Bottom left - height remainder tile
        assert!(tiles[12].x_coord_start == 0);
        assert!(tiles[12].y_coord_start == 30);
        assert!(tiles[12].width == tile_width);
        assert!(tiles[12].height == 1);

        // Bottom right remainder tile
        assert!(tiles[15].x_coord_start == 300);
        assert!(tiles[15].y_coord_start == 30);
        assert!(tiles[15].width == 10);
        assert!(tiles[15].height == 1);
    }
}
