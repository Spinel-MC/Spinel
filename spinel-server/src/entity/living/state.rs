use crate::entity::{
    Damage, EntityAttributeState, EntityId, EntityPosition, EquipmentSlot, LivingAttributes,
};
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::remove_entity_effect::RemoveEntityEffectPacket;
use spinel_core::network::clientbound::play::set_equipment::EntityEquipmentEntry;
use spinel_core::network::clientbound::play::update_attributes::UpdateAttributesPacket;
use spinel_registry::{
    Attribute, EntityBoundingBox, EntityType, ItemStack, MobEffect, RegistryKey,
};
use std::collections::BTreeMap;

use super::{LivingEquipment, TimedPotionEffect};

#[derive(Clone, Debug, PartialEq)]
pub struct LivingState {
    arrow_count: i32,
    fire_ticks: i32,
    health: f32,
    max_health: f32,
    dead: bool,
    invulnerable: bool,
    last_damage: Option<Damage>,
    last_damage_source: Option<String>,
    item_pickup_cooldown: u32,
    can_pickup_item: bool,
    expanded_bounding_box: EntityBoundingBox,
    team: Option<String>,
    bed_position: Option<EntityPosition>,
    is_flying_with_elytra: bool,
    equipment: LivingEquipment,
    attributes: LivingAttributes,
    effects: BTreeMap<RegistryKey<MobEffect>, TimedPotionEffect>,
}

impl LivingState {
    pub fn new(entity_type: EntityType) -> Self {
        let attributes = LivingAttributes::from_entity_type(entity_type);
        let max_health = attributes.get_attribute_value(Attribute::MAX_HEALTH) as f32;
        Self {
            arrow_count: 0,
            fire_ticks: 0,
            health: 1.0_f32.min(max_health),
            max_health,
            dead: false,
            invulnerable: false,
            last_damage: None,
            last_damage_source: None,
            item_pickup_cooldown: 0,
            can_pickup_item: true,
            expanded_bounding_box: expanded_bounding_box(entity_type.get_bounding_box()),
            team: None,
            bed_position: None,
            is_flying_with_elytra: false,
            equipment: LivingEquipment::default(),
            attributes,
            effects: BTreeMap::new(),
        }
    }

    pub const fn get_arrow_count(&self) -> i32 {
        self.arrow_count
    }

    pub fn set_arrow_count(&mut self, arrow_count: i32) {
        self.arrow_count = arrow_count.max(0);
    }

    pub const fn get_fire_ticks(&self) -> i32 {
        self.fire_ticks
    }

    pub fn set_fire_ticks(&mut self, fire_ticks: i32) {
        self.fire_ticks = fire_ticks.max(0);
    }

    pub fn tick_fire_ticks(&mut self) {
        if self.fire_ticks > 0 {
            self.fire_ticks -= 1;
        }
    }

    pub const fn get_health(&self) -> f32 {
        self.health
    }

    pub fn set_health(&mut self, health: f32) {
        self.health = health.clamp(0.0, self.max_health);
    }

    pub const fn get_max_health(&self) -> f32 {
        self.max_health
    }

    pub fn set_max_health(&mut self, max_health: f32) {
        self.max_health = max_health.max(0.0);
        self.health = self.health.min(self.max_health);
    }

    pub fn heal(&mut self) {
        self.set_health(self.max_health);
    }

    pub const fn is_dead(&self) -> bool {
        self.dead
    }

    pub const fn is_invulnerable(&self) -> bool {
        self.invulnerable
    }

    pub fn set_invulnerable(&mut self, invulnerable: bool) {
        self.invulnerable = invulnerable;
    }

    pub fn apply_untyped_damage(&mut self, damage_source: impl Into<String>, amount: f32) -> bool {
        if self.invulnerable || self.dead {
            return false;
        }
        self.last_damage_source = Some(damage_source.into());
        self.set_health(self.health - amount.max(0.0));
        true
    }

    pub fn apply_damage(&mut self, damage: Damage) {
        self.last_damage_source = Some(damage.damage_type().key().to_string());
        self.last_damage = Some(damage.clone());
        self.set_health(self.health - damage.get_amount());
    }

    pub fn store_last_damage(&mut self, damage: Damage) {
        self.last_damage_source = Some(damage.damage_type().key().to_string());
        self.last_damage = Some(damage);
    }

    pub fn kill(&mut self) -> bool {
        if self.dead {
            return false;
        }
        self.set_health(0.0);
        self.dead = true;
        true
    }

    pub fn revive(&mut self) {
        self.dead = false;
        if self.health <= 0.0 {
            self.health = self.max_health;
        }
    }

    pub fn get_last_damage(&self) -> Option<&Damage> {
        self.last_damage.as_ref()
    }

    pub fn get_last_damage_source(&self) -> Option<&str> {
        self.last_damage_source.as_deref()
    }

    pub const fn get_item_pickup_cooldown(&self) -> u32 {
        self.item_pickup_cooldown
    }

    pub fn set_item_pickup_cooldown(&mut self, item_pickup_cooldown: u32) {
        self.item_pickup_cooldown = item_pickup_cooldown;
    }

    pub fn tick_item_pickup_cooldown(&mut self) {
        self.item_pickup_cooldown = self.item_pickup_cooldown.saturating_sub(1);
    }

    pub const fn can_pickup_item(&self) -> bool {
        self.can_pickup_item
    }

    pub fn set_can_pickup_item(&mut self, can_pickup_item: bool) {
        self.can_pickup_item = can_pickup_item;
    }

    pub const fn get_expanded_bounding_box(&self) -> EntityBoundingBox {
        self.expanded_bounding_box
    }

    pub fn set_bounding_box(&mut self, bounding_box: EntityBoundingBox) {
        self.expanded_bounding_box = expanded_bounding_box(bounding_box);
    }

    pub fn get_attribute(&mut self, attribute: Attribute) -> &mut EntityAttributeState {
        self.attributes.get_attribute(attribute)
    }

    pub fn get_attributes(&self) -> Vec<&EntityAttributeState> {
        self.attributes.get_attributes()
    }

    pub fn get_attribute_value(&self, attribute: Attribute) -> f64 {
        self.attributes.get_attribute_value(attribute)
    }

    pub fn get_attributes_mut(&mut self) -> &mut LivingAttributes {
        &mut self.attributes
    }

    pub fn get_equipment(&self, equipment_slot: EquipmentSlot) -> &ItemStack {
        self.equipment.get_item_stack(equipment_slot)
    }

    pub fn set_equipment(&mut self, equipment_slot: EquipmentSlot, item_stack: ItemStack) {
        let previous_item_stack = self.equipment.get_item_stack(equipment_slot).clone();
        self.equipment.set_item_stack(equipment_slot, item_stack);
        self.attributes.update_equipment_attributes(
            &previous_item_stack,
            self.equipment.get_item_stack(equipment_slot),
            equipment_slot,
        );
    }

    pub fn get_visible_equipment_entries(&self) -> Vec<EntityEquipmentEntry> {
        self.equipment.get_visible_entries()
    }

    pub fn update_attributes_packet(&self, entity_id: EntityId) -> UpdateAttributesPacket {
        self.attributes.get_packet(entity_id.get_value())
    }

    pub fn has_attributes(&self) -> bool {
        !self.attributes.is_empty()
    }

    pub fn add_effect(
        &mut self,
        entity_id: EntityId,
        effect: TimedPotionEffect,
    ) -> EntityEffectPacket {
        let packet = effect.get_packet(entity_id);
        self.effects.insert(effect.get_effect_key().clone(), effect);
        packet
    }

    pub fn remove_effect(
        &mut self,
        entity_id: EntityId,
        effect_key: &RegistryKey<MobEffect>,
    ) -> Option<RemoveEntityEffectPacket> {
        self.effects
            .remove(effect_key)
            .map(|effect| effect.remove_packet(entity_id))
    }

    pub fn has_effect(&self, effect_key: &RegistryKey<MobEffect>) -> bool {
        self.effects.contains_key(effect_key)
    }

    pub fn get_effect(&self, effect_key: &RegistryKey<MobEffect>) -> Option<&TimedPotionEffect> {
        self.effects.get(effect_key)
    }

    pub fn get_active_effects(&self) -> Vec<&TimedPotionEffect> {
        self.effects.values().collect()
    }

    pub fn clear_effects(&mut self, entity_id: EntityId) -> Vec<RemoveEntityEffectPacket> {
        std::mem::take(&mut self.effects)
            .into_values()
            .map(|effect| effect.remove_packet(entity_id))
            .collect()
    }

    pub fn get_effect_packets(&self, entity_id: EntityId) -> Vec<EntityEffectPacket> {
        self.effects
            .values()
            .map(|effect| effect.get_packet(entity_id))
            .collect()
    }

    pub fn expire_effects_at(&mut self, tick: u64) -> Vec<TimedPotionEffect> {
        let expired_effect_keys = self
            .effects
            .iter()
            .filter_map(|(effect_key, effect)| {
                effect.is_expired_at(tick).then_some(effect_key.clone())
            })
            .collect::<Vec<_>>();
        expired_effect_keys
            .into_iter()
            .filter_map(|effect_key| self.effects.remove(&effect_key))
            .collect()
    }

    pub fn get_team(&self) -> Option<&str> {
        self.team.as_deref()
    }

    pub fn set_team(&mut self, team: Option<String>) {
        self.team = team;
    }

    pub const fn get_bed_position(&self) -> Option<EntityPosition> {
        self.bed_position
    }

    pub fn set_bed_position(&mut self, bed_position: Option<EntityPosition>) {
        self.bed_position = bed_position;
    }

    pub const fn is_flying_with_elytra(&self) -> bool {
        self.is_flying_with_elytra
    }

    pub fn set_flying_with_elytra(&mut self, is_flying_with_elytra: bool) {
        self.is_flying_with_elytra = is_flying_with_elytra;
    }
}

fn expanded_bounding_box(bounding_box: EntityBoundingBox) -> EntityBoundingBox {
    EntityBoundingBox::new(
        bounding_box.get_width() + 1.0,
        bounding_box.get_height() + 0.5,
        bounding_box.depth() + 1.0,
    )
}
