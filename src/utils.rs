use glam::DVec3;

pub fn near_zero(vec: &DVec3) -> bool {
    vec.x.abs() < f64::EPSILON && vec.y.abs() < f64::EPSILON && vec.z.abs() < f64::EPSILON
}

pub fn reflect(vec: DVec3, normal: DVec3) -> DVec3 {
    vec - 2.0 * vec.dot(normal) * normal
}
