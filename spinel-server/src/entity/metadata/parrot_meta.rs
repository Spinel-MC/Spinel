use crate::entity::metadata::{EntityMeta, ParrotColor, TameableAnimalMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct ParrotMeta<'entity> {
    tameable_animal_meta: TameableAnimalMeta<'entity>,
}

impl<'entity> ParrotMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::PARROT).then(|| Self {
            tameable_animal_meta: TameableAnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_color(&self) -> ParrotColor {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::parrot::get_variant())
        {
            MetadataValue::VarInt(color_id) => {
                ParrotColor::from_protocol_id(color_id).unwrap_or_default()
            }
            _ => ParrotColor::default(),
        }
    }

    pub fn set_color(&mut self, color: ParrotColor) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::parrot::get_variant(),
            MetadataValue::VarInt(color.protocol_id()),
        );
    }
}

impl<'entity> Deref for ParrotMeta<'entity> {
    type Target = TameableAnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.tameable_animal_meta
    }
}

impl<'entity> DerefMut for ParrotMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tameable_animal_meta
    }
}
