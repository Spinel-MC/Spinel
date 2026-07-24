use crate::entity::metadata::{AbstractIllagerMeta, LivingEntityMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct VindicatorMeta<'entity> {
    abstract_illager_meta: AbstractIllagerMeta<'entity>,
}

impl<'entity> VindicatorMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::VINDICATOR).then(|| Self {
            abstract_illager_meta: AbstractIllagerMeta::from_living_entity_meta(living_entity_meta),
        })
    }
}

impl<'entity> Deref for VindicatorMeta<'entity> {
    type Target = AbstractIllagerMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_illager_meta
    }
}

impl<'entity> DerefMut for VindicatorMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_illager_meta
    }
}
