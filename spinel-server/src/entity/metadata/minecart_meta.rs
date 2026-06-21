use crate::entity::metadata::{AbstractMinecartMeta, EntityMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct MinecartMeta<'entity> {
    abstract_minecart_meta: AbstractMinecartMeta<'entity>,
}

impl<'entity> MinecartMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::MINECART).then(|| Self {
            abstract_minecart_meta: AbstractMinecartMeta::from_entity_meta(entity_meta),
        })
    }
}

impl<'entity> Deref for MinecartMeta<'entity> {
    type Target = AbstractMinecartMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart_meta
    }
}

impl<'entity> DerefMut for MinecartMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart_meta
    }
}
