use crate::entity::metadata::{EntityMeta, RaiderMeta};
use std::ops::{Deref, DerefMut};

pub struct AbstractIllagerMeta<'entity> {
    raider_meta: RaiderMeta<'entity>,
}

impl<'entity> AbstractIllagerMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            raider_meta: RaiderMeta::from_entity_meta(entity_meta),
        }
    }
}

impl<'entity> Deref for AbstractIllagerMeta<'entity> {
    type Target = RaiderMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.raider_meta
    }
}

impl<'entity> DerefMut for AbstractIllagerMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raider_meta
    }
}
