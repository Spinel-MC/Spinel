use crate::entity::metadata::{AnimalMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct PolarBearMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> PolarBearMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::POLAR_BEAR).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_standing_up(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::polar_bear::is_standing_up())
        {
            MetadataValue::Boolean(is_standing_up) => is_standing_up,
            _ => false,
        }
    }

    pub fn set_standing_up(&mut self, is_standing_up: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::polar_bear::is_standing_up(),
            MetadataValue::Boolean(is_standing_up),
        );
    }
}

impl<'entity> Deref for PolarBearMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for PolarBearMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
