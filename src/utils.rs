use glam::DVec3;

pub fn near_zero(vec: &DVec3) -> bool {
    vec.x.abs() < f64::EPSILON && vec.y.abs() < f64::EPSILON && vec.z.abs() < f64::EPSILON
}
