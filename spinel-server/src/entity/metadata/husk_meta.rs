use crate::entity::metadata::{EntityMeta, ZombieMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct HuskMeta<'entity> {
    zombie_meta: ZombieMeta<'entity>,
}

impl<'entity> HuskMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::HUSK).then(|| Self {
            zombie_meta: ZombieMeta::from_subtype_entity_meta(entity_meta),
        })
    }
}

impl<'entity> Deref for HuskMeta<'entity> {
    type Target = ZombieMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.zombie_meta
    }
}

impl<'entity> DerefMut for HuskMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.zombie_meta
    }
}
