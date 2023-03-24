use std::sync::Arc;

use glam::{dvec3, DVec3};

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    materials::material::Material,
};

pub struct XyRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    z: f64,
    material: Arc<dyn Material>,
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, z: f64, material: Arc<dyn Material>) -> XyRect {
        XyRect {
            x0,
            x1,
            y0,
            y1,
            z,
            material,
        }
    }
}

impl Hittable for XyRect {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        let t = (self.z - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = DVec3::Z;
        Some(HitRecord::new(
            ray,
            outward_normal,
            t,
            u,
            v,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<crate::aabb::Aabb> {
        // The bounding box cannot have an infinitely small side, so we add epsilon.
        Some(Aabb::new(
            dvec3(self.x0, self.y0, self.z - f64::EPSILON),
            dvec3(self.x1, self.y1, self.z + f64::EPSILON),
        ))
    }
}

pub struct XzRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    y: f64,
    material: Arc<dyn Material>,
}

impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, y: f64, material: Arc<dyn Material>) -> XzRect {
        XzRect {
            x0,
            x1,
            z0,
            z1,
            y,
            material,
        }
    }
}

impl Hittable for XzRect {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        let t = (self.y - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = DVec3::Y;
        Some(HitRecord::new(
            ray,
            outward_normal,
            t,
            u,
            v,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<crate::aabb::Aabb> {
        // The bounding box cannot have an infinitely small side, so we add epsilon.
        Some(Aabb::new(
            dvec3(self.x0, self.y - f64::EPSILON, self.z0),
            dvec3(self.x1, self.y + f64::EPSILON, self.z1),
        ))
    }
}

pub struct YzRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    x: f64,
    material: Arc<dyn Material>,
}

impl YzRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, x: f64, material: Arc<dyn Material>) -> YzRect {
        YzRect {
            y0,
            y1,
            z0,
            z1,
            x,
            material,
        }
    }
}

impl Hittable for YzRect {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        let t = (self.x - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = DVec3::X;
        Some(HitRecord::new(
            ray,
            outward_normal,
            t,
            u,
            v,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<crate::aabb::Aabb> {
        // The bounding box cannot have an infinitely small side, so we add epsilon.
        Some(Aabb::new(
            dvec3(self.x - f64::EPSILON, self.y0, self.z0),
            dvec3(self.x + f64::EPSILON, self.y1, self.z1),
        ))
    }
}
