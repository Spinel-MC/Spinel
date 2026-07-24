use crate::entity::metadata::{ChestedHorseMeta, LivingEntityMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct MuleMeta<'entity> {
    chested_horse_meta: ChestedHorseMeta<'entity>,
}

impl<'entity> MuleMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::MULE).then(|| Self {
            chested_horse_meta: ChestedHorseMeta::from_living_entity_meta(living_entity_meta),
        })
    }
}

impl<'entity> Deref for MuleMeta<'entity> {
    type Target = ChestedHorseMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.chested_horse_meta
    }
}

impl<'entity> DerefMut for MuleMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chested_horse_meta
    }
}
