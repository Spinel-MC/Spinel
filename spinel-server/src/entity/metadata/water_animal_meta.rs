use crate::entity::metadata::{LivingEntityMeta, MobMeta, PathfinderMobMeta};
use std::ops::{Deref, DerefMut};

pub struct WaterAnimalMeta<'entity> {
    pathfinder_mob_meta: PathfinderMobMeta<'entity>,
}

impl<'entity> WaterAnimalMeta<'entity> {
    pub(crate) fn new(living_entity_meta: LivingEntityMeta<'entity>) -> Self {
        Self {
            pathfinder_mob_meta: PathfinderMobMeta::new(MobMeta::new(living_entity_meta)),
        }
    }
}

impl<'entity> Deref for WaterAnimalMeta<'entity> {
    type Target = PathfinderMobMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.pathfinder_mob_meta
    }
}

impl<'entity> DerefMut for WaterAnimalMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pathfinder_mob_meta
    }
}
