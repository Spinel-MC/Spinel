use crate::entity::metadata::{LivingEntityMeta, RaiderMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct RavagerMeta<'entity> {
    raider_meta: RaiderMeta<'entity>,
}

impl<'entity> RavagerMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::RAVAGER).then(|| Self {
            raider_meta: RaiderMeta::from_living_entity_meta(living_entity_meta),
        })
    }
}

impl<'entity> Deref for RavagerMeta<'entity> {
    type Target = RaiderMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.raider_meta
    }
}

impl<'entity> DerefMut for RavagerMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raider_meta
    }
}
