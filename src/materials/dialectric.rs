use std::ops::Neg;

use glam::dvec3;

use crate::{hittable::HitRecord, ray::Ray};

use super::{
    material::{ScatterRecord, Scatterable},
    utils,
};

#[derive(Clone, Copy)]
pub struct Dialectric {
    pub index_of_refraction: f64,
}

impl Dialectric {
    pub fn new(index_of_refraction: f64) -> Dialectric {
        Dialectric {
            index_of_refraction,
        }
    }
}

impl Scatterable for Dialectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = dvec3(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = ray.direction.normalize();

        let cos_theta = f64::min(unit_direction.neg().dot(hit_record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta.powi(2));

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract {
            utils::reflect(unit_direction, hit_record.normal)
        } else {
            utils::refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit_record.point, direction);
        Some(ScatterRecord {
            attenuation,
            ray: scattered,
        })
    }
}
