use crate::entity::{EntityAttributeState, Player, TimedPotionEffect};
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::remove_entity_effect::RemoveEntityEffectPacket;
use spinel_registry::{Attribute, EntityBoundingBox, MobEffect, RegistryKey};
use std::io;

impl Player {
    pub const fn get_max_health(&self) -> f32 {
        self.living.get_max_health()
    }

    pub fn set_max_health(&mut self, max_health: f32) -> io::Result<()> {
        self.living.set_max_health(max_health);
        self.sync_health()
    }

    pub fn heal(&mut self) -> io::Result<()> {
        self.living.heal();
        self.sync_health()
    }

    pub fn get_attribute(&mut self, attribute: Attribute) -> &mut EntityAttributeState {
        self.living.get_attribute(attribute)
    }

    pub fn get_attributes(&self) -> Vec<&EntityAttributeState> {
        self.living.get_attributes()
    }

    pub fn add_effect(&mut self, effect: TimedPotionEffect) -> EntityEffectPacket {
        self.living.add_effect(self.get_entity_id(), effect)
    }

    pub fn remove_effect(
        &mut self,
        effect_key: &RegistryKey<MobEffect>,
    ) -> Option<RemoveEntityEffectPacket> {
        self.living.remove_effect(self.get_entity_id(), effect_key)
    }

    pub fn has_effect(&self, effect_key: &RegistryKey<MobEffect>) -> bool {
        self.living.has_effect(effect_key)
    }

    pub fn get_effect(&self, effect_key: &RegistryKey<MobEffect>) -> Option<&TimedPotionEffect> {
        self.living.get_effect(effect_key)
    }

    pub fn get_effect_level(&self, effect_key: &RegistryKey<MobEffect>) -> i32 {
        self.get_effect(effect_key)
            .map_or(-1, TimedPotionEffect::get_amplifier)
    }

    pub fn get_active_effects(&self) -> Vec<&TimedPotionEffect> {
        self.living.get_active_effects()
    }

    pub fn clear_effects(&mut self) -> Vec<RemoveEntityEffectPacket> {
        self.living.clear_effects(self.get_entity_id())
    }

    pub fn get_effect_packets(&self) -> Vec<EntityEffectPacket> {
        self.living.get_effect_packets(self.get_entity_id())
    }

    pub(crate) fn expire_effects_at(&mut self, tick: u64) -> Vec<TimedPotionEffect> {
        self.living.expire_effects_at(tick)
    }

    pub(crate) fn tick_living_state(&mut self) -> Vec<TimedPotionEffect> {
        self.living.tick_item_pickup_cooldown();
        self.expire_effects_at(self.alive_ticks)
    }

    pub const fn get_item_pickup_cooldown(&self) -> u32 {
        self.living.get_item_pickup_cooldown()
    }

    pub fn set_item_pickup_cooldown(&mut self, item_pickup_cooldown: u32) {
        self.living.set_item_pickup_cooldown(item_pickup_cooldown);
    }

    pub const fn get_expanded_bounding_box(&self) -> EntityBoundingBox {
        self.living.get_expanded_bounding_box()
    }
}
