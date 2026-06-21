use crate::entity::metadata::{AbstractIllagerMeta, EntityMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct VindicatorMeta<'entity> {
    abstract_illager_meta: AbstractIllagerMeta<'entity>,
}

impl<'entity> VindicatorMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::VINDICATOR).then(|| Self {
            abstract_illager_meta: AbstractIllagerMeta::from_entity_meta(entity_meta),
        })
    }
}

impl<'entity> Deref for VindicatorMeta<'entity> {
    type Target = AbstractIllagerMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_illager_meta
    }
}

impl<'entity> DerefMut for VindicatorMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_illager_meta
    }
}
