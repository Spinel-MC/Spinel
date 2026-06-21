use crate::entity::metadata::{EntityMeta, SpellcasterIllagerMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct EvokerMeta<'entity> {
    spellcaster_illager_meta: SpellcasterIllagerMeta<'entity>,
}

impl<'entity> EvokerMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::EVOKER).then(|| Self {
            spellcaster_illager_meta: SpellcasterIllagerMeta::from_entity_meta(entity_meta),
        })
    }
}

impl<'entity> Deref for EvokerMeta<'entity> {
    type Target = SpellcasterIllagerMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.spellcaster_illager_meta
    }
}

impl<'entity> DerefMut for EvokerMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.spellcaster_illager_meta
    }
}
