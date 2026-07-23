use crate::entity::metadata::{LivingEntityMeta, SpellcasterIllagerMeta};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct EvokerMeta<'entity> {
    spellcaster_illager_meta: SpellcasterIllagerMeta<'entity>,
}

impl<'entity> EvokerMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::EVOKER).then(|| Self {
            spellcaster_illager_meta: SpellcasterIllagerMeta::from_living_entity_meta(
                living_entity_meta,
            ),
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
