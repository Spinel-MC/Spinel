use crate::entity::metadata::{LivingEntityMeta, MobMeta, PathfinderMobMeta};
use std::ops::{Deref, DerefMut};

pub struct AbstractGolemMeta<'entity> {
    pathfinder_mob_meta: PathfinderMobMeta<'entity>,
}

impl<'entity> AbstractGolemMeta<'entity> {
    pub(crate) fn from_living_entity_meta(living_entity_meta: LivingEntityMeta<'entity>) -> Self {
        Self {
            pathfinder_mob_meta: PathfinderMobMeta::new(MobMeta::new(living_entity_meta)),
        }
    }
}

impl<'entity> Deref for AbstractGolemMeta<'entity> {
    type Target = PathfinderMobMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.pathfinder_mob_meta
    }
}

impl<'entity> DerefMut for AbstractGolemMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pathfinder_mob_meta
    }
}
