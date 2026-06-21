use crate::entity::metadata::{AnimalMeta, EntityMeta, SnifferState, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct SnifferMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> SnifferMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::SNIFFER).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_state(&self) -> SnifferState {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::sniffer::get_state())
        {
            MetadataValue::SnifferState(state) => {
                SnifferState::from_protocol_id(state).unwrap_or_default()
            }
            _ => SnifferState::default(),
        }
    }

    pub fn set_state(&mut self, state: SnifferState) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::sniffer::get_state(),
            MetadataValue::SnifferState(state.get_protocol_id()),
        );
    }

    pub fn get_drop_seed_at_tick(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::sniffer::get_drop_seed_at_tick())
        {
            MetadataValue::VarInt(drop_seed_at_tick) => drop_seed_at_tick,
            _ => 0,
        }
    }

    pub fn set_drop_seed_at_tick(&mut self, drop_seed_at_tick: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::sniffer::get_drop_seed_at_tick(),
            MetadataValue::VarInt(drop_seed_at_tick),
        );
    }
}

impl<'entity> Deref for SnifferMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for SnifferMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
