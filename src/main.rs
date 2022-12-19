use shimmer::bvh::Bvh;
use shimmer::camera::Camera;
use shimmer::hittable::HittableList;
use shimmer::materials::{
    dialectric::Dialectric,
    lambertian::Lambertian,
    material::Material,
    metal::Metal,
    utils::{random_color, random_color_range},
};
use shimmer::renderer::Renderer;
use shimmer::sphere::Sphere;

use glam::{dvec3, DVec3};

use rand::random;
use std::rc::Rc;
use std::time::Instant;

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
    let renderer = Renderer::from_aspect_ratio(1200, aspect_ratio);

    let start = Instant::now();

    let world = random_scene();

    let samples_per_pixel = 500;
    let max_depth = 50;
    renderer
        .render(&camera, &world, samples_per_pixel, max_depth)
        .unwrap();

    let duration = start.elapsed();
    eprintln!("Render time: {:?}", duration);
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(dvec3(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
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
                let material: Rc<dyn Material> = if choose_mat < 0.8 {
                    let albedo = random_color() * random_color();
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = random_color_range(0.5, 1.0);
                    let fuzz = random::<f64>() * 0.5;
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    Rc::new(Dialectric::new(1.5))
                };
                world.add(Rc::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    let large_sphere_radius = 1.0;
    let glass_material = Rc::new(Dialectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        dvec3(0.0, 1.0, 0.0),
        large_sphere_radius,
        glass_material,
    )));

    let diffuse_material = Rc::new(Lambertian::new(dvec3(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        dvec3(-4.0, 1.0, 0.0),
        large_sphere_radius,
        diffuse_material,
    )));

    let metal_material = Rc::new(Metal::new(dvec3(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        dvec3(4.0, 1.0, 0.0),
        large_sphere_radius,
        metal_material,
    )));

    let bvh = Rc::new(Bvh::new(world, 0.0, 1.0));
    let mut world = HittableList::new();
    world.add(bvh);
    world
}
