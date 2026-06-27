use crate::entity::dynamic_variant::UnregisteredEntityVariantError;
use crate::entity::metadata::{AnimalMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::{EntityType, Registries, RegistryKey, pig_variant};
use std::ops::{Deref, DerefMut};

pub struct PigMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> PigMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::PIG).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_time_to_boost(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::pig::boost_time())
        {
            MetadataValue::VarInt(time_to_boost) => time_to_boost,
            _ => 0,
        }
    }

    pub fn set_time_to_boost(&mut self, time_to_boost: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::pig::boost_time(),
            MetadataValue::VarInt(time_to_boost),
        );
    }

    pub fn get_variant(
        &self,
        registries: &Registries,
    ) -> Option<RegistryKey<pig_variant::PigVariant>> {
        self.get_entity().get_pig_variant_metadata(registries)
    }

    pub fn set_variant(
        &mut self,
        registries: &Registries,
        variant: RegistryKey<pig_variant::PigVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        self.get_entity_mut()
            .set_pig_variant_metadata(registries, variant)
    }
}

impl<'entity> Deref for PigMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for PigMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
