use crate::entity::dynamic_variant::UnregisteredEntityVariantError;
use crate::entity::metadata::{AnimalMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::{EntityType, Registries, RegistryKey, frog_variant};
use std::ops::{Deref, DerefMut};

pub struct FrogMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> FrogMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::FROG).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_variant(
        &self,
        registries: &Registries,
    ) -> Option<RegistryKey<frog_variant::FrogVariant>> {
        self.get_entity().g(registries)
    }

    pub fn set_variant(
        &mut self,
        registries: &Registries,
        variant: RegistryKey<frog_variant::FrogVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        self.get_entity_mut()
            .set_frog_variant_metadata(registries, variant)
    }

    pub fn get_tongue_target(&self) -> Option<i32> {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::frog::get_tongue_target())
        {
            MetadataValue::OptionalVarInt(tongue_target) => tongue_target,
            _ => Some(0),
        }
    }

    pub fn set_tongue_target(&mut self, tongue_target: Option<i32>) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::frog::get_tongue_target(),
            MetadataValue::OptionalVarInt(tongue_target),
        );
    }
}

impl<'entity> Deref for FrogMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for FrogMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
