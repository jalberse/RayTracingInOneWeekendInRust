use std::io;
use std::io::Write;
use std::sync::Arc;

use crossbeam::channel;
use crossbeam::channel::Sender;
use crossbeam::queue::ArrayQueue;
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
        threads: u32,
        tile_width: usize,
        tile_height: usize,
    ) -> std::io::Result<()> {
        let stderr = io::stderr();
        let mut stderr_buf_writer = io::BufWriter::new(stderr);

        let tiles = Tile::tile(self.image_width, self.image_height, tile_width, tile_height);
        let tiles_queue = Arc::new(ArrayQueue::new(tiles.len()));
        tiles.into_iter().for_each(|tile| {
            tiles_queue.push(tile).unwrap();
        });
        let mut colors = ImageColors::new(self.image_width, self.image_height);

        crossbeam::scope(|scope| {
            let (s, r) = channel::unbounded();
            for _i in 0..threads {
                let s = s.clone();
                let tiles_queue = tiles_queue.clone();
                scope.spawn(move |_| {
                    self.render_tile(s, tiles_queue, samples_per_pixel, world, max_depth, camera);
                });
            }
            drop(s);

            for received in r {
                let tiles_remaining = tiles_queue.len();
                write!(
                    stderr_buf_writer,
                    "\rTiles remaining: {:07}",
                    tiles_remaining
                )
                .unwrap();
                stderr_buf_writer.flush().unwrap();

                // Update the image with the colors from this tile
                let tile = &received.tile;
                for x in 0..tile.width {
                    for y in 0..tile.height {
                        let full_image_pixel_coords = tile.get_full_image_pixel_coordinates(x, y);
                        let color = received.colors.get_color(x, y);
                        colors.set_color(&full_image_pixel_coords, *color);
                    }
                }
            }
        })
        .unwrap();

        write!(stderr_buf_writer, "\nDone tracing.\n")?;

        write!(stderr_buf_writer, "Writing to file...\n")?;
        self.write_ppm(&colors).unwrap();
        write!(stderr_buf_writer, "Done writing to file.\n")?;

        stderr_buf_writer.flush().unwrap();
        Ok(())
    }

    fn write_ppm(&self, colors: &ImageColors) -> std::io::Result<()> {
        let stdout = io::stdout();
        let mut buf_writer = io::BufWriter::new(stdout);
        write!(
            buf_writer,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )?;

        for y in (0..self.image_height).rev() {
            for x in 0..self.image_width {
                let color = colors.get_color(x, y);
                let raw: [u8; 3] = Srgb::into_raw(color.into_format());
                write!(buf_writer, "{} {} {}\n", raw[0], raw[1], raw[2])?;
            }
        }

        buf_writer.flush().unwrap();

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

    fn render_tile(
        &self,
        sender: Sender<TileRenderMessage>,
        tiles_queue: Arc<ArrayQueue<Tile>>,
        samples_per_pixel: u32,
        world: &HittableList,
        max_depth: u32,
        camera: &Camera,
    ) {
        loop {
            let tile = tiles_queue.pop();
            if let Some(tile) = tile {
                let mut tile_colors = ImageColors::new(tile.width, tile.height);
                for y in 0..tile.height {
                    for x in 0..tile.width {
                        let pixel_coords = tile.get_full_image_pixel_coordinates(x, y);
                        let color = self.get_color(
                            &pixel_coords,
                            samples_per_pixel,
                            world,
                            max_depth,
                            camera,
                        );
                        tile_colors.set_color(&PixelCoordinates::new(x, y), color);
                    }
                }
                sender
                    .send(TileRenderMessage::new(tile, tile_colors))
                    .unwrap();
            } else {
                return;
            }
        }
    }
}

/// The information necessary to populate ImageColor with color data
/// for a single Tile's pixels. Sent from worker threads to the main
/// thread in Renderer.
struct TileRenderMessage {
    tile: Tile,
    /// The colors for this tile (where this tile is the "Image")
    colors: ImageColors,
}

impl TileRenderMessage {
    pub fn new(tile: Tile, colors: ImageColors) -> TileRenderMessage {
        TileRenderMessage { tile, colors }
    }
}

/// Stores the color of each pixel in an image.
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

impl PixelCoordinates {
    pub fn new(x: usize, y: usize) -> PixelCoordinates {
        PixelCoordinates { x, y }
    }
}

#[derive(Copy, Clone, Debug)]
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

        let mut tiles = Vec::with_capacity(num_horizontal_tiles * num_vertical_tiles);

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
    pub fn get_full_image_pixel_coordinates(&self, x: usize, y: usize) -> PixelCoordinates {
        assert!(x < self.width);
        assert!(y < self.height);
        PixelCoordinates::new(self.x_coord_start + x, self.y_coord_start + y)
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
