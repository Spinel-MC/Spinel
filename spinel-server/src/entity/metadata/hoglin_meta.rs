use crate::entity::metadata::{AnimalMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct HoglinMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> HoglinMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::HOGLIN).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_immune_to_zombification(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::hoglin::is_immune_to_zombification())
        {
            MetadataValue::Boolean(is_immune_to_zombification) => is_immune_to_zombification,
            _ => false,
        }
    }

    pub fn set_immune_to_zombification(&mut self, is_immune_to_zombification: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::hoglin::is_immune_to_zombification(),
            MetadataValue::Boolean(is_immune_to_zombification),
        );
    }
}

impl<'entity> Deref for HoglinMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for HoglinMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
