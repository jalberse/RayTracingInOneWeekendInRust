use glam::DVec3;

use crate::{hittable::HitRecord, ray::Ray, utils};

use super::material::{ScatterRecord, Scatterable};

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: DVec3,
}

impl Metal {
    pub fn new(albedo: DVec3) -> Metal {
        Metal { albedo }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = utils::reflect(ray.direction.normalize(), hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected);
        let attenuation = self.albedo;
        if scattered.direction.dot(hit_record.normal) > 0.0 {
            return Some(ScatterRecord {
                attenuation,
                ray: scattered,
            });
        } else {
            return None;
        }
    }
}
