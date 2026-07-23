use crate::entity::metadata::{AbstractHorseMeta, LivingEntityMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct ZombieHorseMeta<'entity> {
    abstract_horse_meta: AbstractHorseMeta<'entity>,
}

impl<'entity> ZombieHorseMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::ZOMBIE_HORSE).then(|| Self {
            abstract_horse_meta: AbstractHorseMeta::from_living_entity_meta(living_entity_meta),
        })
    }
}

impl<'entity> Deref for ZombieHorseMeta<'entity> {
    type Target = AbstractHorseMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_horse_meta
    }
}

impl<'entity> DerefMut for ZombieHorseMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_horse_meta
    }
}
