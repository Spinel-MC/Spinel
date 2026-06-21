use crate::entity::metadata::{
    AbstractGolemMeta, CopperGolemState, CopperGolemWeatherState, EntityMeta, definitions,
};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct CopperGolemMeta<'entity> {
    abstract_golem_meta: AbstractGolemMeta<'entity>,
}

impl<'entity> CopperGolemMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::COPPER_GOLEM).then(|| Self {
            abstract_golem_meta: AbstractGolemMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_weather_state(&self) -> CopperGolemWeatherState {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::copper_golem::get_weather_state())
        {
            MetadataValue::WeatherState(state_id) => {
                CopperGolemWeatherState::from_protocol_id(state_id).unwrap_or_default()
            }
            _ => CopperGolemWeatherState::default(),
        }
    }

    pub fn set_weather_state(&mut self, weather_state: CopperGolemWeatherState) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::copper_golem::get_weather_state(),
            MetadataValue::WeatherState(weather_state.get_protocol_id()),
        );
    }

    pub fn get_state(&self) -> CopperGolemState {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::copper_golem::get_state())
        {
            MetadataValue::CopperGolemState(state_id) => {
                CopperGolemState::from_protocol_id(state_id).unwrap_or_default()
            }
            _ => CopperGolemState::default(),
        }
    }

    pub fn set_state(&mut self, state: CopperGolemState) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::copper_golem::get_state(),
            MetadataValue::CopperGolemState(state.get_protocol_id()),
        );
    }
}

impl<'entity> Deref for CopperGolemMeta<'entity> {
    type Target = AbstractGolemMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_golem_meta
    }
}

impl<'entity> DerefMut for CopperGolemMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_golem_meta
    }
}
