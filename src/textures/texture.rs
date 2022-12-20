use glam::DVec3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &DVec3) -> DVec3;
}
