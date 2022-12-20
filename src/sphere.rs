use std::rc::Rc;

use glam::{dvec3, DVec3};

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    materials::material::Material,
    ray::Ray,
};

pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant.is_sign_negative() {
            return None;
        }
        let sqrt_discriminant = f64::sqrt(discriminant);
        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        Some(HitRecord::new(&ray, normal, t, self.material.clone()))
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<Aabb> {
        let rad = dvec3(self.radius, self.radius, self.radius);
        let bb = Aabb::new(self.center - rad, self.center + rad);
        Some(bb)
    }
}
