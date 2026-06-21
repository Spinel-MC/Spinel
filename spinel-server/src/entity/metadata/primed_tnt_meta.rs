use crate::entity::metadata::{EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::{BlockState, EntityType, vanilla_world_blocks::Block};
use std::ops::{Deref, DerefMut};

pub struct PrimedTntMeta<'entity> {
    entity_meta: EntityMeta<'entity>,
}

impl<'entity> PrimedTntMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::TNT).then_some(Self { entity_meta })
    }

    pub fn get_fuse_time(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::primed_tnt::get_fuse_time())
        {
            MetadataValue::VarInt(fuse_time) => fuse_time,
            _ => 80,
        }
    }

    pub fn set_fuse_time(&mut self, fuse_time: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::primed_tnt::get_fuse_time(),
            MetadataValue::VarInt(fuse_time),
        );
    }

    pub fn get_block_state(&self) -> BlockState {
        let block_state_id = match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::primed_tnt::get_block_state())
        {
            MetadataValue::BlockState(block_state_id) => block_state_id,
            _ => Block::TNT.state_id(),
        };
        BlockState::from_state_id(block_state_id).unwrap_or_else(|| Block::TNT.default_state())
    }

    pub fn set_block_state(&mut self, block_state: BlockState) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::primed_tnt::get_block_state(),
            MetadataValue::BlockState(block_state.state_id()),
        );
    }
}

impl<'entity> Deref for PrimedTntMeta<'entity> {
    type Target = EntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.entity_meta
    }
}

impl<'entity> DerefMut for PrimedTntMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_meta
    }
}
