use crate::entity::EntityPosition;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

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
    parent: Option<Arc<PathNode>>,
    identity_hash: i32,
}

impl PathNode {
    pub fn new(
        position: EntityPosition,
        cost: f64,
        heuristic: f64,
        node_type: PathNodeType,
    ) -> Self {
        let identity_hash = position_identity_hash(position);
        Self {
            position,
            cost,
            heuristic,
            node_type,
            parent: None,
            identity_hash,
        }
    }

    pub const fn get_position(&self) -> EntityPosition {
        self.position
    }

    pub const fn get_cost(&self) -> f64 {
        self.cost
    }

    pub const fn get_heuristic(&self) -> f64 {
        self.heuristic
    }

    pub const fn get_node_type(&self) -> PathNodeType {
        self.node_type
    }

    pub fn get_parent(&self) -> Option<&PathNode> {
        self.parent.as_deref()
    }

    pub fn get_parent_coordinates(&self) -> Option<(i32, i32, i32)> {
        self.get_parent().map(PathNode::block_coordinates)
    }

    pub fn set_position(&mut self, position: EntityPosition) {
        self.position = position;
        self.identity_hash = position_identity_hash(position);
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

    pub fn set_parent(&mut self, parent: Option<PathNode>) {
        self.parent = parent.map(Arc::new);
    }

    pub fn get_score(&self) -> f64 {
        self.cost + self.heuristic
    }

    pub fn get_block_coordinates(&self) -> (i32, i32, i32) {
        (
            self.position.get_x().floor() as i32,
            self.position.get_y().floor() as i32,
            self.position.get_z().floor() as i32,
        )
    }
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.identity_hash == other.identity_hash
    }
}

impl Eq for PathNode {}

impl Hash for PathNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identity_hash.hash(state);
    }
}

impl Display for PathNode {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let node_type = match self.node_type {
            PathNodeType::Walk => "WALK",
            PathNodeType::Jump => "JUMP",
            PathNodeType::Fall => "FALL",
            PathNodeType::Fly => "FLY",
            PathNodeType::Climb => "CLIMB",
            PathNodeType::ClimbWall => "CLIMB_WALL",
            PathNodeType::Swim => "SWIM",
            PathNodeType::Repath => "REPATH",
        };
        write!(
            formatter,
            "PNode{{point={}, {}, {}, d={}, type={}}}",
            self.position.get_x(),
            self.position.get_y(),
            self.position.get_z(),
            self.get_score(),
            node_type
        )
    }
}

fn position_identity_hash(position: EntityPosition) -> i32 {
    cantor(
        position.get_x().floor() as i32,
        cantor(position.get_y().floor() as i32, position.get_z().floor() as i32),
    )
}

fn cantor(first: i32, second: i32) -> i32 {
    let mapped_first = if first >= 0 {
        first.wrapping_mul(2)
    } else {
        first.wrapping_mul(-2).wrapping_sub(1)
    };
    let mapped_second = if second >= 0 {
        second.wrapping_mul(2)
    } else {
        second.wrapping_mul(-2).wrapping_sub(1)
    };
    let sum = mapped_first.wrapping_add(mapped_second);
    sum.wrapping_add(1)
        .wrapping_mul(sum)
        .wrapping_div(2)
        .wrapping_add(mapped_second)
}
