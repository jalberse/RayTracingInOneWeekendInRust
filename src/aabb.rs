use glam::DVec3;

use crate::ray::Ray;

const DIMENSIONS: usize = 3;

pub struct Aabb {
    min: DVec3,
    max: DVec3,
}

impl Aabb {
    pub fn new(min: DVec3, max: DVec3) -> Aabb {
        Aabb { min, max }
    }

    pub fn min(&self) -> &DVec3 {
        &self.min
    }

    pub fn max(&self) -> &DVec3 {
        &self.max
    }

    /// Returns true iff the ray intersects the bounding box;
    /// follows Andrew Kensler's hit method.
    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..DIMENSIONS {
            let inv_d = 1.0 / ray.direction[i];
            let t0 = (self.min[i] - ray.origin[i]) * inv_d;
            let t1 = (self.max[i] - ray.origin[i]) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            if t_max < t_min {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use glam::DVec3;

    use crate::ray::Ray;

    use super::Aabb;

    #[test]
    fn hits() {
        let origin = DVec3::ZERO;
        let ray = Ray::new(origin, DVec3::Z);

        let min = DVec3::new(-1.0, -1.0, 1.0);
        let max = DVec3::new(1.0, 1.0, 2.0);

        let aabb = Aabb::new(min, max);

        assert!(aabb.hit(&ray, 0.0, 5.0));
    }

    #[test]
    fn misses() {
        let origin = DVec3::ZERO;
        let ray = Ray::new(origin, DVec3::Z);

        let min = DVec3::new(1.0, 1.0, 1.0);
        let max = DVec3::new(2.0, 2.0, 2.0);

        let aabb = Aabb::new(min, max);

        assert!(!aabb.hit(&ray, 0.0, 5.0));
    }
}
