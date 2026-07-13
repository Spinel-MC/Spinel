use crate::entity::EntityPosition;
use spinel_network::types::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub struct PlayerDeathLocation {
    dimension: Identifier,
    position: EntityPosition,
}

impl PlayerDeathLocation {
    pub fn new(dimension: Identifier, position: EntityPosition) -> Self {
        Self {
            dimension,
            position,
        }
    }

    pub fn get_dimension(&self) -> &Identifier {
        &self.dimension
    }

    pub const fn get_position(&self) -> EntityPosition {
        self.position
    }
}
