use crate::entity::metadata::{AbstractMinecartContainerMeta, EntityMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct HopperMinecartMeta<'entity> {
    abstract_minecart_container_meta: AbstractMinecartContainerMeta<'entity>,
}

impl<'entity> HopperMinecartMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::HOPPER_MINECART).then(|| Self {
            abstract_minecart_container_meta: AbstractMinecartContainerMeta::from_entity_meta(
                entity_meta,
            ),
        })
    }
}

impl<'entity> Deref for HopperMinecartMeta<'entity> {
    type Target = AbstractMinecartContainerMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart_container_meta
    }
}

impl<'entity> DerefMut for HopperMinecartMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart_container_meta
    }
}
