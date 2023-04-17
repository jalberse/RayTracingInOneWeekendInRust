use glam::Vec3;

use crate::ray::Ray;

const DIMENSIONS: usize = 3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Aabb {
        Aabb { min, max }
    }

    pub fn min(&self) -> &Vec3 {
        &self.min
    }

    pub fn max(&self) -> &Vec3 {
        &self.max
    }

    /// Returns true iff the ray intersects the bounding box;
    /// follows Andrew Kensler's hit method.
    pub fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
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

    pub fn union(box0: &Option<Aabb>, box1: &Option<Aabb>) -> Option<Aabb> {
        match (box0, box1) {
            (None, None) => None,
            (None, Some(box1)) => Some(box1.clone()),
            (Some(box0), None) => Some(box0.clone()),
            (Some(box0), Some(box1)) => {
                let min = Vec3::new(
                    f32::min(box0.min().x, box1.min().x),
                    f32::min(box0.min().y, box1.min().y),
                    f32::min(box0.min().z, box1.min().z),
                );
                let max = Vec3::new(
                    f32::max(box0.max().x, box1.max().x),
                    f32::max(box0.max().y, box1.max().y),
                    f32::max(box0.max().z, box1.max().z),
                );
                Some(Aabb::new(min, max))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::ray::Ray;

    use super::Aabb;

    #[test]
    fn hits() {
        let origin = Vec3::ZERO;
        let ray = Ray::new(origin, Vec3::Z, 0.0);

        let min = Vec3::new(-1.0, -1.0, 1.0);
        let max = Vec3::new(1.0, 1.0, 2.0);

        let aabb = Aabb::new(min, max);

        assert!(aabb.hit(&ray, 0.0, 5.0));
    }

    #[test]
    fn misses() {
        let origin = Vec3::ZERO;
        let ray = Ray::new(origin, Vec3::Z, 0.0);

        let min = Vec3::new(1.0, 1.0, 1.0);
        let max = Vec3::new(2.0, 2.0, 2.0);

        let aabb = Aabb::new(min, max);

        assert!(!aabb.hit(&ray, 0.0, 5.0));
    }

    #[test]
    fn union_nones() {
        assert!(Aabb::union(&None, &None).is_none());
    }

    #[test]
    fn union_box_0_some_other_none() {
        let min = Vec3::new(1.0, 1.0, 1.0);
        let max = Vec3::new(2.0, 2.0, 2.0);
        let aabb = Aabb::new(min, max);

        assert_eq!(Some(aabb), Aabb::union(&Some(aabb), &None));
    }

    #[test]
    fn union_box_1_some_other_none() {
        let min = Vec3::new(1.0, 1.0, 1.0);
        let max = Vec3::new(2.0, 2.0, 2.0);
        let aabb = Aabb::new(min, max);

        assert_eq!(Some(aabb), Aabb::union(&None, &Some(aabb)));
    }

    #[test]
    fn union() {
        let min_0 = Vec3::new(0.0, 1.0, 0.0);
        let max_0 = Vec3::new(2.0, 4.0, 2.0);
        let aabb_0 = Aabb::new(min_0, max_0);

        let min_1 = Vec3::new(1.0, 0.0, 1.0);
        let max_1 = Vec3::new(3.0, 3.0, 3.0);
        let aabb_1 = Aabb::new(min_1, max_1);

        let expected_min = Vec3::new(0.0, 0.0, 0.0);
        let expected_max = Vec3::new(3.0, 4.0, 3.0);
        let expected_aabb = Aabb::new(expected_min, expected_max);

        assert_eq!(
            Some(expected_aabb),
            Aabb::union(&Some(aabb_0), &Some(aabb_1))
        );
    }
}
