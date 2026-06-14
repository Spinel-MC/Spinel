use crate::entity::EntityPosition;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PathNodeType {
    Walk,
    Jump,
    Fall,
    Climb,
    ClimbWall,
    Swim,
    Fly,
    Repath,
}

#[derive(Clone, Debug)]
pub struct PathNode {
    position: EntityPosition,
    cost: f64,
    heuristic: f64,
    node_type: PathNodeType,
    parent_coordinates: Option<(i32, i32, i32)>,
}

impl PathNode {
    pub fn new(
        position: EntityPosition,
        cost: f64,
        heuristic: f64,
        node_type: PathNodeType,
    ) -> Self {
        Self {
            position,
            cost,
            heuristic,
            node_type,
            parent_coordinates: None,
        }
    }

    pub const fn position(&self) -> EntityPosition {
        self.position
    }

    pub const fn cost(&self) -> f64 {
        self.cost
    }

    pub const fn heuristic(&self) -> f64 {
        self.heuristic
    }

    pub const fn node_type(&self) -> PathNodeType {
        self.node_type
    }

    pub const fn parent_coordinates(&self) -> Option<(i32, i32, i32)> {
        self.parent_coordinates
    }

    pub fn set_position(&mut self, position: EntityPosition) {
        self.position = position;
    }

    pub fn set_cost(&mut self, cost: f64) {
        self.cost = cost;
    }

    pub fn set_heuristic(&mut self, heuristic: f64) {
        self.heuristic = heuristic;
    }

    pub fn set_node_type(&mut self, node_type: PathNodeType) {
        self.node_type = node_type;
    }

    pub fn set_parent_coordinates(&mut self, parent_coordinates: Option<(i32, i32, i32)>) {
        self.parent_coordinates = parent_coordinates;
    }

    pub fn score(&self) -> f64 {
        self.cost + self.heuristic
    }

    pub fn block_coordinates(&self) -> (i32, i32, i32) {
        (
            self.position.x().floor() as i32,
            self.position.y().floor() as i32,
            self.position.z().floor() as i32,
        )
    }
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.block_coordinates() == other.block_coordinates()
    }
}

impl Eq for PathNode {}

impl Hash for PathNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.block_coordinates().hash(state);
    }
}
