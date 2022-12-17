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
use materials::{
    dialectric::Dialectric,
    lambertian::Lambertian,
    material::Material,
    metal::Metal,
    utils::{random_color, random_color_range},
};
use rand::random;
use renderer::Renderer;
use sphere::Sphere;

fn main() {
    let aspect_ratio = 3.0 / 2.0;
    let look_from = dvec3(13.0, 2.0, 3.0);
    let look_at = dvec3(0.0, 0.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        dvec3(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );
    let renderer = Renderer::from_aspect_ratio(1920, 16.0 / 9.0);

    let world = random_scene();

    let samples_per_pixel = 500;
    let max_depth = 50;
    renderer
        .render(&camera, &world, samples_per_pixel, max_depth)
        .unwrap();
}

fn random_scene() -> HittableList<Sphere> {
    let mut world = HittableList::new();

    let material_ground = Material::Lambertian(Lambertian::new(dvec3(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        DVec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f32>();
            let center = dvec3(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if (center - dvec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if choose_mat < 0.8 {
                    let albedo = random_color() * random_color();
                    Material::Lambertian(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = random_color_range(0.5, 1.0);
                    let fuzz = random::<f64>() * 0.5;
                    Material::Metal(Metal::new(albedo, fuzz))
                } else {
                    Material::Dialectric(Dialectric::new(1.5))
                };
                world.add(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    let large_sphere_radius = 1.0;
    let glass_material = Material::Dialectric(Dialectric::new(1.5));
    world.add(Box::new(Sphere::new(
        dvec3(0.0, 1.0, 0.0),
        large_sphere_radius,
        glass_material,
    )));

    let diffuse_material = Material::Lambertian(Lambertian::new(dvec3(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        dvec3(-4.0, 1.0, 0.0),
        large_sphere_radius,
        diffuse_material,
    )));

    let metal_material = Material::Metal(Metal::new(dvec3(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        dvec3(4.0, 1.0, 0.0),
        large_sphere_radius,
        metal_material,
    )));

    world
}
