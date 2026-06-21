use crate::entity::metadata::{AnimalMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct StriderMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> StriderMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::STRIDER).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_time_to_boost(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::strider::fungus_boost())
        {
            MetadataValue::VarInt(time_to_boost) => time_to_boost,
            _ => 0,
        }
    }

    pub fn set_time_to_boost(&mut self, time_to_boost: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::strider::fungus_boost(),
            MetadataValue::VarInt(time_to_boost),
        );
    }

    pub fn is_shaking(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::strider::is_shaking())
        {
            MetadataValue::Boolean(is_shaking) => is_shaking,
            _ => false,
        }
    }

    pub fn set_shaking(&mut self, is_shaking: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::strider::is_shaking(),
            MetadataValue::Boolean(is_shaking),
        );
    }
}

impl<'entity> Deref for StriderMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for StriderMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
