use crate::entity::metadata::{LivingEntityMeta, ZombieMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct ZombifiedPiglinMeta<'entity> {
    zombie_meta: ZombieMeta<'entity>,
}

impl<'entity> ZombifiedPiglinMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::ZOMBIFIED_PIGLIN).then(|| Self {
            zombie_meta: ZombieMeta::from_subtype_living_entity_meta(living_entity_meta),
        })
    }
}

impl<'entity> Deref for ZombifiedPiglinMeta<'entity> {
    type Target = ZombieMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.zombie_meta
    }
}

impl<'entity> DerefMut for ZombifiedPiglinMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.zombie_meta
    }
}
