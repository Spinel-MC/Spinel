use crate::entity::metadata::{EntityMeta, definitions};
use spinel_network::types::Position;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct EndCrystalMeta<'entity> {
    entity_meta: EntityMeta<'entity>,
}

impl<'entity> EndCrystalMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::END_CRYSTAL)
            .then_some(Self { entity_meta })
    }

    pub fn get_beam_target(&self) -> Option<Position> {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::end_crystal::get_beam_target())
        {
            MetadataValue::OptionalPosition(beam_target) => beam_target,
            _ => None,
        }
    }

    pub fn set_beam_target(&mut self, beam_target: Option<Position>) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::end_crystal::get_beam_target(),
            MetadataValue::OptionalPosition(beam_target),
        );
    }

    pub fn is_showing_bottom(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::end_crystal::show_bottom())
        {
            MetadataValue::Boolean(is_showing_bottom) => is_showing_bottom,
            _ => true,
        }
    }

    pub fn set_showing_bottom(&mut self, is_showing_bottom: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::end_crystal::show_bottom(),
            MetadataValue::Boolean(is_showing_bottom),
        );
    }
}

impl<'entity> Deref for EndCrystalMeta<'entity> {
    type Target = EntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.entity_meta
    }
}

impl<'entity> DerefMut for EndCrystalMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_meta
    }
}
