use crate::entity::metadata::{AnimalMeta, EntityMeta, MooshroomVariant, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct MooshroomMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> MooshroomMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::MOOSHROOM).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_variant(&self) -> MooshroomVariant {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::mooshroom::get_variant())
        {
            MetadataValue::VarInt(variant_id) => {
                MooshroomVariant::from_protocol_id(variant_id).unwrap_or_default()
            }
            _ => MooshroomVariant::default(),
        }
    }

    pub fn set_variant(&mut self, variant: MooshroomVariant) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::mooshroom::get_variant(),
            MetadataValue::VarInt(variant.protocol_id()),
        );
    }
}

impl<'entity> Deref for MooshroomMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for MooshroomMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
