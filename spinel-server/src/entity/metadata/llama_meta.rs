use crate::entity::metadata::{ChestedHorseMeta, EntityMeta, LlamaVariant, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct LlamaMeta<'entity> {
    chested_horse_meta: ChestedHorseMeta<'entity>,
}

impl<'entity> LlamaMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        matches!(
            entity_meta.get_entity().get_entity_type(),
            EntityType::LLAMA | EntityType::TRADER_LLAMA
        )
        .then(|| Self {
            chested_horse_meta: ChestedHorseMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_strength(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::llama::get_strength())
        {
            MetadataValue::VarInt(strength) => strength,
            _ => 0,
        }
    }

    pub fn set_strength(&mut self, strength: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::llama::get_strength(),
            MetadataValue::VarInt(strength),
        );
    }

    pub fn get_carpet_color(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::llama::get_carpet_color())
        {
            MetadataValue::VarInt(carpet_color) => carpet_color,
            _ => -1,
        }
    }

    pub fn set_carpet_color(&mut self, carpet_color: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::llama::get_carpet_color(),
            MetadataValue::VarInt(carpet_color),
        );
    }

    pub fn get_variant(&self) -> LlamaVariant {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::llama::get_variant())
        {
            MetadataValue::VarInt(variant_id) => {
                LlamaVariant::from_protocol_id(variant_id).unwrap_or_default()
            }
            _ => LlamaVariant::default(),
        }
    }

    pub fn set_variant(&mut self, variant: LlamaVariant) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::llama::get_variant(),
            MetadataValue::VarInt(variant.protocol_id()),
        );
    }
}

impl<'entity> Deref for LlamaMeta<'entity> {
    type Target = ChestedHorseMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.chested_horse_meta
    }
}

impl<'entity> DerefMut for LlamaMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chested_horse_meta
    }
}
