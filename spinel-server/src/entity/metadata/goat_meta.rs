use crate::entity::metadata::{AnimalMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct GoatMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> GoatMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::GOAT).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_screaming(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::goat::is_screaming())
        {
            MetadataValue::Boolean(is_screaming) => is_screaming,
            _ => false,
        }
    }

    pub fn set_screaming(&mut self, is_screaming: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::goat::is_screaming(),
            MetadataValue::Boolean(is_screaming),
        );
    }

    pub fn has_left_horn(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::goat::has_left_horn())
        {
            MetadataValue::Boolean(has_left_horn) => has_left_horn,
            _ => true,
        }
    }

    pub fn set_has_left_horn(&mut self, has_left_horn: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::goat::has_left_horn(),
            MetadataValue::Boolean(has_left_horn),
        );
    }

    pub fn has_right_horn(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::goat::has_right_horn())
        {
            MetadataValue::Boolean(has_right_horn) => has_right_horn,
            _ => true,
        }
    }

    pub fn set_has_right_horn(&mut self, has_right_horn: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::goat::has_right_horn(),
            MetadataValue::Boolean(has_right_horn),
        );
    }
}

impl<'entity> Deref for GoatMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for GoatMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
