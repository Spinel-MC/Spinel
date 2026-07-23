use crate::entity::metadata::{BasePiglinMeta, LivingEntityMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct PiglinBruteMeta<'entity> {
    base_piglin_meta: BasePiglinMeta<'entity>,
}

impl<'entity> PiglinBruteMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::PIGLIN_BRUTE).then(|| Self {
            base_piglin_meta: BasePiglinMeta::from_living_entity_meta(living_entity_meta),
        })
    }
}

impl<'entity> Deref for PiglinBruteMeta<'entity> {
    type Target = BasePiglinMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.base_piglin_meta
    }
}

impl<'entity> DerefMut for PiglinBruteMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base_piglin_meta
    }
}
