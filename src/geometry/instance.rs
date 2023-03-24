use std::{ops::Neg, sync::Arc};

use glam::{dvec3, DVec3};

use crate::{aabb::Aabb, hittable::Hittable, ray::Ray};

pub struct Translate {
    hittable: Arc<dyn Hittable>,
    displacement: DVec3,
}

impl Translate {
    pub fn new(hittable: Arc<dyn Hittable>, displacement: DVec3) -> Self {
        Translate {
            hittable,
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
        let mut hit_record = self.hittable.hit(&offset_ray, t_min, t_max)?;
        hit_record.point += self.displacement;
        Some(hit_record)
    }

    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<crate::aabb::Aabb> {
        let bbox = self.hittable.bounding_box(time_0, time_1)?;

        Some(Aabb::new(
            *bbox.min() + self.displacement,
            *bbox.max() + self.displacement,
        ))
    }
}

pub struct RotateY {
    hittable: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl RotateY {
    pub fn new(hittable: Arc<dyn Hittable>, degrees: f64) -> Self {
        let radians = f64::to_radians(degrees);

        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);

        let bbox = if let Some(bbox) = hittable.bounding_box(0.0, 1.0) {
            let mut min = dvec3(f64::INFINITY, f64::INFINITY, f64::INFINITY);
            let mut max = dvec3(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
            for i in [0.0, 1.0] {
                for j in [0.0, 1.0] {
                    for k in [0.0, 1.0] {
                        let x = i * bbox.max().x + (1.0 - i) * bbox.min().x;
                        let y = j * bbox.max().y + (1.0 - j) * bbox.min().y;
                        let z = k * bbox.max().z + (1.0 - k) * bbox.min().z;

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = sin_theta.neg() * x + cos_theta * z;

                        let tester = dvec3(new_x, y, new_z);

                        for c in 0..2 {
                            min[c] = f64::min(min[c], tester[c]);
                            max[c] = f64::max(max[c], tester[c]);
                        }
                    }
                }
            }
            Some(Aabb::new(min, max))
        } else {
            None
        };

        RotateY {
            hittable,
            sin_theta,
            cos_theta,
            bbox,
        }
    }

    fn get_rotated_dvec(&self, vec: &DVec3) -> DVec3 {
        DVec3::new(
            self.cos_theta * vec[0] - self.sin_theta * vec[2],
            vec[1],
            self.sin_theta * vec[0] + self.cos_theta * vec[2],
        )
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        let origin = self.get_rotated_dvec(&ray.origin);
        let direction = self.get_rotated_dvec(&ray.direction);

        let ray_rotated = Ray::new(origin, direction, ray.time);

        let mut hit_record = self.hittable.hit(&ray_rotated, t_min, t_max)?;

        let point = DVec3::new(
            self.cos_theta * hit_record.point[0] + self.sin_theta * hit_record.point[2],
            hit_record.point[1],
            -self.sin_theta * hit_record.point[0] + self.cos_theta * hit_record.point[2],
        );
        let normal = DVec3::new(
            self.cos_theta * hit_record.normal[0] + self.sin_theta * hit_record.normal[2],
            hit_record.normal[1],
            -self.sin_theta * hit_record.normal[0] + self.cos_theta * hit_record.normal[2],
        );

        hit_record.point = point;
        hit_record.set_face_normal(&ray_rotated, normal);

        Some(hit_record)
    }

    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<Aabb> {
        self.bbox
    }
}
