use crate::entity::metadata::{AbstractDisplayMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::{BlockState, EntityType, vanilla_world_blocks::Block};
use std::ops::{Deref, DerefMut};

pub struct BlockDisplayMeta<'entity> {
    abstract_display_meta: AbstractDisplayMeta<'entity>,
}

impl<'entity> BlockDisplayMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::BLOCK_DISPLAY).then(|| Self {
            abstract_display_meta: AbstractDisplayMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_block_state(&self) -> BlockState {
        let state_id = match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::block_display::displayed_block_state())
        {
            MetadataValue::BlockState(state_id) => state_id,
            _ => 0,
        };
        BlockState::from_state_id(state_id).unwrap_or_else(|| Block::AIR.default_state())
    }

    pub fn set_block_state(&mut self, block_state: BlockState) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::block_display::displayed_block_state(),
            MetadataValue::BlockState(block_state.state_id()),
        );
    }
}

impl<'entity> Deref for BlockDisplayMeta<'entity> {
    type Target = AbstractDisplayMeta<'entity>;
    fn deref(&self) -> &Self::Target {
        &self.abstract_display_meta
    }
}

impl<'entity> DerefMut for BlockDisplayMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_display_meta
    }
}
