use crate::entity::metadata::{LivingEntityMeta, MonsterMeta};
use std::ops::{Deref, DerefMut};

pub struct AbstractSkeletonMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> AbstractSkeletonMeta<'entity> {
    pub(crate) fn from_living_entity_meta(living_entity_meta: LivingEntityMeta<'entity>) -> Self {
        Self {
            monster_meta: MonsterMeta::from_living_entity_meta(living_entity_meta),
        }
    }
}

impl<'entity> Deref for AbstractSkeletonMeta<'entity> {
    type Target = MonsterMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.monster_meta
    }
}

impl<'entity> DerefMut for AbstractSkeletonMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.monster_meta
    }
}
