use shimmer::bvh::Bvh;
use shimmer::camera::Camera;
use shimmer::geometry::moving_sphere::MovingSphere;
use shimmer::geometry::sphere::Sphere;
use shimmer::hittable::HittableList;
use shimmer::materials::{
    dialectric::Dialectric,
    lambertian::Lambertian,
    material::Material,
    metal::Metal,
    utils::{random_color, random_color_range},
};
use shimmer::renderer::Renderer;
use shimmer::textures::checker::Checker;

use clap::{Parser, ValueEnum};
use glam::{dvec3, DVec3};

use rand::random;
use std::rc::Rc;
use std::time::Instant;

#[derive(ValueEnum, Clone)]
enum Scene {
    RandomSpheres,
    RandomMovingSpheres,
    TwoSpheres,
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(value_enum)]
    scene: Scene,
    /// Image width; image height is determined by this value and the aspect ratio.
    #[arg(short = 'w', long, default_value = "1080")]
    image_width: u32,
    #[arg(short, long, num_args = 2, default_values = vec!["16.0", "9.0"])]
    /// Aspect ratio (horizontal, vertical).
    aspect_ratio: Vec<f64>,
    /// Number of ray samples per pixel.
    #[arg(short, long, default_value = "500")]
    samples_per_pixel: u32,
    /// Maximum number of bounces for each ray.
    #[arg(short, long, default_value = "50")]
    depth: u32,
    /// x, y, z
    /// Origin of the camera.
    #[arg(long, num_args = 3, default_values = vec!["13.0", "2.0", "3.0"])]
    cam_look_from: Vec<f64>,
    /// x, y, z
    /// Determines direction of camera.
    #[arg(long, num_args = 3, default_values = vec!["0.0", "0.0", "0.0"])]
    cam_look_at: Vec<f64>,
    /// x, y, z
    /// Determines roll of the camera along the vector from cam_look_from to cam_look_at.
    /// Useful for dutch angle shots.
    /// Typically "world up" (0.0, 1.0, 0.0).
    #[arg(long, num_args = 3, default_values = vec!["0.0", "1.0", "0.0"])]
    cam_view_up: Vec<f64>,
    /// Vertical field of view. This also dictates the horizontal FOV according to the aspect ratio.
    #[arg(long, default_value = "20.0")]
    cam_vertical_fov: f64,
    /// Camera aperture; twice the lens radius.
    #[arg(long, default_value = "0.1")]
    cam_aperture: f64,
    /// Distance to the focal plane from the camera.
    #[arg(long, default_value = "10.0")]
    cam_focus_dist: f64,
    /// Camera shutter open time.
    #[arg(long, default_value = "0.0")]
    cam_start_time: f64,
    /// Camera shutter close time.
    #[arg(long, default_value = "0.0")]
    cam_end_time: f64,
}

fn main() {
    let cli = Cli::parse();

    let aspect_ratio = cli.aspect_ratio;
    let aspect_ratio = aspect_ratio[0] / aspect_ratio[1];
    let look_from = dvec3(
        cli.cam_look_from[0],
        cli.cam_look_from[1],
        cli.cam_look_from[2],
    );
    let look_at = dvec3(cli.cam_look_at[0], cli.cam_look_at[1], cli.cam_look_at[2]);
    let view_up = dvec3(cli.cam_view_up[0], cli.cam_view_up[1], cli.cam_view_up[2]);
    let vfov = cli.cam_vertical_fov;
    let aperture = cli.cam_aperture;
    let focus_dist = cli.cam_focus_dist;
    let cam_start_time = cli.cam_start_time;
    let cam_end_time = cli.cam_end_time;

    let camera = Camera::new(
        look_from,
        look_at,
        view_up,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        cam_start_time,
        cam_end_time,
    );

    let image_width = cli.image_width;
    let renderer = Renderer::from_aspect_ratio(image_width, aspect_ratio);

    let start = Instant::now();

    let world = match cli.scene {
        Scene::RandomSpheres => random_spheres(),
        Scene::RandomMovingSpheres => random_moving_spheres(),
        Scene::TwoSpheres => two_spheres(),
    };

    let samples_per_pixel = cli.samples_per_pixel;
    let max_depth = cli.depth;
    renderer
        .render(&camera, &world, samples_per_pixel, max_depth)
        .unwrap();

    let duration = start.elapsed();
    eprintln!("Render time: {:?}", duration);
}

fn random_spheres() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Rc::new(Checker::from_color(
        10.0,
        dvec3(0.2, 0.3, 0.1),
        dvec3(0.9, 0.9, 0.9),
    ))));
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
                    Rc::new(Lambertian::from_color(albedo))
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

    let diffuse_material = Rc::new(Lambertian::from_color(dvec3(0.4, 0.2, 0.1)));
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

fn random_moving_spheres() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Rc::new(Checker::from_color(
        10.0,
        dvec3(0.2, 0.3, 0.1),
        dvec3(0.9, 0.9, 0.9),
    ))));
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
                    Rc::new(Lambertian::from_color(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = random_color_range(0.5, 1.0);
                    let fuzz = random::<f64>() * 0.5;
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    Rc::new(Dialectric::new(1.5))
                };
                let center_end = center + dvec3(0.0, random::<f64>() * 0.5, 0.0);
                world.add(Rc::new(MovingSphere::new(
                    center, center_end, 0.0, 1.0, 0.2, material,
                )));
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

    let diffuse_material = Rc::new(Lambertian::from_color(dvec3(0.4, 0.2, 0.1)));
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

fn two_spheres() -> HittableList {
    let mut world = HittableList::new();
    let checkerboard = Rc::new(Lambertian::new(Rc::new(Checker::from_color(
        10.0,
        dvec3(0.2, 0.3, 0.1),
        dvec3(0.9, 0.9, 0.9),
    ))));

    world.add(Rc::new(Sphere::new(
        dvec3(0.0, -10.0, 0.0),
        10.0,
        checkerboard.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        dvec3(0.0, 10.0, 0.0),
        10.0,
        checkerboard.clone(),
    )));

    world
}
