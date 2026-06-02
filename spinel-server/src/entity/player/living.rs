use crate::entity::{EntityAttributeState, Player, TimedPotionEffect};
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::remove_entity_effect::RemoveEntityEffectPacket;
use spinel_registry::{Attribute, EntityBoundingBox};
use std::io;

impl Player {
    pub const fn max_health(&self) -> f32 {
        self.living.max_health()
    }

    pub fn set_max_health(&mut self, max_health: f32) -> io::Result<()> {
        self.living.set_max_health(max_health);
        self.sync_health()
    }

    pub fn heal(&mut self) -> io::Result<()> {
        self.living.heal();
        self.sync_health()
    }

    pub fn attribute(&mut self, attribute: Attribute) -> &mut EntityAttributeState {
        self.living.attribute(attribute)
    }

    pub fn attributes(&self) -> Vec<&EntityAttributeState> {
        self.living.attributes()
    }

    pub fn add_effect(&mut self, effect: TimedPotionEffect) -> EntityEffectPacket {
        self.living.add_effect(self.entity_id(), effect)
    }

    pub fn remove_effect(&mut self, effect_id: i32) -> Option<RemoveEntityEffectPacket> {
        self.living.remove_effect(self.entity_id(), effect_id)
    }

    pub fn has_effect(&self, effect_id: i32) -> bool {
        self.living.has_effect(effect_id)
    }

    pub fn effect(&self, effect_id: i32) -> Option<&TimedPotionEffect> {
        self.living.effect(effect_id)
    }

    pub fn effect_level(&self, effect_id: i32) -> Option<i32> {
        self.effect(effect_id).map(TimedPotionEffect::amplifier)
    }

    pub fn active_effects(&self) -> Vec<&TimedPotionEffect> {
        self.living.active_effects()
    }

    pub fn clear_effects(&mut self) -> Vec<RemoveEntityEffectPacket> {
        self.living.clear_effects(self.entity_id())
    }

    pub fn effect_packets(&self) -> Vec<EntityEffectPacket> {
        self.living.effect_packets(self.entity_id())
    }

    pub(crate) fn expire_effects_at(&mut self, tick: u64) -> Vec<TimedPotionEffect> {
        self.living.expire_effects_at(tick)
    }

    pub(crate) fn tick_living_state(&mut self) -> Vec<TimedPotionEffect> {
        self.living.tick_item_pickup_cooldown();
        self.expire_effects_at(self.alive_ticks)
    }

    pub const fn item_pickup_cooldown(&self) -> u32 {
        self.living.item_pickup_cooldown()
    }

    pub fn set_item_pickup_cooldown(&mut self, item_pickup_cooldown: u32) {
        self.living.set_item_pickup_cooldown(item_pickup_cooldown);
    }

    pub const fn expanded_bounding_box(&self) -> EntityBoundingBox {
        self.living.expanded_bounding_box()
    }
}
