use crate::entity::metadata::definitions;
use crate::entity::{EntityState, LivingEntity, PlayerHand};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{Particle, Position};
use std::ops::Deref;

#[derive(Clone, Copy)]
pub struct LivingEntityMetaRef<'entity> {
    living_entity: &'entity LivingEntity,
}

impl<'entity> LivingEntityMetaRef<'entity> {
    pub(crate) const fn new(living_entity: &'entity LivingEntity) -> Self {
        Self { living_entity }
    }

    pub(crate) const fn get_entity_state(&self) -> &EntityState {
        self.living_entity.get_entity().get_state()
    }

    pub fn is_hand_active(&self) -> bool {
        self.get_entity_state()
            .get_metadata()
            .get_flag(&definitions::living_entity::is_hand_active())
    }

    pub fn get_active_hand(&self) -> PlayerHand {
        if self
            .get_entity_state()
            .get_metadata()
            .get_flag(&definitions::living_entity::get_active_hand())
        {
            return PlayerHand::Off;
        }
        PlayerHand::Main
    }

    pub fn get_health(&self) -> f32 {
        match self
            .get_entity_state()
            .get_metadata()
            .get_value(&definitions::living_entity::get_health())
        {
            MetadataValue::Float(health) => health,
            _ => 1.0,
        }
    }

    pub fn get_effect_particles(&self) -> Vec<Particle> {
        match self
            .get_entity_state()
            .get_metadata()
            .get_value(&definitions::living_entity::potion_effect_particles())
        {
            MetadataValue::ParticleList(effect_particles) => effect_particles,
            _ => Vec::new(),
        }
    }

    pub fn is_potion_effect_ambient(&self) -> bool {
        match self
            .get_entity_state()
            .get_metadata()
            .get_value(&definitions::living_entity::is_potion_effect_ambient())
        {
            MetadataValue::Boolean(is_potion_effect_ambient) => is_potion_effect_ambient,
            _ => false,
        }
    }

    pub fn get_arrow_count(&self) -> i32 {
        match self
            .get_entity_state()
            .get_metadata()
            .get_value(&definitions::living_entity::number_of_arrows())
        {
            MetadataValue::VarInt(arrow_count) => arrow_count,
            _ => 0,
        }
    }

    pub fn get_bee_stinger_count(&self) -> i32 {
        match self
            .get_entity_state()
            .get_metadata()
            .get_value(&definitions::living_entity::number_of_bee_stingers())
        {
            MetadataValue::VarInt(bee_stinger_count) => bee_stinger_count,
            _ => 0,
        }
    }

    pub fn get_bed_in_which_sleeping_position(&self) -> Option<Position> {
        match self
            .get_entity_state()
            .get_metadata()
            .get_value(&definitions::living_entity::location_of_bed())
        {
            MetadataValue::OptionalPosition(bed_position) => bed_position,
            _ => None,
        }
    }
}

impl<'entity> Deref for LivingEntityMetaRef<'entity> {
    type Target = LivingEntity;

    fn deref(&self) -> &Self::Target {
        self.living_entity
    }
}
