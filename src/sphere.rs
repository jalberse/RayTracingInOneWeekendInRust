use glam::Vec3;

use crate::{
    hittable::{hittable, HitRecord},
    ray::Ray,
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant.is_sign_negative() {
            return None;
        }
        let sqrt_determinant = f32::sqrt(discriminant);
        let root = (-half_b - sqrt_determinant) / a;
        if root < t_min || root > t_max {
            let root = (-half_b + sqrt_determinant) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let t = root;
        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        Some(HitRecord::new(&ray, normal, t))
    }
}
