mod camera;
mod color;
mod ray;
mod renderer;

use camera::Camera;
use renderer::Renderer;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let camera = Camera::new(viewport_height, aspect_ratio * viewport_height, 1.0);
    let renderer = Renderer::from_aspect_ratio(400, 16.0 / 9.0);

    renderer.render(&camera).unwrap();
}
