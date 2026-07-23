use crate::entity::metadata::{LivingEntityMeta, MobMeta, PathfinderMobMeta};
use std::ops::{Deref, DerefMut};

pub struct MonsterMeta<'entity> {
    pathfinder_mob_meta: PathfinderMobMeta<'entity>,
}

impl<'entity> MonsterMeta<'entity> {
    pub(crate) fn from_living_entity_meta(living_entity_meta: LivingEntityMeta<'entity>) -> Self {
        Self {
            pathfinder_mob_meta: PathfinderMobMeta::new(MobMeta::new(living_entity_meta)),
        }
    }
}

impl<'entity> Deref for MonsterMeta<'entity> {
    type Target = PathfinderMobMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.pathfinder_mob_meta
    }
}

impl<'entity> DerefMut for MonsterMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pathfinder_mob_meta
    }
}
