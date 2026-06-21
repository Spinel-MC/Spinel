use crate::entity::metadata::{AbstractIllagerMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct PillagerMeta<'entity> {
    abstract_illager_meta: AbstractIllagerMeta<'entity>,
}

impl<'entity> PillagerMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::PILLAGER).then(|| Self {
            abstract_illager_meta: AbstractIllagerMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_charging_crossbow(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::pillager::is_charging())
        {
            MetadataValue::Boolean(is_charging_crossbow) => is_charging_crossbow,
            _ => false,
        }
    }

    pub fn set_charging_crossbow(&mut self, is_charging_crossbow: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::pillager::is_charging(),
            MetadataValue::Boolean(is_charging_crossbow),
        );
    }
}

impl<'entity> Deref for PillagerMeta<'entity> {
    type Target = AbstractIllagerMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_illager_meta
    }
}

impl<'entity> DerefMut for PillagerMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_illager_meta
    }
}
