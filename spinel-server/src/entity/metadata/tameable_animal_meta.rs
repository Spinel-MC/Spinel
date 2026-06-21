use crate::entity::metadata::{AnimalMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

pub struct TameableAnimalMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> TameableAnimalMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        }
    }

    pub fn is_sitting(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .flag(&definitions::tameable_animal::is_sitting())
    }

    pub fn set_sitting(&mut self, is_sitting: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::tameable_animal::is_sitting(), is_sitting);
    }

    pub fn is_tamed(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .flag(&definitions::tameable_animal::is_tamed())
    }

    pub fn set_tamed(&mut self, is_tamed: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::tameable_animal::is_tamed(), is_tamed);
    }

    pub fn get_owner(&self) -> Option<Uuid> {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::tameable_animal::get_owner())
        {
            MetadataValue::OptionalLivingEntityReference(owner) => owner,
            _ => None,
        }
    }

    pub fn set_owner(&mut self, owner: Option<Uuid>) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::tameable_animal::get_owner(),
            MetadataValue::OptionalLivingEntityReference(owner),
        );
    }
}

impl<'entity> Deref for TameableAnimalMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for TameableAnimalMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
