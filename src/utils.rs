use glam::{Vec3, vec3};
use palette::Srgb;
use rand::Rng;

pub fn near_zero(vec: &Vec3) -> bool {
    vec.x.abs() < f32::EPSILON && vec.y.abs() < f32::EPSILON && vec.z.abs() < f32::EPSILON
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = vec3(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn srgb_from_vec3(vec: Vec3) -> Srgb {
    // Our colors from ray tracing are already in linear rgb space, so
    // we make no conversions.
    Srgb::from_components((vec.x as f32, vec.y as f32, vec.z as f32))
}
