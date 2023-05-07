use std::{
    cmp::Ordering,
    sync::{Arc, Mutex},
};

use ahash::AHashMap;
use rand::Rng;
use uuid::Uuid;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable, HittableList},
    hrpp::Predictor,
};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct BvhId(Uuid);

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
    id: BvhId,
    root_index: usize,
    nodes: Vec<BvhNode>,
}

impl Bvh {
    pub fn new(list: HittableList, time_0: f32, time_1: f32) -> Bvh {
        // 2n + 1 - num nodes in binary tree for n leaf nodes.
        //   This assumes on object per leaf node, which would be the upper bound
        //   on how many leaf nodes we need.
        let mut nodes = Vec::with_capacity(list.objects.len() * 2 + 1);
        let id = BvhId(Uuid::new_v4());
        let root_index = BvhNode::new(list, time_0, time_1, &mut nodes);
        Bvh {
            id,
            root_index,
            nodes,
        }
    }

    /// Creates a BVH from the *list*, and creates a predictor for the BVH,
    /// adding it to the *predictors*.
    /// The predictors are stored separately from the BVH, as they must be modified
    /// at render-time across threads, requiring them to be locked behind a mutex.
    /// The predictors can be accessed by the ID of the BHV, assigned during construction.
    pub fn with_predictor(
        list: HittableList,
        time_0: f32,
        time_1: f32,
        predictors: &mut Arc<Mutex<AHashMap<BvhId, Predictor>>>,
    ) -> Bvh {
        let bvh = Bvh::new(list, time_0, time_1);
        let predictor = Predictor::new();
        let mut data = predictors.lock().unwrap();
        data.insert(bvh.id, predictor);
        bvh
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
        predictors: &Arc<Option<AHashMap<BvhId, Mutex<Predictor>>>>,
    ) -> Option<HitRecord> {
        // Get the predictor, if the set of predictors is supplied and if this BVH has a predictor in the set.
        let this_bvh_predictor_maybe = match predictors.as_ref() {
            Some(predictor_map) => predictor_map.get(&self.id),
            None => None,
        };

        if let Some(predictor_mtx) = this_bvh_predictor_maybe {
            let predictor = predictor_mtx.lock().unwrap();
            let predicted_node_idx = predictor.get_prediction(ray);
            drop(predictor);

            if let Some(predicted_node_idx) = predicted_node_idx {
                // We have a prediction for this ray.
                let hit_record =
                    self.nodes[predicted_node_idx].hit(ray, t_min, t_max, &self.nodes, &predictors);
                if let Some(hit_record) = hit_record {
                    // A true postive - the ray DID hit something within the predicted node.
                    // This is the best case outcome - we can use this result, thereby skipping traversal up to the predicted node.
                    // This case can result in the wrong visual output, however, where the ray does not find the closest intersection
                    // that may lie in a different node. See 4.3 of https://arxiv.org/abs/1910.01304
                    return Some(hit_record);
                } else {
                    // A false positive - the ray did not hit anything within the predicted node.
                    // Go back and traverse the tree from the root.
                    // A replacement policy here instead might improve HRPP performance.
                    return self.nodes[self.root_index].hit(
                        ray,
                        t_min,
                        t_max,
                        &self.nodes,
                        predictors,
                    );
                }
            } else {
                // No prediction for this ray.
                // Find a hit_record via regular traversal, and then add a prediction to the table for this ray.

                // Return if no hit; we won't make a prediction if no geometry is hit.
                let hit_record =
                    self.nodes[self.root_index].hit(ray, t_min, t_max, &self.nodes, &predictors)?;

                // TODO remove parent bvh node from the hit record.
                // The hit record should only contain information about final hit for rendering.
                // Instead, the parent should be stored with the Child::Hittable (in a new struct that holds the parent index
                // and the Arc<dyn Hittable>.
                // We will populate that just like we populate for Child::Index (should be renamed to Child::Node I think?)
                // We will then pass up the parent index alongside the HitRecord, from wherever we hit a Child::Hittable.
                assert!(hit_record.parent_bvh_node.is_some());
                let leaf_node_idx = hit_record.parent_bvh_node?;

                // HRPP’s go up level as the level in the acceleration structure tree the predictor table predicts.
                // A Go Up Level of 0 predicts the acceleration structure’s leaf nodes.
                // A Go Up Level of 1 predicts the parent node of the leaf nodes.
                // A Go Up Level of 2 predicts the grand-parent node of the leaf nodes, etc
                // TODO: The original HRPP paper shows a Go Up Level of 1 is most efficient, so we will hardcode it here,
                // but in the future it might become configurable.
                let go_up_level = 1;
                let predicted_node_idx = {
                    let mut cur_node_idx = leaf_node_idx;
                    for _ in 0..go_up_level {
                        assert!(self.nodes[leaf_node_idx].parent.is_some());
                        cur_node_idx = self.nodes[leaf_node_idx].parent?;
                    }
                    cur_node_idx
                };

                let mut predictor = predictor_mtx.lock().unwrap();
                predictor.insert(&ray, predicted_node_idx);
                drop(predictor);
            }

            todo!()
        } else {
            // No predictor for this BVH. Simply traverse the tree and get the result.
            self.nodes[self.root_index].hit(ray, t_min, t_max, &self.nodes, &predictors)
        }
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
        predictors: &Arc<Option<AHashMap<BvhId, Mutex<Predictor>>>>,
    ) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        let mut hit_left = match &self.left {
            Child::Index(i) => nodes[*i].hit(ray, t_min, t_max, nodes, &predictors),
            Child::Hittable(hittable) => hittable.hit(ray, t_min, t_max, &predictors),
        };
        let t_max_for_right = if let Some(hit_left) = &hit_left {
            hit_left.t
        } else {
            t_max
        };
        let mut hit_right = match &self.right {
            Child::Index(i) => nodes[*i].hit(ray, t_min, t_max, nodes, &predictors),
            Child::Hittable(hittable) => hittable.hit(ray, t_min, t_max_for_right, &predictors),
        };

        if let Some(ref mut hit_record) = hit_left {
            hit_record.parent_bvh_node = Some(self.idx);
        }
        if let Some(ref mut hit_record) = hit_right {
            hit_record.parent_bvh_node = Some(self.idx);
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
