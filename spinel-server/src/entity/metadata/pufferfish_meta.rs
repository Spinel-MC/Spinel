use crate::entity::metadata::{AbstractFishMeta, EntityMeta, PufferfishState, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct PufferfishMeta<'entity> {
    abstract_fish_meta: AbstractFishMeta<'entity>,
}

impl<'entity> PufferfishMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        if entity_meta.get_entity().get_entity_type() != EntityType::PUFFERFISH {
            return None;
        }
        let mut pufferfish_meta = Self {
            abstract_fish_meta: AbstractFishMeta::from_entity_meta(entity_meta),
        };
        pufferfish_meta.update_bounding_box(PufferfishState::Unpuffed);
        Some(pufferfish_meta)
    }

    pub fn get_state(&self) -> PufferfishState {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::puffer_fish::puff_state())
        {
            MetadataValue::VarInt(state) => {
                PufferfishState::from_protocol_id(state).unwrap_or_default()
            }
            _ => PufferfishState::default(),
        }
    }

    pub fn set_state(&mut self, state: PufferfishState) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::puffer_fish::puff_state(),
            MetadataValue::VarInt(state.get_protocol_id()),
        );
        self.update_bounding_box(state);
    }

    fn update_bounding_box(&mut self, state: PufferfishState) {
        let size = state.get_bounding_box_size();
        self.get_entity_mut()
            .set_bounding_box_dimensions(size, size, size);
    }
}

impl<'entity> Deref for PufferfishMeta<'entity> {
    type Target = AbstractFishMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_fish_meta
    }
}

impl<'entity> DerefMut for PufferfishMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_fish_meta
    }
}
