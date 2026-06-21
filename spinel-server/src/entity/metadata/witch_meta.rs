use crate::entity::metadata::{EntityMeta, RaiderMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct WitchMeta<'entity> {
    raider_meta: RaiderMeta<'entity>,
}

impl<'entity> WitchMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::WITCH).then(|| Self {
            raider_meta: RaiderMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_drinking_potion(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::witch::is_drinking_potion())
        {
            MetadataValue::Boolean(is_drinking_potion) => is_drinking_potion,
            _ => false,
        }
    }

    pub fn set_drinking_potion(&mut self, is_drinking_potion: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::witch::is_drinking_potion(),
            MetadataValue::Boolean(is_drinking_potion),
        );
    }
}

impl<'entity> Deref for WitchMeta<'entity> {
    type Target = RaiderMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.raider_meta
    }
}

impl<'entity> DerefMut for WitchMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raider_meta
    }
}
