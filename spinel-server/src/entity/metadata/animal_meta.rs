use crate::entity::metadata::{AgeableMobMeta, EntityMeta};
use std::ops::{Deref, DerefMut};

pub struct AnimalMeta<'entity> {
    ageable_mob_meta: AgeableMobMeta<'entity>,
}

impl<'entity> AnimalMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            ageable_mob_meta: AgeableMobMeta::new(entity_meta),
        }
    }
}

impl<'entity> Deref for AnimalMeta<'entity> {
    type Target = AgeableMobMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.ageable_mob_meta
    }
}

impl<'entity> DerefMut for AnimalMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ageable_mob_meta
    }
}
