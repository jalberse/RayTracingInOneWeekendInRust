use std::{cmp::Ordering, rc::Rc};

use rand::Rng;

use crate::{
    aabb::Aabb,
    hittable::{Hittable, HittableList},
};

pub struct Bvh {
    root: BvhNode,
}

impl Bvh {
    pub fn new(list: HittableList, time_0: f64, time_1: f64) -> Bvh {
        let root = BvhNode::new(list, time_0, time_1);
        Bvh { root }
    }
}

impl Hittable for Bvh {
    fn bounding_box(&self, time_0: f64, time_1: f64) -> Option<Aabb> {
        self.root.bounding_box(time_0, time_1)
    }

    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        self.root.hit(ray, t_min, t_max)
    }
}

struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bounding_box: Aabb,
}

impl Hittable for BvhNode {
    fn bounding_box(&self, _time_0: f64, _time_1: f64) -> Option<Aabb> {
        Some(self.bounding_box)
    }

    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_min, t_max);
        let t_max_for_right = if let Some(hit_left) = &hit_left {
            hit_left.t
        } else {
            t_max
        };
        let hit_right = self.right.hit(ray, t_min, t_max_for_right);

        match (hit_left, hit_right) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(left), Some(right)) => {
                if left.t < right.t {
                    Some(left)
                } else {
                    Some(right)
                }
            }
        }
    }
}

impl BvhNode {
    pub fn new(mut list: HittableList, time_0: f64, time_1: f64) -> BvhNode {
        BvhNode::new_helper(list.objects.as_mut_slice(), time_0, time_1)
    }

    fn new_helper(objects: &mut [Rc<dyn Hittable>], time_0: f64, time_1: f64) -> BvhNode {
        let mut rng = rand::thread_rng();
        // Random axis on which to divide the objects
        let axis = rng.gen_range(0..=2);
        let comparator = match axis {
            0 => box_compare_x,
            1 => box_compare_y,
            _ => box_compare_z,
        };

        let (left, right) = match objects.len() {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => {
                if comparator(&objects[0], &objects[1]) == Ordering::Less {
                    (objects[0].clone(), objects[1].clone())
                } else {
                    (objects[1].clone(), objects[0].clone())
                }
            }
            _ => {
                objects.sort_by(comparator);
                let mid = objects.len() / 2;
                let (left_objects, right_objects) = objects.split_at_mut(mid);
                (
                    Rc::new(BvhNode::new_helper(left_objects, time_0, time_1)) as Rc<dyn Hittable>,
                    Rc::new(BvhNode::new_helper(right_objects, time_0, time_1)) as Rc<dyn Hittable>,
                )
            }
        };

        let left_box = left.bounding_box(time_0, time_1);
        let right_box = right.bounding_box(time_0, time_1);

        let bounding_box = match (left_box, right_box) {
            (Some(left), Some(right)) => Aabb::union(&Some(left), &Some(right)),
            _ => panic!("Missing bounding box in BVH construction"),
        }
        .unwrap();

        BvhNode {
            left,
            right,
            bounding_box,
        }
    }
}

fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> std::cmp::Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    match (box_a, box_b) {
        (Some(a), Some(b)) => a.min()[axis].total_cmp(&b.min()[axis]),
        _ => panic!("Missing bounding box in Bvh construction!"),
    }
}

fn box_compare_x(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 0)
}

fn box_compare_y(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 1)
}

fn box_compare_z(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 2)
}
