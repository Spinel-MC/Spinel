use crate::entity::metadata::{
    AbstractIllagerMeta, EntityMeta, SpellcasterIllagerSpell, definitions,
};
use spinel_network::types::entity_metadata::MetadataValue;
use std::ops::{Deref, DerefMut};

pub struct SpellcasterIllagerMeta<'entity> {
    abstract_illager_meta: AbstractIllagerMeta<'entity>,
}

impl<'entity> SpellcasterIllagerMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            abstract_illager_meta: AbstractIllagerMeta::from_entity_meta(entity_meta),
        }
    }

    pub fn get_spell(&self) -> SpellcasterIllagerSpell {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::spellcaster_illager::get_spell())
        {
            MetadataValue::Byte(spell_id) => {
                SpellcasterIllagerSpell::from_protocol_id(spell_id).unwrap_or_default()
            }
            _ => SpellcasterIllagerSpell::default(),
        }
    }

    pub fn set_spell(&mut self, spell: SpellcasterIllagerSpell) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::spellcaster_illager::get_spell(),
            MetadataValue::Byte(spell.get_protocol_id()),
        );
    }
}

impl<'entity> Deref for SpellcasterIllagerMeta<'entity> {
    type Target = AbstractIllagerMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_illager_meta
    }
}

impl<'entity> DerefMut for SpellcasterIllagerMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_illager_meta
    }
}
