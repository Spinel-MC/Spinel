use crate::entity::metadata::{EntityMeta, LivingEntityMeta, MobMeta};
use std::ops::{Deref, DerefMut};

pub struct AmbientCreatureMeta<'entity> {
    mob_meta: MobMeta<'entity>,
}

impl<'entity> AmbientCreatureMeta<'entity> {
    pub(crate) fn new(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            mob_meta: MobMeta::new(LivingEntityMeta::new(entity_meta)),
        }
    }
}

impl<'entity> Deref for AmbientCreatureMeta<'entity> {
    type Target = MobMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.mob_meta
    }
}

impl<'entity> DerefMut for AmbientCreatureMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mob_meta
    }
}
