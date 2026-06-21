use crate::entity::metadata::{AnimalMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct TurtleMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> TurtleMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::TURTLE).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn has_egg(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::turtle::has_egg())
        {
            MetadataValue::Boolean(has_egg) => has_egg,
            _ => false,
        }
    }

    pub fn set_has_egg(&mut self, has_egg: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::turtle::has_egg(),
            MetadataValue::Boolean(has_egg),
        );
    }

    pub fn is_laying_egg(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::turtle::is_laying_egg())
        {
            MetadataValue::Boolean(is_laying_egg) => is_laying_egg,
            _ => false,
        }
    }

    pub fn set_laying_egg(&mut self, is_laying_egg: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::turtle::is_laying_egg(),
            MetadataValue::Boolean(is_laying_egg),
        );
    }
}

impl<'entity> Deref for TurtleMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for TurtleMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
