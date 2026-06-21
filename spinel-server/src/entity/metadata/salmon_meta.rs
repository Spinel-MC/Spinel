use crate::entity::metadata::{AbstractFishMeta, EntityMeta, SalmonSize, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct SalmonMeta<'entity> {
    abstract_fish_meta: AbstractFishMeta<'entity>,
}

impl<'entity> SalmonMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::SALMON).then(|| Self {
            abstract_fish_meta: AbstractFishMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_size(&self) -> SalmonSize {
        match self.get_entity().get_metadata().get_value(&definitions::salmon::get_size()) {
            MetadataValue::VarInt(size) => SalmonSize::from_protocol_id(size).unwrap_or_default(),
            _ => SalmonSize::default(),
        }
    }

    pub fn set_size(&mut self, size: SalmonSize) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::salmon::get_size(),
            MetadataValue::VarInt(size.protocol_id()),
        );
    }
}

impl<'entity> Deref for SalmonMeta<'entity> {
    type Target = AbstractFishMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_fish_meta
    }
}

impl<'entity> DerefMut for SalmonMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_fish_meta
    }
}
