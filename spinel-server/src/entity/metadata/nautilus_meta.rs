use crate::entity::metadata::{AbstractNautilusMeta, EntityMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct NautilusMeta<'entity> {
    abstract_nautilus_meta: AbstractNautilusMeta<'entity>,
}

impl<'entity> NautilusMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::NAUTILUS).then(|| Self {
            abstract_nautilus_meta: AbstractNautilusMeta::from_entity_meta(entity_meta),
        })
    }
}

impl<'entity> Deref for NautilusMeta<'entity> {
    type Target = AbstractNautilusMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_nautilus_meta
    }
}

impl<'entity> DerefMut for NautilusMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_nautilus_meta
    }
}
