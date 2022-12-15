use std::ops::Neg;

use glam::DVec3;
use rand::Rng;

pub fn random_in_unit_sphere() -> DVec3 {
    let mut rng = rand::thread_rng();

    loop {
        let vec = DVec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        if vec.length_squared() < 1.0 {
            return vec;
        }
    }
}

/// Useful for lambertian diffuse shading
pub fn random_unit_vector() -> DVec3 {
    random_in_unit_sphere().normalize()
}

/// Useful as an alternative diffuse shading approach compared to random_on_unit_sphere()
#[allow(dead_code)]
pub fn random_in_hemisphere(normal: &DVec3) -> DVec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(*normal).is_sign_positive() {
        in_unit_sphere
    } else {
        in_unit_sphere.neg()
    }
}
