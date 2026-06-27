use crate::entity::EntityPosition;
use crate::entity::pathfinding::perfect::simulation::position_is_simulatable;
use crate::entity::pathfinding::perfect::{
    PerfectControlState, PerfectMotionSimulator, PerfectMotionState,
};
use crate::world::WorldSnapshot;
use spinel_registry::EntityBoundingBox;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerfectPathBudget {
    pub maximum_expanded_states: usize,
    pub maximum_ticks: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerfectPathTermination {
    Success,
    Unreachable,
    Cancelled,
    ExpandedStateBudget,
    RouteDurationBudget,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PerfectPathPlan {
    pub controls: Vec<PerfectControlState>,
    pub final_state: PerfectMotionState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PerfectPathResult {
    pub termination: PerfectPathTermination,
    pub plan: Option<PerfectPathPlan>,
    pub expanded_states: usize,
    pub peak_frontier: usize,
}

pub struct PerfectPathRequest<'a> {
    pub world: &'a WorldSnapshot,
    pub bounding_box: EntityBoundingBox,
    pub start: PerfectMotionState,
    pub destination: EntityPosition,
    pub destination_radius: f64,
    pub budget: PerfectPathBudget,
    pub is_cancelled: &'a dyn Fn() -> bool,
}

#[derive(Default)]
pub struct PerfectPathPlanner {
    simulator: PerfectMotionSimulator,
}

impl PerfectPathPlanner {
    pub fn plan(&self, request: PerfectPathRequest<'_>) -> PerfectPathResult {
        if (request.is_cancelled)() {
            return PerfectPathResult {
                termination: PerfectPathTermination::Cancelled,
                plan: None,
                expanded_states: 0,
                peak_frontier: 0,
            };
        }
        if !position_is_simulatable(request.world, request.start.position, request.bounding_box)
            || !position_is_simulatable(request.world, request.destination, request.bounding_box)
        {
            return search_result(PerfectPathTermination::Unreachable, None, 0, 0);
        }
        let controls = control_palette();
        let mut frontier = BinaryHeap::new();
        let mut states = vec![SearchState {
            motion: request.start,
            parent: None,
            control: PerfectControlState::default(),
            cost: 0,
        }];
        let mut visited = HashSet::new();
        frontier.push(QueuedState::new(
            0,
            heuristic(request.start.position, request.destination),
        ));
        let mut expanded_states = 0usize;
        let mut peak_frontier = 1usize;

        while let Some(queued) = frontier.pop() {
            if (request.is_cancelled)() {
                return search_result(
                    PerfectPathTermination::Cancelled,
                    None,
                    expanded_states,
                    peak_frontier,
                );
            }
            if expanded_states >= request.budget.maximum_expanded_states {
                return search_result(
                    PerfectPathTermination::ExpandedStateBudget,
                    None,
                    expanded_states,
                    peak_frontier,
                );
            }
            let state = states[queued.index].clone();
            if state.cost >= request.budget.maximum_ticks {
                return search_result(
                    PerfectPathTermination::RouteDurationBudget,
                    None,
                    expanded_states,
                    peak_frontier,
                );
            }
            let key = motion_key(state.motion);
            if !visited.insert(key) {
                continue;
            }
            expanded_states += 1;
            if destination_is_reached(
                state.motion.position,
                request.destination,
                request.destination_radius,
            ) {
                return search_result(
                    PerfectPathTermination::Success,
                    Some(reconstruct_plan(&states, queued.index)),
                    expanded_states,
                    peak_frontier,
                );
            }
            controls.iter().for_each(|control| {
                let next_motion = self.simulator.tick(
                    request.world,
                    request.bounding_box,
                    state.motion,
                    *control,
                );
                if !position_is_simulatable(
                    request.world,
                    next_motion.position,
                    request.bounding_box,
                ) {
                    return;
                }
                let index = states.len();
                let cost = state.cost + 1;
                states.push(SearchState {
                    motion: next_motion,
                    parent: Some(queued.index),
                    control: *control,
                    cost,
                });
                frontier.push(QueuedState::new(
                    index,
                    cost as f64 + heuristic(next_motion.position, request.destination),
                ));
            });
            peak_frontier = peak_frontier.max(frontier.len());
        }
        search_result(
            PerfectPathTermination::Unreachable,
            None,
            expanded_states,
            peak_frontier,
        )
    }
}

#[derive(Clone)]
struct SearchState {
    motion: PerfectMotionState,
    parent: Option<usize>,
    control: PerfectControlState,
    cost: u32,
}

#[derive(Clone, Copy)]
struct QueuedState {
    index: usize,
    score: f64,
}

impl QueuedState {
    const fn new(index: usize, score: f64) -> Self {
        Self { index, score }
    }
}

impl PartialEq for QueuedState {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.score.total_cmp(&other.score) == Ordering::Equal
    }
}

impl Eq for QueuedState {}

impl PartialOrd for QueuedState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueuedState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .total_cmp(&self.score)
            .then_with(|| other.index.cmp(&self.index))
    }
}

fn control_palette() -> [PerfectControlState; 9] {
    [
        PerfectControlState::default(),
        PerfectControlState {
            forward: true,
            ..PerfectControlState::default()
        },
        PerfectControlState {
            backward: true,
            ..PerfectControlState::default()
        },
        PerfectControlState {
            left: true,
            ..PerfectControlState::default()
        },
        PerfectControlState {
            right: true,
            ..PerfectControlState::default()
        },
        PerfectControlState {
            forward: true,
            sprint: true,
            ..PerfectControlState::default()
        },
        PerfectControlState {
            forward: true,
            jump: true,
            ..PerfectControlState::default()
        },
        PerfectControlState {
            forward: true,
            sprint: true,
            jump: true,
            ..PerfectControlState::default()
        },
        PerfectControlState {
            forward: true,
            left: true,
            sprint: true,
            jump: true,
            ..PerfectControlState::default()
        },
    ]
}

fn reconstruct_plan(states: &[SearchState], mut index: usize) -> PerfectPathPlan {
    let final_state = states[index].motion;
    let mut controls = Vec::new();
    while let Some(parent) = states[index].parent {
        controls.push(states[index].control);
        index = parent;
    }
    controls.reverse();
    PerfectPathPlan {
        controls,
        final_state,
    }
}

fn search_result(
    termination: PerfectPathTermination,
    plan: Option<PerfectPathPlan>,
    expanded_states: usize,
    peak_frontier: usize,
) -> PerfectPathResult {
    PerfectPathResult {
        termination,
        plan,
        expanded_states,
        peak_frontier,
    }
}

fn destination_is_reached(
    position: EntityPosition,
    destination: EntityPosition,
    radius: f64,
) -> bool {
    position.get_distance_squared(destination) <= radius * radius
}

fn heuristic(position: EntityPosition, destination: EntityPosition) -> f64 {
    position.get_distance_squared(destination).sqrt()
}

fn motion_key(state: PerfectMotionState) -> (i32, i32, i32, i32, i32, i32, bool) {
    (
        (state.position.get_x() * 20.0).round() as i32,
        (state.position.get_y() * 20.0).round() as i32,
        (state.position.get_z() * 20.0).round() as i32,
        (state.velocity.x * 100.0).round() as i32,
        (state.velocity.y * 100.0).round() as i32,
        (state.velocity.z * 100.0).round() as i32,
        state.on_ground,
    )
}
