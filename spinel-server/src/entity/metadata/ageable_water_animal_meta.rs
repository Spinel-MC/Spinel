use crate::entity::metadata::{AgeableMobMeta, LivingEntityMeta};
use std::ops::{Deref, DerefMut};

pub struct AgeableWaterAnimalMeta<'entity> {
    ageable_mob_meta: AgeableMobMeta<'entity>,
}

impl<'entity> AgeableWaterAnimalMeta<'entity> {
    pub(crate) fn new(living_entity_meta: LivingEntityMeta<'entity>) -> Self {
        Self {
            ageable_mob_meta: AgeableMobMeta::new(living_entity_meta),
        }
    }
}

impl<'entity> Deref for AgeableWaterAnimalMeta<'entity> {
    type Target = AgeableMobMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.ageable_mob_meta
    }
}

impl<'entity> DerefMut for AgeableWaterAnimalMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ageable_mob_meta
    }
}
