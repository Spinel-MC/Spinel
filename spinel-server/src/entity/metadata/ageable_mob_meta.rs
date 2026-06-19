use crate::entity::metadata::{EntityMeta, LivingEntityMeta, MobMeta, PathfinderMobMeta};
use std::ops::{Deref, DerefMut};

pub struct AgeableMobMeta<'entity> {
    pathfinder_mob_meta: PathfinderMobMeta<'entity>,
}

impl<'entity> AgeableMobMeta<'entity> {
    pub(crate) fn new(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            pathfinder_mob_meta: PathfinderMobMeta::new(MobMeta::new(LivingEntityMeta::new(
                entity_meta,
            ))),
        }
    }

    pub fn is_baby(&self) -> bool {
        self.entity().is_baby()
    }

    pub fn set_baby(&mut self, is_baby: bool) {
        self.entity_mut().set_baby(is_baby);
    }
}

impl<'entity> Deref for AgeableMobMeta<'entity> {
    type Target = PathfinderMobMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.pathfinder_mob_meta
    }
}

impl<'entity> DerefMut for AgeableMobMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pathfinder_mob_meta
    }
}
