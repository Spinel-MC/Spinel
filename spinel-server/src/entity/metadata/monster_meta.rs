use crate::entity::metadata::{EntityMeta, LivingEntityMeta, MobMeta, PathfinderMobMeta};
use std::ops::{Deref, DerefMut};

pub struct MonsterMeta<'entity> {
    pathfinder_mob_meta: PathfinderMobMeta<'entity>,
}

impl<'entity> MonsterMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            pathfinder_mob_meta: PathfinderMobMeta::new(MobMeta::new(LivingEntityMeta::new(
                entity_meta,
            ))),
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
