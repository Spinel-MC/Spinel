use crate::entity::metadata::{AbstractHorseMeta, HorseVariant, LivingEntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct HorseMeta<'entity> {
    abstract_horse_meta: AbstractHorseMeta<'entity>,
}

impl<'entity> HorseMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::HORSE).then(|| Self {
            abstract_horse_meta: AbstractHorseMeta::from_living_entity_meta(living_entity_meta),
        })
    }

    pub fn get_variant(&self) -> HorseVariant {
        match self
            .get_state()
            .get_metadata()
            .get_value(&definitions::horse::get_variant())
        {
            MetadataValue::VarInt(variant_id) => {
                HorseVariant::from_protocol_id(variant_id).unwrap_or_default()
            }
            _ => HorseVariant::default(),
        }
    }

    pub fn set_variant(&mut self, variant: HorseVariant) {
        self.get_entity_state_mut().get_metadata_mut().set(
            &definitions::horse::get_variant(),
            MetadataValue::VarInt(variant.get_protocol_id()),
        );
    }
}

impl<'entity> Deref for HorseMeta<'entity> {
    type Target = AbstractHorseMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_horse_meta
    }
}

impl<'entity> DerefMut for HorseMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_horse_meta
    }
}
