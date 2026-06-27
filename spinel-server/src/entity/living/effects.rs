use crate::entity::EntityId;
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::remove_entity_effect::RemoveEntityEffectPacket;
use spinel_registry::{MobEffect, RegistryKey};

#[derive(Clone, Debug, PartialEq)]
pub struct TimedPotionEffect {
    effect_key: RegistryKey<MobEffect>,
    protocol_id: i32,
    amplifier: i32,
    duration_ticks: i32,
    flags: i8,
    start_tick: u64,
}

impl TimedPotionEffect {
    pub const AMBIENT_FLAG: i8 = 0x01;
    pub const PARTICLES_FLAG: i8 = 0x02;
    pub const ICON_FLAG: i8 = 0x04;
    pub const BLEND_FLAG: i8 = 0x08;
    pub const INFINITE_DURATION: i32 = -1;

    pub const fn new(
        effect_key: RegistryKey<MobEffect>,
        protocol_id: i32,
        amplifier: i32,
        duration_ticks: i32,
        flags: i8,
        start_tick: u64,
    ) -> Self {
        Self {
            effect_key,
            protocol_id,
            amplifier,
            duration_ticks,
            flags,
            start_tick,
        }
    }

    pub const fn get_effect_key(&self) -> &RegistryKey<MobEffect> {
        &self.effect_key
    }

    #[must_use]
    pub const fn get_protocol_id(&self) -> i32 {
        self.protocol_id
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

    pub const fn get_start_tick(&self) -> u64 {
        self.start_tick
    }

    pub const fn is_ambient(&self) -> bool {
        self.flags & Self::AMBIENT_FLAG == Self::AMBIENT_FLAG
    }

    pub const fn has_particles(&self) -> bool {
        self.flags & Self::PARTICLES_FLAG == Self::PARTICLES_FLAG
    }

    pub const fn has_icon(&self) -> bool {
        self.flags & Self::ICON_FLAG == Self::ICON_FLAG
    }

    pub const fn has_blend(&self) -> bool {
        self.flags & Self::BLEND_FLAG == Self::BLEND_FLAG
    }

    pub fn is_expired_at(&self, tick: u64) -> bool {
        self.duration_ticks >= 0
            && tick.saturating_sub(self.start_tick) >= self.duration_ticks as u64
    }

    pub fn get_packet(&self, entity_id: EntityId) -> EntityEffectPacket {
        EntityEffectPacket {
            entity_id: entity_id.get_value(),
            effect_id: self.protocol_id,
            amplifier: self.amplifier,
            duration_ticks: self.duration_ticks,
            flags: self.flags,
        }
    }

    pub fn remove_packet(&self, entity_id: EntityId) -> RemoveEntityEffectPacket {
        RemoveEntityEffectPacket {
            entity_id: entity_id.get_value(),
            effect_id: self.protocol_id,
        }
    }
}
