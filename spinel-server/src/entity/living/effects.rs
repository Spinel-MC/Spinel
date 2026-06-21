use crate::entity::EntityId;
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::remove_entity_effect::RemoveEntityEffectPacket;

#[derive(Clone, Debug, PartialEq)]
pub struct TimedPotionEffect {
    effect_id: i32,
    amplifier: i32,
    duration_ticks: i32,
    flags: i8,
    start_tick: u64,
}

impl TimedPotionEffect {
    pub const fn new(
        effect_id: i32,
        amplifier: i32,
        duration_ticks: i32,
        flags: i8,
        start_tick: u64,
    ) -> Self {
        Self {
            effect_id,
            amplifier,
            duration_ticks,
            flags,
            start_tick,
        }
    }

    pub const fn get_effect_id(&self) -> i32 {
        self.effect_id
    }

    pub const fn get_amplifier(&self) -> i32 {
        self.amplifier
    }

    pub const fn get_duration_ticks(&self) -> i32 {
        self.duration_ticks
    }

    pub const fn get_flags(&self) -> i8 {
        self.flags
    }

    pub const fn start_tick(&self) -> u64 {
        self.start_tick
    }

    pub fn is_expired_at(&self, tick: u64) -> bool {
        self.duration_ticks >= 0
            && tick.saturating_sub(self.start_tick) >= self.duration_ticks as u64
    }

    pub fn get_packet(&self, entity_id: EntityId) -> EntityEffectPacket {
        EntityEffectPacket {
            entity_id: entity_id.get_value(),
            effect_id: self.effect_id,
            amplifier: self.amplifier,
            duration_ticks: self.duration_ticks,
            flags: self.flags,
        }
    }

    pub fn remove_packet(&self, entity_id: EntityId) -> RemoveEntityEffectPacket {
        RemoveEntityEffectPacket {
            entity_id: entity_id.get_value(),
            effect_id: self.effect_id,
        }
    }
}
