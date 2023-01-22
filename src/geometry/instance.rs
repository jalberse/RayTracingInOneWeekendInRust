use std::sync::Arc;

use glam::DVec3;

use crate::{aabb::Aabb, hittable::Hittable, ray::Ray};

pub struct Translate {
    object: Arc<dyn Hittable>,
    displacement: DVec3,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, displacement: DVec3) -> Self {
        Translate {
            object,
            displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        let offset_ray = Ray::new(ray.origin - self.displacement, ray.direction, ray.time);
        let mut hit_record = self.object.hit(&offset_ray, t_min, t_max)?;
        hit_record.point += self.displacement;
        Some(hit_record)
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<crate::aabb::Aabb> {
        let bbox = self.object.bounding_box(time_0, time_1)?;

        Some(Aabb::new(
            *bbox.min() + self.displacement,
            *bbox.max() + self.displacement,
        ))
    }
}
