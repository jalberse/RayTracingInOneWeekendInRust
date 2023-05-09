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

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub struct BvhId(Uuid);

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct LeafNodeIdx(usize);

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
        predictors: &mut AHashMap<BvhId, Mutex<Predictor>>,
    ) -> Bvh {
        let bvh = Bvh::new(list, time_0, time_1);

        let predictor = Mutex::new(Predictor::new(bvh.id));
        predictors.insert(bvh.id, predictor);

        bvh
    }

    // Goes up the tree from the specified node, go_up_level times
    // If the top of the tree is reached, returns the top of the tree
    fn go_up_level(&self, start_node: usize, go_up_level: u32) -> usize {
        // HRPP’s go up level as the level in the acceleration structure tree the predictor table predicts.
        // A Go Up Level of 0 predicts the acceleration structure’s leaf nodes.
        // A Go Up Level of 1 predicts the parent node of the leaf nodes.
        // A Go Up Level of 2 predicts the grand-parent node of the leaf nodes, etc
        let mut cur_node_idx = start_node;
        for _ in 0..go_up_level {
            if let Some(parent) = self.nodes[cur_node_idx].parent {
                cur_node_idx = parent;
            } else {
                return cur_node_idx;
            }
        }
        cur_node_idx
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
            let predicted_node_idx = predictor.get_predictions(ray).cloned();
            drop(predictor);

            if let Some(predicted_node_indices) = predicted_node_idx {
                // We have a prediction(s) for this ray.
                // Find the closest hit within the predicted nodes.

                let mut closest_so_far = t_max;
                let mut closest_hit_record_and_leaf_node = None;
                for predicted_index in predicted_node_indices.into_iter() {
                    let hit_record_and_leaf_node = self.nodes[predicted_index].hit(
                        ray,
                        t_min,
                        closest_so_far,
                        &self.nodes,
                        &predictors,
                    );
                    if let Some(hit_record_and_leaf_node) = hit_record_and_leaf_node {
                        closest_so_far = hit_record_and_leaf_node.0.t;
                        closest_hit_record_and_leaf_node = Some(hit_record_and_leaf_node);
                    }
                }

                if let Some(hit_record_and_leaf_node) = closest_hit_record_and_leaf_node {
                    // A true postive - the ray DID hit something within the predicted node(s).
                    // This is the best case outcome - we can use this result, thereby skipping traversal up to the predicted node.
                    // This case can result in the wrong visual output, however, where the ray does not find the closest intersection
                    // that may lie in a different node. See 4.3 of https://arxiv.org/abs/1910.01304

                    // Update stats
                    let mut predictor = predictor_mtx.lock().unwrap();
                    predictor.true_positive_predictions += 1;
                    drop(predictor);

                    return Some(hit_record_and_leaf_node.0);
                } else {
                    // A false positive - the ray did not hit anything within the predicted node(s).
                    // Go back and traverse the tree from the root.
                    // A replacement policy here instead might improve HRPP performance.

                    // Update stats
                    let mut predictor = predictor_mtx.lock().unwrap();
                    predictor.false_positive_predictions += 1;
                    drop(predictor);

                    let hit_rec_and_leaf_node =
                        self.nodes[self.root_index].hit(ray, t_min, t_max, &self.nodes, predictors);

                    return match hit_rec_and_leaf_node {
                        Some(hit_rec_and_leaf_node) => {
                            let (_, leaf_node) = hit_rec_and_leaf_node;

                            let predicted_node_idx = self.go_up_level(leaf_node.0, 0);

                            // Add the predicted node to the table
                            let mut predictor = predictor_mtx.lock().unwrap();
                            predictor.insert(ray, predicted_node_idx);
                            drop(predictor);

                            Some(hit_rec_and_leaf_node.0)
                        }
                        None => None,
                    };
                }
            } else {
                // No prediction for this ray.
                // Find a hit_record via regular traversal, and then add a prediction to the table for this ray.

                // update stats
                let mut predictor = predictor_mtx.lock().unwrap();
                predictor.no_predictions += 1;
                drop(predictor);

                // Return if no hit; we won't make a prediction if no geometry is hit.
                let (hit_record, leaf_node_idx) =
                    self.nodes[self.root_index].hit(ray, t_min, t_max, &self.nodes, &predictors)?;

                // We will return the hit record, but first add a prediction to the table for this ray.

                // Get the prediction index
                assert!(self.nodes[leaf_node_idx.0].parent.is_some());
                let predicted_node_idx = self.go_up_level(leaf_node_idx.0, 0);

                // Insert prediction into table
                let mut predictor = predictor_mtx.lock().unwrap();
                predictor.insert(&ray, predicted_node_idx);
                drop(predictor);

                return Some(hit_record);
            }
        } else {
            // No predictor for this BVH. Simply traverse the tree and get the result.
            let (hit_record, _) =
                self.nodes[self.root_index].hit(ray, t_min, t_max, &self.nodes, &predictors)?;
            Some(hit_record)
        }
    }
}

impl Drop for Bvh {
    fn drop(&mut self) {
        eprintln!("BVH id: {}", self.id.0);
        eprintln!("BVH height: {}", self.nodes.len().ilog2());
        eprintln!("\n")
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
    /// Returns the hit record from traversing down the BVH, as well as the index of
    /// the leaf node that was traversed to within this BVH.
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        nodes: &[BvhNode],
        predictors: &Arc<Option<AHashMap<BvhId, Mutex<Predictor>>>>,
    ) -> Option<(HitRecord, LeafNodeIdx)> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        let hit_left = match &self.left {
            Child::Index(i) => nodes[*i].hit(ray, t_min, t_max, nodes, &predictors),
            Child::Hittable(hittable) => {
                // If this is a Child::Hittable, we need to know which leaf node it is under.
                // This will let us walk up the tree for the Predictor in Bvh::hit().
                let hit_record = hittable.hit(ray, t_min, t_max, &predictors);
                if let Some(hit_record) = hit_record {
                    Some((hit_record, LeafNodeIdx(self.idx)))
                } else {
                    None
                }
            }
        };
        let t_max_for_right = if let Some(hit_left) = &hit_left {
            hit_left.0.t
        } else {
            t_max
        };
        let hit_right = match &self.right {
            Child::Index(i) => nodes[*i].hit(ray, t_min, t_max, nodes, &predictors),
            Child::Hittable(hittable) => {
                let hit_record = hittable.hit(ray, t_min, t_max_for_right, &predictors);
                if let Some(hit_record) = hit_record {
                    Some((hit_record, LeafNodeIdx(self.idx)))
                } else {
                    None
                }
            }
        };

        match (hit_left, hit_right) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(left), Some(right)) => {
                if left.0.t < right.0.t {
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
