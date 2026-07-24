use crate::entity::metadata::{LivingEntityMeta, MobMeta};
use std::ops::{Deref, DerefMut};

pub struct FlyingMeta<'entity> {
    mob_meta: MobMeta<'entity>,
}

impl<'entity> FlyingMeta<'entity> {
    pub(crate) fn from_living_entity_meta(living_entity_meta: LivingEntityMeta<'entity>) -> Self {
        Self {
            mob_meta: MobMeta::new(living_entity_meta),
        }
    }
}

impl<'entity> Deref for FlyingMeta<'entity> {
    type Target = MobMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.mob_meta
    }
}

impl<'entity> DerefMut for FlyingMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mob_meta
    }
}
