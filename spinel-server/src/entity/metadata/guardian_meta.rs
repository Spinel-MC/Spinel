use crate::entity::Entity;
use crate::entity::metadata::{EntityMeta, MonsterMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct GuardianMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> GuardianMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        matches!(
            entity_meta.entity().entity_type(),
            EntityType::GUARDIAN | EntityType::ELDER_GUARDIAN
        )
        .then(|| Self {
            monster_meta: MonsterMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_retracting_spikes(&self) -> bool {
        match self
            .entity()
            .metadata()
            .value(&definitions::guardian::is_retracting_spikes())
        {
            MetadataValue::Boolean(is_retracting_spikes) => is_retracting_spikes,
            _ => false,
        }
    }

    pub fn set_retracting_spikes(&mut self, is_retracting_spikes: bool) {
        self.entity_mut().metadata_mut().set(
            &definitions::guardian::is_retracting_spikes(),
            MetadataValue::Boolean(is_retracting_spikes),
        );
    }

    pub fn target_entity_id(&self) -> i32 {
        match self
            .entity()
            .metadata()
            .value(&definitions::guardian::target_entity_id())
        {
            MetadataValue::VarInt(target_entity_id) => target_entity_id,
            _ => 0,
        }
    }

    pub fn set_target_entity_id(&mut self, target_entity_id: i32) {
        self.entity_mut().metadata_mut().set(
            &definitions::guardian::target_entity_id(),
            MetadataValue::VarInt(target_entity_id),
        );
    }

    pub fn set_target(&mut self, target: Option<&Entity>) {
        self.set_target_entity_id(target.map_or(0, |target| target.entity_id().value()));
    }
}

impl<'entity> Deref for GuardianMeta<'entity> {
    type Target = MonsterMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.monster_meta
    }
}

impl<'entity> DerefMut for GuardianMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster_meta
    }
}
