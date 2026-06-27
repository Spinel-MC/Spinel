use crate::entity::metadata::{AnimalMeta, EntityMeta, RabbitVariant, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct RabbitMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> RabbitMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::RABBIT).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_variant(&self) -> RabbitVariant {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::rabbit::kind())
        {
            MetadataValue::VarInt(variant_id) => {
                RabbitVariant::from_protocol_id(variant_id).unwrap_or_default()
            }
            _ => RabbitVariant::default(),
        }
    }

    pub fn set_variant(&mut self, variant: RabbitVariant) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::rabbit::kind(),
            MetadataValue::VarInt(variant.protocol_id()),
        );
    }
}

impl<'entity> Deref for RabbitMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for RabbitMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
