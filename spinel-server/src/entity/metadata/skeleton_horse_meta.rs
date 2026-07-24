use crate::entity::metadata::{AbstractHorseMeta, LivingEntityMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct SkeletonHorseMeta<'entity> {
    abstract_horse_meta: AbstractHorseMeta<'entity>,
}

impl<'entity> SkeletonHorseMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::SKELETON_HORSE).then(|| Self {
            abstract_horse_meta: AbstractHorseMeta::from_living_entity_meta(living_entity_meta),
        })
    }
}

impl<'entity> Deref for SkeletonHorseMeta<'entity> {
    type Target = AbstractHorseMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_horse_meta
    }
}

impl<'entity> DerefMut for SkeletonHorseMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_horse_meta
    }
}
