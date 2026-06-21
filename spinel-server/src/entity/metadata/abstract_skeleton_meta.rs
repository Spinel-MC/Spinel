use crate::entity::metadata::{EntityMeta, MonsterMeta};
use std::ops::{Deref, DerefMut};

pub struct AbstractSkeletonMeta<'entity> {
    monster_meta: MonsterMeta<'entity>,
}

impl<'entity> AbstractSkeletonMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            monster_meta: MonsterMeta::from_entity_meta(entity_meta),
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
