use crate::entity::EntityPosition;
use crate::entity::pathfinding::{PathNode, PathNodeType};
use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathState {
    Calculating,
    Following,
    Terminating,
    Terminated,
    Computed,
    BestEffort,
    Invalid,
}

impl PathState {
    const fn id(self) -> u8 {
        match self {
            Self::Calculating => 0,
            Self::Following => 1,
            Self::Terminating => 2,
            Self::Terminated => 3,
            Self::Computed => 4,
            Self::BestEffort => 5,
            Self::Invalid => 6,
        }
    }

    const fn from_id(id: u8) -> Self {
        match id {
            0 => Self::Calculating,
            1 => Self::Following,
            2 => Self::Terminating,
            3 => Self::Terminated,
            4 => Self::Computed,
            5 => Self::BestEffort,
            _ => Self::Invalid,
        }
    }
}

pub struct Path {
    nodes: Vec<PathNode>,
    index: usize,
    state: AtomicU8,
    maximum_distance: f64,
    variance: f64,
    on_complete: Option<Box<dyn FnOnce() + Send>>,
}

impl Path {
    pub fn new(
        maximum_distance: f64,
        variance: f64,
        on_complete: Option<Box<dyn FnOnce() + Send>>,
    ) -> Self {
        Self {
            nodes: Vec::new(),
            index: 0,
            state: AtomicU8::new(PathState::Calculating.id()),
            maximum_distance,
            variance,
            on_complete,
        }
    }

    pub fn get_state(&self) -> PathState {
        PathState::from_id(self.state.load(Ordering::Acquire))
    }

    pub fn set_state(&self, state: PathState) {
        self.state.store(state.id(), Ordering::Release);
    }

    pub fn get_nodes(&self) -> &[PathNode] {
        &self.nodes
    }

    pub fn get_nodes_mut(&mut self) -> &mut Vec<PathNode> {
        &mut self.nodes
    }

    pub(crate) fn set_nodes(&mut self, nodes: Vec<PathNode>) {
        self.nodes = nodes;
        self.index = 0;
    }

    pub(crate) fn remove_nodes_before(&mut self, index: usize) {
        self.nodes.drain(..index);
        self.index = 0;
    }

    pub fn get_current(&self) -> Option<EntityPosition> {
        self.nodes.get(self.index).map(PathNode::get_position)
    }

    pub fn get_next_position(&self) -> Option<EntityPosition> {
        self.nodes.get(self.index + 1).map(PathNode::get_position)
    }

    pub fn get_current_type(&self) -> Option<PathNodeType> {
        self.nodes.get(self.index).map(PathNode::get_node_type)
    }

    pub fn advance(&mut self) {
        if self.index < self.nodes.len() {
            self.index += 1;
        }
    }

    pub const fn get_maximum_distance(&self) -> f64 {
        self.maximum_distance
    }

    pub const fn get_variance(&self) -> f64 {
        self.variance
    }

    pub fn complete(&mut self) {
        if let Some(on_complete) = self.on_complete.take() {
            on_complete();
        }
    }
}

impl Display for Path {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("[")?;
        for (index, node) in self.nodes.iter().enumerate() {
            if index > 0 {
                formatter.write_str(", ")?;
            }
            Display::fmt(node, formatter)?;
        }
        formatter.write_str("]")
    }
}
