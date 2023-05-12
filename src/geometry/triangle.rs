use std::sync::Arc;

use glam::{vec3, Vec3};

use crate::{
    aabb::Aabb,
    bvh::BvhId,
    hittable::{HitRecord, Hittable},
    hrpp::Predictor,
    materials::material::Material,
};

pub struct Tri {
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    material: Arc<dyn Material>,
}

impl Tri {
    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, material: Arc<dyn Material>) -> Tri {
        Tri {
            p0,
            p1,
            p2,
            material,
        }
    }
}

impl Hittable for Tri {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        _predictors: &Arc<Option<ahash::AHashMap<BvhId, std::sync::Mutex<Predictor>>>>,
    ) -> Option<crate::hittable::HitRecord> {
        // Moller-Trumbore intersection algorithm
        let epsilon = 0.0000001;
        let vertex0 = self.p0;
        let vertex1 = self.p1;
        let vertex2 = self.p2;
        let edge1 = vertex1 - vertex0;
        let edge2 = vertex2 - vertex0;
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        if a > -epsilon && a < epsilon {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - vertex0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(q);

        if t < t_min || t > t_max {
            return None;
        }

        if t > epsilon {
            // TODO We should use barycentric coordinates to get the uvs proper
            //  for the triangle, but for now we'll just give 0,0 for UVs
            //  since I just want to get it working with a solid color lambertian.
            // let intersection_point = ray.origin + ray.direction * t;
            let normal = edge1.cross(edge2).normalize();

            Some(HitRecord::new(
                ray,
                normal,
                t,
                0.0,
                0.0,
                self.material.clone(),
            ))
        } else {
            None
        }
    }

    fn bounding_box(&self, _time_0: f32, _time_1: f32) -> Option<Aabb> {
        // Add/sub epsilon to avoid infinitely-thin boxes for axis-aligned tris.
        let min_x = f32::min(self.p0.x, f32::min(self.p1.x, self.p2.x)) - f32::EPSILON;
        let min_y = f32::min(self.p0.y, f32::min(self.p1.y, self.p2.y)) - f32::EPSILON;
        let min_z = f32::min(self.p0.z, f32::min(self.p1.z, self.p2.z)) - f32::EPSILON;
        let max_x = f32::max(self.p0.x, f32::max(self.p1.x, self.p2.x)) + f32::EPSILON;
        let max_y = f32::max(self.p0.y, f32::max(self.p1.y, self.p2.y)) + f32::EPSILON;
        let max_z = f32::max(self.p0.z, f32::max(self.p1.z, self.p2.z)) + f32::EPSILON;

        Some(Aabb::new(
            vec3(min_x, min_y, min_z),
            vec3(max_x, max_y, max_z),
        ))
    }
}
