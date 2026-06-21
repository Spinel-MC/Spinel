use crate::entity::metadata::{CreeperState, EntityMeta, MonsterMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct CreeperMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> CreeperMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::CREEPER).then(|| Self {
            monster_meta: MonsterMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_state(&self) -> CreeperState {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::creeper::get_state())
        {
            MetadataValue::VarInt(state) => CreeperState::from_protocol_id(state),
            _ => CreeperState::default(),
        }
    }

    pub fn set_state(&mut self, state: CreeperState) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::creeper::get_state(),
            MetadataValue::VarInt(state.get_protocol_id()),
        );
    }

    pub fn is_charged(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::creeper::is_charged())
        {
            MetadataValue::Boolean(is_charged) => is_charged,
            _ => false,
        }
    }

    pub fn set_charged(&mut self, is_charged: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::creeper::is_charged(),
            MetadataValue::Boolean(is_charged),
        );
    }

    pub fn is_ignited(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::creeper::is_ignited())
        {
            MetadataValue::Boolean(is_ignited) => is_ignited,
            _ => false,
        }
    }

    pub fn set_ignited(&mut self, is_ignited: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::creeper::is_ignited(),
            MetadataValue::Boolean(is_ignited),
        );
    }
}

impl<'entity> Deref for CreeperMeta<'entity> {
    type Target = MonsterMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.monster_meta
    }
}

impl<'entity> DerefMut for CreeperMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster_meta
    }
}
