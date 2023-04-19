use std::{cmp::Ordering, sync::Arc};

use rand::Rng;

use crate::{
    aabb::Aabb,
    hittable::{Hittable, HittableList},
};

// Note that there are various crates for e.g. Arena-backed trees (as opposed to Vec-backed trees)
// which e.g. ensure that references are not invalidated when nodes are deleted and so on.
// However, we know that the Bvh will not change once constructed, so this simple approach
// is sufficient for our purposes.

/// The child of a BVH node is either another BVH node, which we store the index of,
/// or a hittable object.
enum Child {
    Index(usize),
    Hittable(Arc<dyn Hittable>),
}

/// A bounding volume hierarchy implemented via a binary tree.
/// The binary tree is maintained in a Vec.
pub struct Bvh {
    root_index: usize,
    nodes: Vec<BvhNode>,
}

impl Bvh {
    pub fn new(list: HittableList, time_0: f32, time_1: f32) -> Bvh {
        // 2n + 1 - num nodes in binary tree for n leaf nodes.
        //   This assumes on object per leaf node, which would be the upper bound
        //   on how many leaf nodes we need.
        let mut nodes = Vec::with_capacity(list.objects.len() * 2 + 1);
        let root_index = BvhNode::new(list, time_0, time_1, &mut nodes);
        Bvh { root_index, nodes }
    }
}

impl Hittable for Bvh {
    fn bounding_box(&self, time_0: f32, time_1: f32) -> Option<Aabb> {
        self.nodes[self.root_index].bounding_box(time_0, time_1)
    }

    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<crate::hittable::HitRecord> {
        // TODO Here, we should handle using a Predictor.
        //   We can have the Predictor as an Option<Predictor> passed into Bvh::new().
        //   If None is supplied, we can just use our normal implementation without skipping.

        self.nodes[self.root_index].hit(ray, t_min, t_max, &self.nodes)
    }
}

pub struct BvhNode {
    parent: Option<usize>,
    // Index in BVH node list
    idx: usize,
    left: Child,
    right: Child,
    bounding_box: Aabb,
}

impl BvhNode {
    pub fn new(
        mut list: HittableList,
        time_0: f32,
        time_1: f32,
        nodes: &mut Vec<BvhNode>,
    ) -> usize {
        BvhNode::new_helper(list.objects.as_mut_slice(), time_0, time_1, nodes)
    }

    // Creates a BvhNode and adds it the nodes list. Returns the index of that BvhNode in the nodes list.
    fn new_helper(
        objects: &mut [Arc<dyn Hittable>],
        time_0: f32,
        time_1: f32,
        nodes: &mut Vec<BvhNode>,
    ) -> usize {
        let mut rng = rand::thread_rng();
        // Random axis on which to divide the objects
        let axis = rng.gen_range(0..=2);
        let comparator = match axis {
            0 => box_compare_x,
            1 => box_compare_y,
            _ => box_compare_z,
        };

        let (left, right): (Child, Child) = match objects.len() {
            1 => (
                Child::Hittable(objects[0].clone()),
                Child::Hittable(objects[0].clone()),
            ),
            2 => {
                if comparator(&objects[0], &objects[1]) == Ordering::Less {
                    (
                        Child::Hittable(objects[0].clone()),
                        Child::Hittable(objects[1].clone()),
                    )
                } else {
                    (
                        Child::Hittable(objects[1].clone()),
                        Child::Hittable(objects[0].clone()),
                    )
                }
            }
            _ => {
                objects.sort_by(comparator);
                let mid = objects.len() / 2;
                let (left_objects, right_objects) = objects.split_at_mut(mid);
                (
                    Child::Index(BvhNode::new_helper(left_objects, time_0, time_1, nodes)),
                    Child::Index(BvhNode::new_helper(right_objects, time_0, time_1, nodes)),
                )
            }
        };

        let left_box = match &left {
            Child::Index(i) => nodes[*i].bounding_box(time_0, time_1),
            Child::Hittable(hittable) => hittable.bounding_box(time_0, time_1),
        };
        let right_box = match &right {
            Child::Index(i) => nodes[*i].bounding_box(time_0, time_1),
            Child::Hittable(hittable) => hittable.bounding_box(time_0, time_1),
        };
        
        let bounding_box = match (left_box, right_box) {
            (Some(left), Some(right)) => Aabb::union(&Some(left), &Some(right)),
            _ => panic!("Missing bounding box in BVH construction"),
        }
        .unwrap();

        // Now that we know the parent's index, we can update the children
        // with that information.
        let new_node_idx = nodes.len();
        match left {
            Child::Index(i) => nodes[i].parent = Some(new_node_idx),
            Child::Hittable(_) => (),
        };
        match right {
            Child::Index(i) => nodes[i].parent = Some(new_node_idx),
            Child::Hittable(_) => (),
        };

        // All nodes are created with no parent initially;
        // when we create the parent node, we'll update its children
        let new_node = BvhNode {
            parent: None,
            idx: new_node_idx,
            left,
            right,
            bounding_box,
        };

        nodes.push(new_node);

        new_node_idx
    }

    fn bounding_box(&self, _time_0: f32, _time_1: f32) -> Option<Aabb> {
        Some(self.bounding_box)
    }

    // We implement hit as a bespoke function for Bvh rather than as a Hittable
    // implementation because we need to pass the nodes list and don't want
    // to change the Hittable::hit() signature. Since we should never use
    // a BvhNode outside of acceleration, that's okay; we can handle it
    // via enumerations.
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        nodes: &[BvhNode],
    ) -> Option<crate::hittable::HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        let mut hit_left = match &self.left {
            Child::Index(i) => nodes[*i].hit(ray, t_min, t_max, nodes),
            Child::Hittable(hittable) => hittable.hit(ray, t_min, t_max),
        };
        let t_max_for_right = if let Some(hit_left) = &hit_left {
            hit_left.t
        } else {
            t_max
        };
        let mut hit_right = match &self.right {
            Child::Index(i) => nodes[*i].hit(ray, t_min, t_max, nodes),
            Child::Hittable(hittable) => hittable.hit(ray, t_min, t_max_for_right),
        };

        if let Some(ref mut hit_record) = hit_left {
            hit_record.parentBvhNode = Some(self.idx);
        }
        if let Some(ref mut hit_record) = hit_right {
            hit_record.parentBvhNode = Some(self.idx);
        }

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

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> std::cmp::Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    match (box_a, box_b) {
        (Some(a), Some(b)) => a.min()[axis].total_cmp(&b.min()[axis]),
        _ => panic!("Missing bounding box in Bvh construction!"),
    }
}

fn box_compare_x(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 0)
}

fn box_compare_y(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 1)
}

fn box_compare_z(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 2)
}
