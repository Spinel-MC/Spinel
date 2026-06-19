use crate::entity::metadata::MobMeta;
use std::ops::{Deref, DerefMut};

pub struct PathfinderMobMeta<'entity> {
    mob_meta: MobMeta<'entity>,
}

impl<'entity> PathfinderMobMeta<'entity> {
    pub(crate) fn new(mob_meta: MobMeta<'entity>) -> Self {
        Self { mob_meta }
    }
}

impl<'entity> Deref for PathfinderMobMeta<'entity> {
    type Target = MobMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.mob_meta
    }
}

impl<'entity> DerefMut for PathfinderMobMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mob_meta
    }
}
