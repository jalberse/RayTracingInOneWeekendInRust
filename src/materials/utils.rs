use std::ops::Neg;

use glam::Vec3;
use rand::{random, Rng};

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let vec = Vec3::new(
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
pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}

/// Useful as an alternative diffuse shading approach compared to random_on_unit_sphere()
#[allow(dead_code)]
pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(*normal).is_sign_positive() {
        in_unit_sphere
    } else {
        in_unit_sphere.neg()
    }
}

pub fn reflect(vec: Vec3, normal: Vec3) -> Vec3 {
    vec - 2.0 * vec.dot(normal) * normal
}

pub fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min(uv.neg().dot(normal), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * normal);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * normal;
    r_out_parallel + r_out_perp
}

pub fn random_color() -> Vec3 {
    Vec3::new(random::<f32>(), random::<f32>(), random::<f32>())
}

/// Gets a random color where r, g, b are all bound by min and max (and by 0.0 and 1.0)
pub fn random_color_range(min: f32, max: f32) -> Vec3 {
    let min = f32::max(min, 0.0);
    let max = f32::min(1.0, max);

    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}
