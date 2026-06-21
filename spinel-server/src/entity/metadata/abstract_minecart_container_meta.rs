use crate::entity::metadata::{AbstractMinecartMeta, EntityMeta};
use std::ops::{Deref, DerefMut};

pub struct AbstractMinecartContainerMeta<'entity> {
    abstract_minecart_meta: AbstractMinecartMeta<'entity>,
}

impl<'entity> AbstractMinecartContainerMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            abstract_minecart_meta: AbstractMinecartMeta::from_entity_meta(entity_meta),
        }
    }
}

impl<'entity> Deref for AbstractMinecartContainerMeta<'entity> {
    type Target = AbstractMinecartMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart_meta
    }
}

impl<'entity> DerefMut for AbstractMinecartContainerMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart_meta
    }
}
