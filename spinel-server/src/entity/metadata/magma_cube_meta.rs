use crate::entity::metadata::{LivingEntityMeta, SlimeMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct MagmaCubeMeta<'entity> {
    slime_meta: SlimeMeta<'entity>,
}

impl<'entity> MagmaCubeMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::MAGMA_CUBE).then(|| Self {
            slime_meta: SlimeMeta::from_living_entity_meta_unchecked(living_entity_meta),
        })
    }
}

impl<'entity> Deref for MagmaCubeMeta<'entity> {
    type Target = SlimeMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.slime_meta
    }
}

impl<'entity> DerefMut for MagmaCubeMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.slime_meta
    }
}
