use crate::{
    geometry::{Aabb, Cube, SurfaceHit},
    math::Ray,
};

const LEAF_SIZE: usize = 4;
const MAX_STACK_DEPTH: usize = 64;

#[derive(Debug, Clone)]
pub(super) struct Bvh {
    nodes: Vec<BvhNode>,
    cube_indices: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
struct BvhNode {
    bounds: Aabb,
    start: usize,
    count: usize,
    left: usize,
    right: usize,
}

impl BvhNode {
    fn placeholder(bounds: Aabb) -> Self {
        Self {
            bounds,
            start: 0,
            count: 0,
            left: 0,
            right: 0,
        }
    }

    fn is_leaf(self) -> bool {
        self.count > 0
    }
}

impl Bvh {
    pub(super) fn build(cubes: &[Cube]) -> Option<Self> {
        let first = cubes.first()?;
        let mut bvh = Self {
            nodes: Vec::with_capacity(cubes.len() * 2),
            cube_indices: (0..cubes.len()).collect(),
        };
        let root_bounds = cubes
            .iter()
            .skip(1)
            .fold(first.bounds, |bounds, cube| bounds.union(&cube.bounds));
        bvh.build_node(cubes, 0, cubes.len(), root_bounds);
        Some(bvh)
    }

    pub(super) fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub(super) fn intersect(
        &self,
        cubes: &[Cube],
        ray: &Ray,
        min_distance: f32,
        max_distance: f32,
    ) -> Option<(usize, SurfaceHit)> {
        if self.nodes.is_empty() {
            return None;
        }
        let mut closest_distance = max_distance;
        let mut closest = None;
        let mut stack = [0_usize; MAX_STACK_DEPTH];
        let mut stack_len = 1;

        while stack_len > 0 {
            stack_len -= 1;
            let node = self.nodes[stack[stack_len]];
            if node
                .bounds
                .entry_distance(ray, min_distance, closest_distance)
                .is_none()
            {
                continue;
            }

            if node.is_leaf() {
                for &cube_index in &self.cube_indices[node.start..node.start + node.count] {
                    if let Some(surface) =
                        cubes[cube_index].intersect(ray, min_distance, closest_distance)
                    {
                        let replaces_tie = (surface.distance - closest_distance).abs() < 1.0e-6
                            && closest
                                .is_none_or(|(previous_index, _)| cube_index > previous_index);
                        if surface.distance < closest_distance || replaces_tie {
                            closest_distance = surface.distance;
                            closest = Some((cube_index, surface));
                        }
                    }
                }
            } else {
                let left_distance = self.nodes[node.left].bounds.entry_distance(
                    ray,
                    min_distance,
                    closest_distance,
                );
                let right_distance = self.nodes[node.right].bounds.entry_distance(
                    ray,
                    min_distance,
                    closest_distance,
                );

                match (left_distance, right_distance) {
                    (Some(left), Some(right)) if left <= right => {
                        push(&mut stack, &mut stack_len, node.right);
                        push(&mut stack, &mut stack_len, node.left);
                    }
                    (Some(_), Some(_)) => {
                        push(&mut stack, &mut stack_len, node.left);
                        push(&mut stack, &mut stack_len, node.right);
                    }
                    (Some(_), None) => push(&mut stack, &mut stack_len, node.left),
                    (None, Some(_)) => push(&mut stack, &mut stack_len, node.right),
                    (None, None) => {}
                }
            }
        }

        closest
    }

    fn build_node(&mut self, cubes: &[Cube], start: usize, end: usize, bounds: Aabb) -> usize {
        let node_index = self.nodes.len();
        self.nodes.push(BvhNode::placeholder(bounds));
        let count = end - start;
        if count <= LEAF_SIZE {
            self.nodes[node_index].start = start;
            self.nodes[node_index].count = count;
            return node_index;
        }

        let extent = bounds.extent();
        let axis = if extent.x >= extent.y && extent.x >= extent.z {
            0
        } else if extent.y >= extent.z {
            1
        } else {
            2
        };
        self.cube_indices[start..end].sort_unstable_by(|left, right| {
            cubes[*left].bounds.center()[axis].total_cmp(&cubes[*right].bounds.center()[axis])
        });
        let middle = start + count / 2;
        let left_bounds = range_bounds(cubes, &self.cube_indices[start..middle]);
        let right_bounds = range_bounds(cubes, &self.cube_indices[middle..end]);
        let left = self.build_node(cubes, start, middle, left_bounds);
        let right = self.build_node(cubes, middle, end, right_bounds);
        self.nodes[node_index].left = left;
        self.nodes[node_index].right = right;
        node_index
    }
}

fn range_bounds(cubes: &[Cube], indices: &[usize]) -> Aabb {
    let first = cubes[indices[0]].bounds;
    indices[1..]
        .iter()
        .fold(first, |bounds, &index| bounds.union(&cubes[index].bounds))
}

fn push(stack: &mut [usize; MAX_STACK_DEPTH], length: &mut usize, node: usize) {
    debug_assert!(*length < MAX_STACK_DEPTH, "BVH stack capacity exceeded");
    if *length < MAX_STACK_DEPTH {
        stack[*length] = node;
        *length += 1;
    }
}
