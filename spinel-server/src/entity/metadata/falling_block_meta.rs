use crate::entity::metadata::{EntityMeta, definitions};
use spinel_network::types::Position;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::{BlockState, EntityType, vanilla_world_blocks::Block};
use std::ops::{Deref, DerefMut};

pub struct FallingBlockMeta<'entity> {
    entity_meta: EntityMeta<'entity>,
}

impl<'entity> FallingBlockMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::FALLING_BLOCK)
            .then_some(Self { entity_meta })
    }

    pub fn get_spawn_position(&self) -> Position {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::falling_block::get_spawn_position())
        {
            MetadataValue::Position(spawn_position) => spawn_position,
            _ => Position { x: 0, y: 0, z: 0 },
        }
    }

    pub fn set_spawn_position(&mut self, spawn_position: Position) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::falling_block::get_spawn_position(),
            MetadataValue::Position(spawn_position),
        );
    }

    pub fn get_block(&self) -> BlockState {
        BlockState::from_state_id(self.get_entity().get_falling_block_state())
            .unwrap_or_else(|| Block::STONE.default_state())
    }

    pub fn set_block(&mut self, block_state: BlockState) {
        self.get_entity_mut()
            .set_falling_block_state(block_state.state_id());
    }
}

impl<'entity> Deref for FallingBlockMeta<'entity> {
    type Target = EntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.entity_meta
    }
}

impl<'entity> DerefMut for FallingBlockMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_meta
    }
}
