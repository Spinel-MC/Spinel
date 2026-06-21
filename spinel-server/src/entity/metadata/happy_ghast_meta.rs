use crate::entity::metadata::{AnimalMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct HappyGhastMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> HappyGhastMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::HAPPY_GHAST).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_leash_holder(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::happy_ghast::is_leash_holder())
        {
            MetadataValue::Boolean(is_leash_holder) => is_leash_holder,
            _ => false,
        }
    }

    pub fn set_leash_holder(&mut self, is_leash_holder: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::happy_ghast::is_leash_holder(),
            MetadataValue::Boolean(is_leash_holder),
        );
    }

    pub fn should_stay_still(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::happy_ghast::stays_still())
        {
            MetadataValue::Boolean(stays_still) => stays_still,
            _ => false,
        }
    }

    pub fn set_stays_still(&mut self, stays_still: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::happy_ghast::stays_still(),
            MetadataValue::Boolean(stays_still),
        );
    }
}

impl<'entity> Deref for HappyGhastMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for HappyGhastMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
