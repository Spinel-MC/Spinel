use crate::entity::dynamic_variant::UnregisteredEntityVariantError;
use crate::entity::metadata::{AbstractNautilusMeta, EntityMeta};
use spinel_registry::{EntityType, Registries, RegistryKey, zombie_nautilus_variant};
use std::ops::{Deref, DerefMut};

pub struct ZombieNautilusMeta<'entity> {
    abstract_nautilus_meta: AbstractNautilusMeta<'entity>,
}

impl<'entity> ZombieNautilusMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::ZOMBIE_NAUTILUS).then(|| Self {
            abstract_nautilus_meta: AbstractNautilusMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_variant(
        &self,
        registries: &Registries,
    ) -> Option<RegistryKey<zombie_nautilus_variant::ZombieNautilusVariant>> {
        self.get_entity().g(registries)
    }

    pub fn set_variant(
        &mut self,
        registries: &Registries,
        variant: RegistryKey<zombie_nautilus_variant::ZombieNautilusVariant>,
    ) -> Result<(), UnregisteredEntityVariantError> {
        self.get_entity_mut()
            .set_zombie_nautilus_variant_metadata(registries, variant)
    }
}

impl<'entity> Deref for ZombieNautilusMeta<'entity> {
    type Target = AbstractNautilusMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_nautilus_meta
    }
}

impl<'entity> DerefMut for ZombieNautilusMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_nautilus_meta
    }
}
