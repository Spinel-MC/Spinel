use crate::entity::EntityPosition;
use crate::entity::pathfinding::{NodeGenerator, Path, PathNode, PathNodeType, PathState};
use crate::world::WorldSnapshot;
use spinel_registry::EntityBoundingBox;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

pub struct PathGenerator;

impl PathGenerator {
    pub fn generate(
        world: &WorldSnapshot,
        original_start: EntityPosition,
        original_goal: EntityPosition,
        bounding_box: EntityBoundingBox,
        is_on_ground: bool,
        minimum_distance: f64,
        maximum_distance: f64,
        variance: f64,
        node_generator: &dyn NodeGenerator,
        on_complete: Option<Box<dyn FnOnce() + Send>>,
    ) -> Path {
        let mut path = Path::new(maximum_distance, variance, on_complete);
        let start = if !is_on_ground && node_generator.has_gravity_snap() {
            node_generator
                .gravity_snap(world, original_start, bounding_box, 100)
                .map(|y| {
                    EntityPosition::new(
                        original_start.get_x(),
                        y,
                        original_start.get_z(),
                        original_start.get_yaw(),
                        original_start.get_pitch(),
                    )
                })
                .unwrap_or(original_start)
        } else {
            original_start
        };
        let goal = if node_generator.has_gravity_snap() {
            node_generator
                .gravity_snap(world, original_goal, bounding_box, 100)
                .map(|y| {
                    EntityPosition::new(
                        original_goal.get_x(),
                        y,
                        original_goal.get_z(),
                        original_goal.get_yaw(),
                        original_goal.get_pitch(),
                    )
                })
                .unwrap_or(original_goal)
        } else {
            original_goal
        };
        let minimum_distance = minimum_distance.max(0.8);
        let straight_distance = node_generator.heuristic(start, goal);
        let maximum_nodes = (maximum_distance * 10.0).floor().max(0.0) as usize;
        let start_node = PathNode::new(
            start,
            0.0,
            node_generator.heuristic(start, goal),
            PathNodeType::Walk,
        );
        let mut open = BinaryHeap::from([OpenNode::new(0, start_node.get_score())]);
        let mut nodes = vec![start_node];
        let mut closed = HashSet::new();
        let mut closest_distance = f64::MAX;
        let mut closest = None;

        while !open.is_empty() && closed.len() < maximum_nodes {
            let Some(open_node) = open.pop() else {
                break;
            };
            let current_index = open_node.index;
            let current = &nodes[current_index];
            if current.get_score() - straight_distance > variance
                || start.get_distance_squared(current.get_position())
                    > maximum_distance * maximum_distance
            {
                continue;
            }
            if current.get_position().get_distance_squared(goal)
                < minimum_distance * minimum_distance
            {
                open.push(open_node);
                break;
            }
            if current.get_heuristic() < closest_distance {
                closest_distance = current.get_heuristic();
                closest = Some(current_index);
            }
            node_generator
                .walkable(world, &closed, current, goal, bounding_box)
                .into_iter()
                .for_each(|candidate| {
                    if start.get_distance_squared(candidate.get_position())
                        > maximum_distance * maximum_distance
                    {
                        return;
                    }
                    let candidate_index = nodes.len();
                    open.push(OpenNode::new(candidate_index, candidate.get_score()));
                    closed.insert(candidate.clone());
                    nodes.push(candidate);
                });
        }

        let frontier_destination = open.pop().map(|open_node| open_node.index);
        let reached_destination = frontier_destination.is_some_and(|destination| {
            nodes[destination].get_position().get_distance_squared(goal)
                < minimum_distance * minimum_distance
        });
        let destination_index = if reached_destination {
            frontier_destination
        } else {
            closest
        };
        let Some(destination_index) = destination_index else {
            path.set_state(PathState::Invalid);
            return path;
        };
        let should_repath = !reached_destination && !open.is_empty();
        let mut current = nodes[destination_index].clone();
        if should_repath {
            let mut repath =
                PathNode::new(EntityPosition::default(), 0.0, 0.0, PathNodeType::Repath);
            repath.set_parent(Some(current));
            current = repath;
        }
        let mut path_nodes = Vec::new();
        while let Some(parent) = current.get_parent().cloned() {
            path_nodes.push(current);
            current = parent;
        }
        path_nodes.reverse();
        if path_nodes.first().map(PathNode::get_node_type) == Some(PathNodeType::Repath)
            || path_nodes.is_empty()
        {
            path.set_state(PathState::Invalid);
            return path;
        }
        let reached_goal = path_nodes.last().is_some_and(|node| {
            node.get_position().get_distance_squared(goal) <= minimum_distance * minimum_distance
        });
        if !reached_goal {
            path.set_nodes(path_nodes);
            path.set_state(PathState::BestEffort);
            return path;
        }
        path_nodes.push(PathNode::new(goal, 0.0, 0.0, PathNodeType::Walk));
        path.set_nodes(path_nodes);
        path.set_state(PathState::Computed);
        path
    }
}

struct OpenNode {
    index: usize,
    score: f64,
}

impl OpenNode {
    fn new(index: usize, score: f64) -> Self {
        Self { index, score }
    }

    fn minestom_order(&self, other: &Self) -> Ordering {
        (((self.score - other.score) * 1000.0) as i32).cmp(&0)
    }
}

impl PartialEq for OpenNode {
    fn eq(&self, other: &Self) -> bool {
        self.minestom_order(other) == Ordering::Equal
    }
}

impl Eq for OpenNode {}

impl PartialOrd for OpenNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OpenNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.minestom_order(other) {
            Ordering::Equal => self.index.cmp(&other.index),
            order => order.reverse(),
        }
    }
}
