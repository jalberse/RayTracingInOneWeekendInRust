use std::sync::Arc;

use glam::DVec3;

use crate::{
    aabb::Aabb,
    hittable::{Hittable, HittableList},
    materials::material::Material,
};

use super::rectangle::{XyRect, XzRect, YzRect};

pub struct Cube {
    min_point: DVec3,
    max_point: DVec3,
    sides: HittableList,
}

impl Cube {
    pub fn new(min_point: DVec3, max_point: DVec3, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();
        sides.add(Arc::new(XyRect::new(
            min_point.x,
            max_point.x,
            min_point.y,
            max_point.y,
            min_point.z,
            material.clone(),
        )));
        sides.add(Arc::new(XyRect::new(
            min_point.x,
            max_point.x,
            min_point.y,
            max_point.y,
            max_point.z,
            material.clone(),
        )));

        sides.add(Arc::new(XzRect::new(
            min_point.x,
            max_point.x,
            min_point.z,
            max_point.z,
            min_point.y,
            material.clone(),
        )));
        sides.add(Arc::new(XzRect::new(
            min_point.x,
            max_point.x,
            min_point.z,
            max_point.z,
            max_point.y,
            material.clone(),
        )));

        sides.add(Arc::new(YzRect::new(
            min_point.y,
            max_point.y,
            min_point.z,
            max_point.z,
            min_point.x,
            material.clone(),
        )));
        sides.add(Arc::new(YzRect::new(
            min_point.y,
            max_point.y,
            min_point.z,
            max_point.z,
            max_point.x,
            material,
        )));

        Cube {
            min_point,
            max_point,
            sides,
        }
    }
}

impl Hittable for Cube {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<crate::aabb::Aabb> {
        Some(Aabb::new(self.min_point, self.max_point))
    }
}
