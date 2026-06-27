use crate::entity::dynamic_variant::UnregisteredEntityVariantError;
use crate::entity::metadata::{AnimalMeta, EntityMeta};
use spinel_registry::{EntityType, Registries, RegistryKey, cow_variant};
use std::ops::{Deref, DerefMut};

pub struct CowMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> CowMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::COW).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_variant(
        &self,
        registries: &Registries,
    ) -> Option<RegistryKey<cow_variant::CowVariant>> {
        self.get_entity().get_cow_variant_metadata(registries)
    }

    pub fn set_variant(
        &mut self,
        registries: &Registries,
        variant: RegistryKey<cow_variant::CowVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        self.get_entity_mut()
            .set_cow_variant_metadata(registries, variant)
    }
}

impl<'entity> Deref for CowMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for CowMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
