use crate::entity::metadata::{EntityMeta, MonsterMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::{BlockState, EntityType};
use std::ops::{Deref, DerefMut};

pub struct EndermanMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> EndermanMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::ENDERMAN).then(|| Self {
            monster_meta: MonsterMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_carried_block(&self) -> Option<BlockState> {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::enderman::get_carried_block())
        {
            MetadataValue::OptionalBlockState(0) => None,
            MetadataValue::OptionalBlockState(block_state_id) => {
                BlockState::from_state_id(block_state_id)
            }
            _ => None,
        }
    }

    pub fn set_carried_block(&mut self, carried_block: Option<BlockState>) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::enderman::get_carried_block(),
            MetadataValue::OptionalBlockState(carried_block.map_or(0, BlockState::state_id)),
        );
    }

    pub fn is_screaming(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::enderman::is_screaming())
        {
            MetadataValue::Boolean(is_screaming) => is_screaming,
            _ => false,
        }
    }

    pub fn set_screaming(&mut self, is_screaming: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::enderman::is_screaming(),
            MetadataValue::Boolean(is_screaming),
        );
    }

    pub fn is_staring(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::enderman::is_staring())
        {
            MetadataValue::Boolean(is_staring) => is_staring,
            _ => false,
        }
    }

    pub fn set_staring(&mut self, is_staring: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::enderman::is_staring(),
            MetadataValue::Boolean(is_staring),
        );
    }
}

impl<'entity> Deref for EndermanMeta<'entity> {
    type Target = MonsterMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.monster_meta
    }
}

impl<'entity> DerefMut for EndermanMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster_meta
    }
}
