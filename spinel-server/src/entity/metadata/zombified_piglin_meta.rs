use crate::entity::metadata::{EntityMeta, ZombieMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct ZombifiedPiglinMeta<'entity> {
    zombie_meta: ZombieMeta<'entity>,
}

impl<'entity> ZombifiedPiglinMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::ZOMBIFIED_PIGLIN).then(|| Self {
            zombie_meta: ZombieMeta::from_subtype_entity_meta(entity_meta),
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
