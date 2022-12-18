use glam::{dvec3, DVec3};
use palette::Srgb;
use rand::Rng;

pub fn near_zero(vec: &DVec3) -> bool {
    vec.x.abs() < f64::EPSILON && vec.y.abs() < f64::EPSILON && vec.z.abs() < f64::EPSILON
}

pub fn random_in_unit_disk() -> DVec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = dvec3(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn srgb_from_dvec3(vec: DVec3) -> Srgb {
    // Our colors from ray tracing are already in linear rgb space, so
    // we make no conversions.
    Srgb::from_components((vec.x as f32, vec.y as f32, vec.z as f32))
}
