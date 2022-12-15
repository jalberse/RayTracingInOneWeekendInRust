mod camera;
mod hittable;
mod materials;
mod ray;
mod renderer;
mod sphere;
mod utils;

use camera::Camera;
use glam::{dvec3, DVec3};
use hittable::HittableList;
use materials::{lambertian::Lambertian, material::Material};
use renderer::Renderer;
use sphere::Sphere;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let camera = Camera::new(
        DVec3::ZERO,
        viewport_height,
        aspect_ratio * viewport_height,
        1.0,
    );
    let renderer = Renderer::from_aspect_ratio(400, 16.0 / 9.0);

    let material_ground = Material::Lambertian(Lambertian::new(dvec3(0.8, 0.8, 0.0)));
    let material_center = Material::Lambertian(Lambertian::new(dvec3(0.7, 0.3, 0.3)));

    let sphere = Box::new(Sphere::new(
        DVec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    ));
    let sphere_2 = Box::new(Sphere::new(
        DVec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    let world = HittableList::from_vec(vec![sphere, sphere_2]);
    let samples_per_pixel = 100;
    let max_depth = 50;
    renderer
        .render(&camera, &world, samples_per_pixel, max_depth)
        .unwrap();
}
