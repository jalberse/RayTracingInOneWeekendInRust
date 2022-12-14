mod camera;
mod color;
mod hittable;
mod ray;
mod renderer;
mod sphere;

use camera::Camera;
use glam::Vec3;
use renderer::Renderer;
use sphere::Sphere;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let camera = Camera::new(viewport_height, aspect_ratio * viewport_height, 1.0);
    let renderer = Renderer::from_aspect_ratio(400, 16.0 / 9.0);

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

    renderer.render(&camera, &sphere).unwrap();
}
