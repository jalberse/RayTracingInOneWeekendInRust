use std::sync::{Arc, Mutex};

use ahash::AHashMap;
use glam::Vec3;

use crate::{
    aabb::Aabb,
    bvh::BvhId,
    hittable::{HitRecord, Hittable, HittableList},
    hrpp::Predictor,
    materials::material::Material,
};

use super::rectangle::{XyRect, XzRect, YzRect};

pub struct Cube {
    min_point: Vec3,
    max_point: Vec3,
    sides: HittableList,
}

impl Cube {
    pub fn new(min_point: Vec3, max_point: Vec3, material: Arc<dyn Material>) -> Self {
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
        t_min: f32,
        t_max: f32,
        predictors: &Arc<Option<Mutex<AHashMap<BvhId, Predictor>>>>,
    ) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max, predictors)
    }

    fn bounding_box(&self, _time_0: f32, _time_1: f32) -> Option<crate::aabb::Aabb> {
        Some(Aabb::new(self.min_point, self.max_point))
    }
}
