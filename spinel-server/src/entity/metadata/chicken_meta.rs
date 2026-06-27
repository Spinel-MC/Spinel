use crate::entity::dynamic_variant::UnregisteredEntityVariantError;
use crate::entity::metadata::{AnimalMeta, EntityMeta};
use spinel_registry::{EntityType, Registries, RegistryKey, chicken_variant};
use std::ops::{Deref, DerefMut};

pub struct ChickenMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> ChickenMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::CHICKEN).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_variant(
        &self,
        registries: &Registries,
    ) -> Option<RegistryKey<chicken_variant::ChickenVariant>> {
        self.get_entity().get_chicken_variant_metadata(registries)
    }

    pub fn set_variant(
        &mut self,
        registries: &Registries,
        variant: RegistryKey<chicken_variant::ChickenVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        self.get_entity_mut()
            .set_chicken_variant_metadata(registries, variant)
    }
}

impl<'entity> Deref for ChickenMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for ChickenMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
