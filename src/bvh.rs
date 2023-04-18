use std::{cmp::Ordering, sync::Arc};

use rand::Rng;

use crate::{
    aabb::Aabb,
    hittable::{Hittable, HittableList},
};

// TODO - I think we will have an optional Predictor struct that the Bvh has.
//        If it's present, we do the prediction stuff.
//        Otherwise, we do stuff normally.
//        This would all basically be handled within the Bvh::hit() implementation.
//          The current implementation would be the "default", no-prediction case (we'd need to add to the table)
//          Before that we'd do a prediction, and we'd have an Enum with the 4 cases - true positive, false positive, false negative, true negative
//            (though we may not be able to detect false negatives I guess)
//          and if it's e.g. a true positive, we just use that.

// TODO I think the process for adding a new entry to the table will be to add
//      an optional field to the HitRecord, that is the pointer/index to the parent
//      of the hit object in the acceleration structure.
//      So scene objects' hit records will point to the leaf node containing them
//          (which we could just pass as an optional argument to the hit() function,
//           and we'll pass it in the BvhNode::hit() function).
//         BvhNode hit records don't need a pointer to their parent.
//      So maybe it's leafNode: Optional<BvhNode>.
//      BvhNodes themselves will then have an optional pointer to their parent nodes,
//      since we want to traverse up the tree according to the go_up_level,
//      since we don't just store the leaf nodes but some number of layers above the leaf node
//      in the table.
//      But, creating self-referential trees like that is NO BUENO in Rust due to ownership issues.
//      So, we'll have to change to a Vec or Arena based allocations system for the Bvh nodes.
//      Alternatively, the nodes can store a weak reference to their parents (non-owning).
//      But that might have its own issues; I think an Arena or Vec based methods with indices is better.

// Note that there are various crates for e.g. Arena-backed trees (as opposed to Vec-backed trees)
// which e.g. ensure that references are not invalidated when nodes are deleted and so on.
// However, we know that the Bvh will not change once constructed, so this simple approach
// is sufficient for our purposes.

// TODO alright, I think I've got the actual construction down. Just need to fix compilation errors - match
//        and look up node via usize if necessary.
//      I guess the problem is the BvhNode doesn't know about the nodes list, so it can't access it.
//       (it can during construction, since we're passing nodes in, but not for hit fns).
//       So maybe a BvhNode isn't a hittable, and we just move its Hitting logic up to the Bvh hit() function?
//       I think that's the approach we need.

// TODO Once I have gotten it to compile with the Vec backing, A/B test with prior commit to
//       ensure it's working as expected. Then I can start working on the Predictor integration.

/// The child of a BVH node is either another BVH node, which we store the index of,
/// or a hittable object.
enum Child {
    Index(usize),
    Hittable(Arc<dyn Hittable>),
}

pub struct Bvh {
    root_index: usize,
    nodes: Vec<BvhNode>,
}

impl Bvh {
    pub fn new(list: HittableList, time_0: f32, time_1: f32) -> Bvh {
        let mut nodes = Vec::new();
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
        self.nodes[self.root_index].hit(ray, t_min, t_max)
    }
}

pub struct BvhNode {
    // TODO We'll add a parent, but first get it working with a non-pointer based approa ,ch.
    left: Child,
    right: Child,
    bounding_box: Aabb,
}

impl Hittable for BvhNode {
    fn bounding_box(&self, _time_0: f32, _time_1: f32) -> Option<Aabb> {
        Some(self.bounding_box)
    }

    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
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

        let left_box = match left {
            Child::Index(i) => nodes[i].bounding_box(time_0, time_1),
            Child::Hittable(hittable) => hittable.bounding_box(time_0, time_1),
        };
        let right_box = match right {
            Child::Index(i) => nodes[i].bounding_box(time_0, time_1),
            Child::Hittable(hittable) => hittable.bounding_box(time_0, time_1),
        };
        
        let bounding_box = match (left_box, right_box) {
            (Some(left), Some(right)) => Aabb::union(&Some(left), &Some(right)),
            _ => panic!("Missing bounding box in BVH construction"),
        }
        .unwrap();

        let new_node = BvhNode {
            left,
            right,
            bounding_box,
        };

        let new_node_idx = nodes.len();
        nodes.push(new_node);
        new_node_idx
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
