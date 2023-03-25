use shimmer::bvh::Bvh;
use shimmer::camera::Camera;
use shimmer::geometry::cube::Cube;
use shimmer::geometry::instance::{RotateY, Translate};
use shimmer::geometry::moving_sphere::MovingSphere;
use shimmer::geometry::rectangle::{XyRect, XzRect, YzRect};
use shimmer::geometry::sphere::Sphere;
use shimmer::hittable::{ConstantMedium, HittableList};
use shimmer::materials::diffuse_light::DiffuseLight;
use shimmer::materials::{
    dialectric::Dialectric,
    lambertian::Lambertian,
    material::Material,
    metal::Metal,
    utils::{random_color, random_color_range},
};
use shimmer::renderer::Renderer;
use shimmer::textures::checker::Checker;
use shimmer::textures::image_texture::ImageTexture;

use clap::{Parser, ValueEnum};
use glam::{dvec3, DVec3};

use rand::{random, Rng};
use shimmer::textures::marble::Marble;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

#[derive(ValueEnum, Clone)]
enum Scene {
    RandomSpheres,
    RandomMovingSpheres,
    TwoSpheres,
    Marble,
    Earth,
    SimpleLights,
    Cornell,
    CornellSmoke,
    Showcase,
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(value_enum)]
    scene: Scene,
    /// Image width; image height is determined by this value and the aspect ratio.
    #[arg(short = 'w', long, default_value = "1080")]
    image_width: usize,
    #[arg(short, long, num_args = 2, default_values = vec!["16.0", "9.0"])]
    /// Aspect ratio (horizontal, vertical).
    aspect_ratio: Vec<f64>,
    /// Number of ray samples per pixel.
    #[arg(short, long, default_value = "500")]
    samples_per_pixel: u32,
    /// Maximum number of bounces for each ray.
    #[arg(short, long, default_value = "50")]
    depth: u32,
    /// Width of each render tile, in pixels.
    #[arg(long, default_value = "8")]
    tile_width: usize,
    /// Height of each render tile, in pixels.
    #[arg(long, default_value = "8")]
    tile_height: usize,
    /// x, y, z
    /// Origin of the camera.
    #[arg(long, num_args = 3, allow_negative_numbers=true, default_values = vec!["13.0", "2.0", "3.0"])]
    cam_look_from: Vec<f64>,
    /// x, y, z
    /// Determines direction of camera.
    #[arg(long, num_args = 3, allow_negative_numbers=true, default_values = vec!["0.0", "0.0", "0.0"])]
    cam_look_at: Vec<f64>,
    /// x, y, z
    /// Determines roll of the camera along the vector from cam_look_from to cam_look_at.
    /// Useful for dutch angle shots.
    /// Typically "world up" (0.0, 1.0, 0.0).
    #[arg(long, num_args = 3, allow_negative_numbers=true, default_values = vec!["0.0", "1.0", "0.0"])]
    cam_view_up: Vec<f64>,
    /// Vertical field of view. This also dictates the horizontal FOV according to the aspect ratio.
    #[arg(long, default_value = "20.0")]
    cam_vertical_fov: f64,
    /// Camera aperture; twice the lens radius.
    #[arg(long, default_value = "0.0")]
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
        Scene::Marble => two_marble_spheres(),
        Scene::Earth => earth(),
        Scene::SimpleLights => simple_lights(),
        Scene::Cornell => cornell_box(),
        Scene::CornellSmoke => cornell_smoke(),
        Scene::Showcase => showcase(),
    };

    let background = match cli.scene {
        Scene::SimpleLights => DVec3::ZERO,
        Scene::Cornell => DVec3::ZERO,
        Scene::CornellSmoke => DVec3::ZERO,
        Scene::Showcase => DVec3::ZERO,
        _ => dvec3(0.70, 0.80, 1.00),
    };

    let samples_per_pixel = cli.samples_per_pixel;
    let max_depth = cli.depth;
    renderer
        .render(
            &camera,
            &world,
            background,
            samples_per_pixel,
            max_depth,
            cli.tile_width,
            cli.tile_height,
        )
        .unwrap();

    let duration = start.elapsed();
    eprintln!("Render time: {:?}", duration);
}

fn random_spheres() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Arc::new(Checker::from_color(
        10.0,
        dvec3(0.2, 0.3, 0.1),
        dvec3(0.9, 0.9, 0.9),
    ))));
    world.add(Arc::new(Sphere::new(
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
                let material: Arc<dyn Material> = if choose_mat < 0.8 {
                    let albedo = random_color() * random_color();
                    Arc::new(Lambertian::from_color(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = random_color_range(0.5, 1.0);
                    let fuzz = random::<f64>() * 0.5;
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    Arc::new(Dialectric::new(1.5))
                };
                world.add(Arc::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    let large_sphere_radius = 1.0;
    let glass_material = Arc::new(Dialectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        dvec3(0.0, 1.0, 0.0),
        large_sphere_radius,
        glass_material,
    )));

    let diffuse_material = Arc::new(Lambertian::from_color(dvec3(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        dvec3(-4.0, 1.0, 0.0),
        large_sphere_radius,
        diffuse_material,
    )));

    let metal_material = Arc::new(Metal::new(dvec3(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        dvec3(4.0, 1.0, 0.0),
        large_sphere_radius,
        metal_material,
    )));

    let bvh = Arc::new(Bvh::new(world, 0.0, 1.0));
    let mut world = HittableList::new();
    world.add(bvh);
    world
}

fn random_moving_spheres() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Arc::new(Checker::from_color(
        10.0,
        dvec3(0.2, 0.3, 0.1),
        dvec3(0.9, 0.9, 0.9),
    ))));
    world.add(Arc::new(Sphere::new(
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
                let material: Arc<dyn Material> = if choose_mat < 0.8 {
                    let albedo = random_color() * random_color();
                    Arc::new(Lambertian::from_color(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = random_color_range(0.5, 1.0);
                    let fuzz = random::<f64>() * 0.5;
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    Arc::new(Dialectric::new(1.5))
                };
                let center_end = center + dvec3(0.0, random::<f64>() * 0.5, 0.0);
                world.add(Arc::new(MovingSphere::new(
                    center, center_end, 0.0, 1.0, 0.2, material,
                )));
            }
        }
    }

    let large_sphere_radius = 1.0;
    let glass_material = Arc::new(Dialectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        dvec3(0.0, 1.0, 0.0),
        large_sphere_radius,
        glass_material,
    )));

    let diffuse_material = Arc::new(Lambertian::from_color(dvec3(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        dvec3(-4.0, 1.0, 0.0),
        large_sphere_radius,
        diffuse_material,
    )));

    let metal_material = Arc::new(Metal::new(dvec3(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        dvec3(4.0, 1.0, 0.0),
        large_sphere_radius,
        metal_material,
    )));

    let bvh = Arc::new(Bvh::new(world, 0.0, 1.0));
    let mut world = HittableList::new();
    world.add(bvh);
    world
}

fn two_spheres() -> HittableList {
    let mut world = HittableList::new();
    let checkerboard = Arc::new(Lambertian::new(Arc::new(Checker::from_color(
        10.0,
        dvec3(0.2, 0.3, 0.1),
        dvec3(0.9, 0.9, 0.9),
    ))));

    world.add(Arc::new(Sphere::new(
        dvec3(0.0, -10.0, 0.0),
        10.0,
        checkerboard.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        dvec3(0.0, 10.0, 0.0),
        10.0,
        checkerboard.clone(),
    )));

    world
}

fn two_marble_spheres() -> HittableList {
    let mut world = HittableList::new();

    let marble_texture = Arc::new(Marble::new(4.0));
    world.add(Arc::new(Sphere::new(
        dvec3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(marble_texture.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        dvec3(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(marble_texture)),
    )));
    world
}

// The relative filepath of the image texture means this works if running from the top level of the git repository,
// but not from other working directories (such as if the built app is run elsewhere).
// This is sufficient for now as this executable is just to demo the library for developers.
// Ideally, the image file (and other file resources) would be specified by a scene defined in some file (in JSON, maybe)
// and we wouldn't be defining sample scenes via code like this at all (we would provide sample scenes as separate files
// and would just use Shimmer to parse and render the provided scene).
fn earth() -> HittableList {
    let earth_texture = Arc::new(ImageTexture::new(Path::new("images/earthmap.jpg")));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new(dvec3(0.0, 0.0, 0.0), 2.0, earth_surface));
    let mut world = HittableList::new();
    world.add(globe);
    world
}

fn simple_lights() -> HittableList {
    let mut world = HittableList::new();
    let marble_texture = Arc::new(Marble::new(4.0));
    let ground = Arc::new(Sphere::new(
        dvec3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(marble_texture.clone())),
    ));
    world.add(ground);
    let sphere = Arc::new(Sphere::new(
        dvec3(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(marble_texture)),
    ));
    world.add(sphere);

    let light_mat = Arc::new(DiffuseLight::from_color(dvec3(4.0, 4.0, 4.0)));
    let light = Arc::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, light_mat.clone()));
    world.add(light);

    let sphere_light = Arc::new(Sphere::new(dvec3(0.0, 7.0, 0.0), 2.0, light_mat));
    world.add(sphere_light);

    world
}

fn cornell_box() -> HittableList {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from_color(dvec3(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(dvec3(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(dvec3(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(dvec3(15.0, 15.0, 15.0)));

    world.add(Arc::new(YzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
    )));
    world.add(Arc::new(YzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    )));
    world.add(Arc::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let box1 = Arc::new(Cube::new(
        DVec3::ZERO,
        dvec3(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, dvec3(265.0, 0.0, 295.0)));

    let box2 = Arc::new(Cube::new(
        DVec3::ZERO,
        dvec3(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, dvec3(130.0, 0.0, 65.0)));

    world.add(box1);
    world.add(box2);

    world
}

fn cornell_smoke() -> HittableList {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from_color(dvec3(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(dvec3(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(dvec3(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(dvec3(7.0, 7.0, 7.0)));

    world.add(Arc::new(YzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
    )));
    world.add(Arc::new(YzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    )));
    world.add(Arc::new(XzRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let box1 = Arc::new(Cube::new(
        DVec3::ZERO,
        dvec3(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, dvec3(265.0, 0.0, 295.0)));

    let box2 = Arc::new(Cube::new(
        DVec3::ZERO,
        dvec3(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, dvec3(130.0, 0.0, 65.0)));

    world.add(Arc::new(ConstantMedium::new_with_color(
        box1,
        0.01,
        DVec3::new(0.0, 0.0, 0.0),
    )));
    world.add(Arc::new(ConstantMedium::new_with_color(
        box2,
        0.01,
        DVec3::new(1.0, 1.0, 1.0),
    )));

    world
}

fn showcase() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut boxes = HittableList::new();
    let ground_mat = Arc::new(Lambertian::from_color(dvec3(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;

            boxes.add(Arc::new(Cube::new(
                dvec3(x0, y0, z0),
                dvec3(x1, y1, z1),
                ground_mat.clone(),
            )));
        }
    }

    let mut world = HittableList::new();
    world.add(Arc::new(Bvh::new(boxes, 0.0, 1.0)));

    let light_mat = Arc::new(DiffuseLight::from_color(dvec3(7.0, 7.0, 7.0)));
    world.add(Arc::new(XzRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light_mat,
    )));

    let center1 = dvec3(400.0, 400.0, 200.0);
    let center2 = center1 + dvec3(30.0, 0.0, 0.0);

    let moving_sphere_mat = Arc::new(Lambertian::from_color(dvec3(0.7, 0.3, 0.1)));
    world.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_mat,
    )));

    world.add(Arc::new(Sphere::new(
        dvec3(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dialectric::new(1.5)),
    )));

    world.add(Arc::new(Sphere::new(
        dvec3(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(dvec3(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        dvec3(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dialectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::new_with_color(
        boundary,
        0.2,
        dvec3(0.2, 0.4, 0.9),
    )));

    let boundary = Arc::new(Sphere::new(
        dvec3(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dialectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new_with_color(
        boundary,
        0.0001,
        dvec3(1.0, 1.0, 1.0),
    )));

    let earth_mat = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(Path::new(
        "images/earthmap.jpg",
    )))));
    world.add(Arc::new(Sphere::new(
        dvec3(400.0, 200.0, 400.0),
        100.0,
        earth_mat,
    )));

    let perlin_texture = Arc::new(Marble::new(0.1));
    world.add(Arc::new(Sphere::new(
        dvec3(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(perlin_texture)),
    )));

    let mut spheres = HittableList::new();
    let white_mat = Arc::new(Lambertian::from_color(dvec3(0.73, 0.73, 0.73)));
    let num_spheres = 1000;
    for _ in 0..num_spheres {
        let max_val = 165.0;
        let random_x = rng.gen_range(0.0..max_val);
        let random_y = rng.gen_range(0.0..max_val);
        let random_z = rng.gen_range(0.0..max_val);
        spheres.add(Arc::new(Sphere::new(
            dvec3(random_x, random_y, random_z),
            10.0,
            white_mat.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(Bvh::new(spheres, 0.0, 1.0)), 15.0)),
        dvec3(-100.0, 270.0, 395.0),
    )));

    world
}
