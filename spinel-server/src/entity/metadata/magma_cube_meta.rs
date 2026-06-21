use crate::entity::metadata::{EntityMeta, SlimeMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct MagmaCubeMeta<'entity> {
    slime_meta: SlimeMeta<'entity>,
}

impl<'entity> MagmaCubeMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::MAGMA_CUBE).then(|| Self {
            slime_meta: SlimeMeta::from_entity_meta_unchecked(entity_meta),
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
