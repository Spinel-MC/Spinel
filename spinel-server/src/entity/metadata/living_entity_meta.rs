use crate::entity::metadata::{
    MetadataBitMaskDefinition, MetadataByteMaskDefinition, MetadataDefinition, definitions,
};
use crate::entity::{EntityState, LivingEntity, PlayerHand};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{Particle, Position};
use std::ops::{Deref, DerefMut};

pub struct LivingEntityMeta<'entity> {
    living_entity: &'entity mut LivingEntity,
}

impl<'entity> LivingEntityMeta<'entity> {
    pub(crate) fn new(living_entity: &'entity mut LivingEntity) -> Self {
        Self { living_entity }
    }

    pub(crate) const fn get_living_entity(&self) -> &LivingEntity {
        self.living_entity
    }

    pub(crate) fn get_living_entity_mut(&mut self) -> &mut LivingEntity {
        self.living_entity
    }

    pub(crate) const fn get_entity_state(&self) -> &EntityState {
        self.living_entity.get_entity().get_state()
    }

    pub(crate) fn get_entity_state_mut(&mut self) -> &mut EntityState {
        self.living_entity.get_entity_mut().get_state_mut()
    }

    pub fn set_notify_about_changes(&mut self, should_notify_about_changes: bool) {
        self.get_entity_state_mut()
            .get_metadata_mut()
            .set_change_notifications_enabled(should_notify_about_changes);
    }

    pub fn get_metadata_value(&self, definition: &MetadataDefinition) -> MetadataValue {
        self.get_entity_state().get_metadata().get_value(definition)
    }

    pub fn set_metadata_value(
        &mut self,
        definition: &MetadataDefinition,
        metadata_value: MetadataValue,
    ) {
        self.get_entity_state_mut()
            .get_metadata_mut()
            .set(definition, metadata_value);
    }

    pub fn get_metadata_flag(&self, definition: &MetadataBitMaskDefinition) -> bool {
        self.get_entity_state().get_metadata().get_flag(definition)
    }

    pub fn set_metadata_flag(&mut self, definition: &MetadataBitMaskDefinition, is_enabled: bool) {
        self.get_entity_state_mut()
            .get_metadata_mut()
            .set_flag(definition, is_enabled);
    }

    pub fn get_metadata_byte(&self, definition: &MetadataByteMaskDefinition) -> i8 {
        self.get_entity_state().get_metadata().get_byte(definition)
    }

    pub fn set_metadata_byte(&mut self, definition: &MetadataByteMaskDefinition, byte_value: i8) {
        self.get_entity_state_mut()
            .get_metadata_mut()
            .set_byte(definition, byte_value);
    }

    pub fn is_hand_active(&self) -> bool {
        self.get_metadata_flag(&definitions::living_entity::is_hand_active())
    }

    pub fn set_hand_active(&mut self, is_hand_active: bool) {
        self.set_metadata_flag(
            &definitions::living_entity::is_hand_active(),
            is_hand_active,
        );
    }

    pub fn get_active_hand(&self) -> PlayerHand {
        if self.get_metadata_flag(&definitions::living_entity::get_active_hand()) {
            return PlayerHand::Off;
        }
        PlayerHand::Main
    }

    pub fn set_active_hand(&mut self, active_hand: PlayerHand) {
        self.set_metadata_flag(
            &definitions::living_entity::get_active_hand(),
            active_hand == PlayerHand::Off,
        );
    }

    pub fn is_in_riptide_spin_attack(&self) -> bool {
        self.get_metadata_flag(&definitions::living_entity::is_riptide_spin_attack())
    }

    pub fn set_in_riptide_spin_attack(&mut self, is_in_riptide_spin_attack: bool) {
        self.set_metadata_flag(
            &definitions::living_entity::is_riptide_spin_attack(),
            is_in_riptide_spin_attack,
        );
    }

    pub fn get_health(&self) -> f32 {
        match self.get_metadata_value(&definitions::living_entity::get_health()) {
            MetadataValue::Float(health) => health,
            _ => 1.0,
        }
    }

    pub fn set_health(&mut self, health: f32) {
        self.set_metadata_value(
            &definitions::living_entity::get_health(),
            MetadataValue::Float(health),
        );
    }

    pub fn get_effect_particles(&self) -> Vec<Particle> {
        match self.get_metadata_value(&definitions::living_entity::potion_effect_particles()) {
            MetadataValue::ParticleList(effect_particles) => effect_particles,
            _ => Vec::new(),
        }
    }

    pub fn set_effect_particles(&mut self, effect_particles: Vec<Particle>) {
        self.set_metadata_value(
            &definitions::living_entity::potion_effect_particles(),
            MetadataValue::ParticleList(effect_particles),
        );
    }

    pub fn is_potion_effect_ambient(&self) -> bool {
        match self.get_metadata_value(&definitions::living_entity::is_potion_effect_ambient()) {
            MetadataValue::Boolean(is_potion_effect_ambient) => is_potion_effect_ambient,
            _ => false,
        }
    }

    pub fn set_potion_effect_ambient(&mut self, is_potion_effect_ambient: bool) {
        self.set_metadata_value(
            &definitions::living_entity::is_potion_effect_ambient(),
            MetadataValue::Boolean(is_potion_effect_ambient),
        );
    }

    pub fn get_arrow_count(&self) -> i32 {
        match self.get_metadata_value(&definitions::living_entity::number_of_arrows()) {
            MetadataValue::VarInt(arrow_count) => arrow_count,
            _ => 0,
        }
    }

    pub fn set_arrow_count(&mut self, arrow_count: i32) {
        self.set_metadata_value(
            &definitions::living_entity::number_of_arrows(),
            MetadataValue::VarInt(arrow_count),
        );
    }

    pub fn get_bee_stinger_count(&self) -> i32 {
        match self.get_metadata_value(&definitions::living_entity::number_of_bee_stingers()) {
            MetadataValue::VarInt(bee_stinger_count) => bee_stinger_count,
            _ => 0,
        }
    }

    pub fn set_bee_stinger_count(&mut self, bee_stinger_count: i32) {
        self.set_metadata_value(
            &definitions::living_entity::number_of_bee_stingers(),
            MetadataValue::VarInt(bee_stinger_count),
        );
    }

    pub fn get_bed_in_which_sleeping_position(&self) -> Option<Position> {
        match self.get_metadata_value(&definitions::living_entity::location_of_bed()) {
            MetadataValue::OptionalPosition(bed_position) => bed_position,
            _ => None,
        }
    }

    pub fn set_bed_in_which_sleeping_position(&mut self, bed_position: Option<Position>) {
        self.set_metadata_value(
            &definitions::living_entity::location_of_bed(),
            MetadataValue::OptionalPosition(bed_position),
        );
    }
}

impl<'entity> Deref for LivingEntityMeta<'entity> {
    type Target = LivingEntity;

    fn deref(&self) -> &Self::Target {
        self.living_entity
    }
}

impl<'entity> DerefMut for LivingEntityMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.living_entity
    }
}
