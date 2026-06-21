use crate::entity::metadata::{AnimalMeta, AxolotlVariant, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct AxolotlMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> AxolotlMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::AXOLOTL).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_variant(&self) -> AxolotlVariant {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::axolotl::get_variant())
        {
            MetadataValue::VarInt(variant_id) => {
                AxolotlVariant::from_protocol_id(variant_id).unwrap_or_default()
            }
            _ => AxolotlVariant::default(),
        }
    }

    pub fn set_variant(&mut self, variant: AxolotlVariant) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::axolotl::get_variant(),
            MetadataValue::VarInt(variant.protocol_id()),
        );
    }

    pub fn is_playing_dead(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::axolotl::is_playing_dead())
        {
            MetadataValue::Boolean(is_playing_dead) => is_playing_dead,
            _ => false,
        }
    }

    pub fn set_playing_dead(&mut self, is_playing_dead: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::axolotl::is_playing_dead(),
            MetadataValue::Boolean(is_playing_dead),
        );
    }

    pub fn is_from_bucket(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::axolotl::is_from_bucket())
        {
            MetadataValue::Boolean(is_from_bucket) => is_from_bucket,
            _ => false,
        }
    }

    pub fn set_from_bucket(&mut self, is_from_bucket: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::axolotl::is_from_bucket(),
            MetadataValue::Boolean(is_from_bucket),
        );
    }
}

impl<'entity> Deref for AxolotlMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for AxolotlMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
