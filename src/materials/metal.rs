use glam::DVec3;

use crate::{hittable::HitRecord, ray::Ray};

use super::{
    material::{ScatterRecord, Scatterable},
    utils,
};

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: DVec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: DVec3, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: f64::clamp(fuzz, 0.0, 1.0),
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = utils::reflect(ray.direction.normalize(), hit_record.normal);
        let scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * utils::random_in_unit_sphere(),
        );
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
