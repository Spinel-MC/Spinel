use crate::entity::metadata::{AbstractFishMeta, EntityMeta, TropicalFishVariant, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct TropicalFishMeta<'entity> {
    abstract_fish_meta: AbstractFishMeta<'entity>,
}

impl<'entity> TropicalFishMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::TROPICAL_FISH).then(|| Self {
            abstract_fish_meta: AbstractFishMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_variant(&self) -> TropicalFishVariant {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::tropical_fish::get_variant())
        {
            MetadataValue::VarInt(variant) => {
                TropicalFishVariant::from_packed_id(variant).unwrap_or_default()
            }
            _ => TropicalFishVariant::default(),
        }
    }

    pub fn set_variant(&mut self, variant: TropicalFishVariant) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::tropical_fish::get_variant(),
            MetadataValue::VarInt(variant.get_packed_id()),
        );
    }
}

impl<'entity> Deref for TropicalFishMeta<'entity> {
    type Target = AbstractFishMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_fish_meta
    }
}

impl<'entity> DerefMut for TropicalFishMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_fish_meta
    }
}
