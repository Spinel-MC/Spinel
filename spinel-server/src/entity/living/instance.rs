use crate::entity::metadata::{LivingEntityMetaCast, LivingEntityMetaRef};
use crate::entity::metadata::{MetadataHolder, definitions};
use crate::entity::physics::knockback_velocity;
use crate::entity::{
    Damage, EntityAttributeState, EntityId, EntityPose, EquipmentHandler, EquipmentSlot, Error,
    GenericEntity, LivingState, TimedPotionEffect,
};
use crate::scoreboard::Team;
use crate::world::World;
use spinel_core::network::clientbound::play::entity_animation::{
    EntityAnimation, EntityAnimationPacket,
};
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::remove_entity_effect::RemoveEntityEffectPacket;
use spinel_core::network::clientbound::play::set_equipment::{
    EntityEquipmentEntry, SetEquipmentPacket,
};
use spinel_core::network::clientbound::play::update_attributes::UpdateAttributesPacket;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{Slot, Vector3d, Velocity};
use spinel_registry::{
    Attribute, EntityBoundingBox, EntityType, ItemStack, MobEffect, RegistryKey,
};
use std::io;
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

pub struct LivingEntity {
    entity: GenericEntity,
    living_state: LivingState,
}

impl EquipmentHandler for LivingEntity {
    fn get_entity_id(&self) -> EntityId {
        LivingEntity::get_entity_id(self)
    }

    fn get_equipment(&self, equipment_slot: EquipmentSlot) -> ItemStack {
        LivingEntity::get_equipment(self, equipment_slot).clone()
    }

    fn set_equipment(
        &mut self,
        world: &mut World,
        equipment_slot: EquipmentSlot,
        item_stack: ItemStack,
    ) -> Result<(), Error> {
        let was_applied =
            world.set_entity_equipment(self.get_entity_id(), equipment_slot, item_stack)?;
        if !was_applied {
            return Err(Error::EquipmentMutationRejected);
        }
        Ok(())
    }
}
impl LivingEntity {
    pub fn new(entity_type: EntityType) -> Self {
        Self::with_uuid(entity_type, Uuid::new_v4())
    }

    pub fn with_uuid(entity_type: EntityType, uuid: Uuid) -> Self {
        Self {
            entity: GenericEntity::with_uuid(entity_type, uuid),
            living_state: LivingState::new(entity_type),
        }
    }

    pub const fn get_entity(&self) -> &GenericEntity {
        &self.entity
    }

    pub fn get_entity_meta(&self) -> LivingEntityMetaRef<'_> {
        LivingEntityMetaRef::new(self)
    }

    pub fn get_entity_meta_mut(&mut self) -> LivingEntityMetaCast<'_> {
        LivingEntityMetaCast::new(self)
    }

    pub fn get_entity_mut(&mut self) -> &mut GenericEntity {
        &mut self.entity
    }

    pub const fn get_entity_id(&self) -> EntityId {
        self.entity.get_entity_id()
    }

    pub fn get_equipment(&self, equipment_slot: EquipmentSlot) -> &ItemStack {
        self.living_state.get_equipment(equipment_slot)
    }

    pub fn get_equipment_packet(&self) -> SetEquipmentPacket {
        let equipment_slots = [
            EquipmentSlot::MainHand,
            EquipmentSlot::OffHand,
            EquipmentSlot::Boots,
            EquipmentSlot::Leggings,
            EquipmentSlot::Chestplate,
            EquipmentSlot::Helmet,
            EquipmentSlot::Body,
            EquipmentSlot::Saddle,
        ];
        let equipment = equipment_slots
            .into_iter()
            .map(|equipment_slot| EntityEquipmentEntry {
                slot: equipment_slot.get_entity_equipment_slot(),
                item: Slot::from_item_stack(self.get_equipment(equipment_slot)),
            })
            .collect();
        SetEquipmentPacket::new(self.get_entity_id().get_value(), equipment)
    }

    pub fn update_new_viewer(
        &self,
        client: &mut crate::network::client::instance::Client,
    ) -> io::Result<()> {
        self.entity.update_new_viewer(client)?;
        self.get_equipment_packet().dispatch(client)?;
        self.update_attributes_packet().dispatch(client)
    }

    pub fn tick(&mut self) {
        self.entity.tick();
        self.tick_living_state();
    }

    pub(crate) fn set_equipment_state(
        &mut self,
        equipment_slot: EquipmentSlot,
        item_stack: ItemStack,
    ) {
        self.living_state.set_equipment(equipment_slot, item_stack);
    }

    pub fn take_knockback(&mut self, strength: f32, x: f64, z: f64) {
        let living_strength = strength
            * (1.0
                - self.get_attribute_value(
                    Attribute::KNOCKBACK_RESISTANCE.protocol_id(),
                    Attribute::KNOCKBACK_RESISTANCE.default_value(),
                ) as f32);
        self.entity.set_velocity(knockback_velocity(
            self.entity.get_velocity(),
            self.entity.is_on_ground(),
            living_strength,
            x,
            z,
        ));
    }

    pub fn enter_bed(&mut self, position: crate::entity::EntityPosition) {
        self.living_state.set_bed_position(Some(position));
        self.entity
            .get_metadata_mut()
            .set(&definitions::get_pose(), MetadataValue::Pose(2));
    }

    pub fn leave_bed(&mut self) -> EntityAnimationPacket {
        self.living_state.set_bed_position(None);
        self.entity
            .get_metadata_mut()
            .set(&definitions::get_pose(), MetadataValue::Pose(0));
        self.get_animation_packet(EntityAnimation::LeaveBed)
    }

    pub const fn get_bed_position(&self) -> Option<crate::entity::EntityPosition> {
        self.living_state.get_bed_position()
    }

    pub const fn get_arrow_count(&self) -> i32 {
        self.living_state.get_arrow_count()
    }

    pub fn set_arrow_count(&mut self, arrow_count: i32) {
        self.living_state.set_arrow_count(arrow_count);
        self.entity.get_metadata_mut().set(
            &definitions::living_entity::number_of_arrows(),
            MetadataValue::VarInt(self.living_state.get_arrow_count()),
        );
    }

    pub const fn get_fire_ticks(&self) -> i32 {
        self.living_state.get_fire_ticks()
    }

    pub fn set_fire_ticks(&mut self, fire_ticks: i32) {
        self.living_state.set_fire_ticks(fire_ticks);
        self.entity
            .set_on_fire(self.living_state.get_fire_ticks() > 0);
    }

    pub(crate) fn set_fire_ticks_after_cancelled_extinguish(&mut self, fire_ticks: i32) {
        self.living_state.set_fire_ticks(fire_ticks);
    }

    pub const fn get_health(&self) -> f32 {
        self.living_state.get_health()
    }

    pub fn set_health(&mut self, health: f32) {
        self.living_state.set_health(health);
        self.entity.get_metadata_mut().set(
            &definitions::living_entity::get_health(),
            MetadataValue::Float(self.living_state.get_health()),
        );
        if self.living_state.get_health() <= 0.0 {
            self.kill();
        }
    }

    pub const fn get_max_health(&self) -> f32 {
        self.living_state.get_max_health()
    }

    pub fn set_max_health(&mut self, max_health: f32) {
        self.living_state.set_max_health(max_health);
    }

    pub fn heal(&mut self) {
        self.set_health(self.get_max_health());
    }

    pub const fn is_dead(&self) -> bool {
        self.living_state.is_dead()
    }

    pub const fn is_invulnerable(&self) -> bool {
        self.living_state.is_invulnerable()
    }

    pub fn set_invulnerable(&mut self, invulnerable: bool) {
        self.living_state.set_invulnerable(invulnerable);
    }

    pub fn damage(&mut self, damage_source: impl Into<String>, amount: f32) -> bool {
        let was_damaged = self
            .living_state
            .apply_untyped_damage(damage_source, amount);
        if was_damaged && self.get_health() <= 0.0 {
            self.kill();
        }
        was_damaged
    }

    pub(crate) fn apply_damage(&mut self, damage: Damage) {
        self.living_state.apply_damage(damage);
    }

    pub fn kill(&mut self) -> bool {
        if !self.living_state.kill() {
            return false;
        }
        self.entity.set_pose(EntityPose::Dying);
        self.entity.set_velocity(Velocity(Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }));
        true
    }

    pub fn is_immune_to_damage(&self, _damage_source: &str) -> bool {
        self.living_state.is_invulnerable()
    }

    pub fn get_last_damage(&self) -> Option<&Damage> {
        self.living_state.get_last_damage()
    }

    pub fn get_last_damage_source(&self) -> Option<&str> {
        self.living_state.get_last_damage_source()
    }

    pub const fn get_item_pickup_cooldown(&self) -> u32 {
        self.living_state.get_item_pickup_cooldown()
    }

    pub const fn can_pickup_item(&self) -> bool {
        self.living_state.can_pickup_item()
    }

    pub fn set_can_pickup_item(&mut self, can_pickup_item: bool) {
        self.living_state.set_can_pickup_item(can_pickup_item);
    }

    pub fn set_item_pickup_cooldown(&mut self, item_pickup_cooldown: u32) {
        self.living_state
            .set_item_pickup_cooldown(item_pickup_cooldown);
    }

    pub const fn get_expanded_bounding_box(&self) -> EntityBoundingBox {
        self.living_state.get_expanded_bounding_box()
    }

    pub fn get_attribute(
        &mut self,
        attribute_id: i32,
        default_value: f64,
    ) -> &mut EntityAttributeState {
        let attribute = Attribute::from_protocol_id(attribute_id).unwrap_or_else(|| {
            Attribute::new(
                attribute_id,
                "unknown",
                default_value,
                f64::MIN,
                f64::MAX,
                true,
            )
        });
        self.living_state.get_attribute(attribute)
    }

    pub fn get_attributes(&self) -> Vec<&EntityAttributeState> {
        self.living_state.get_attributes()
    }

    pub fn get_attribute_value(&self, attribute_id: i32, default_value: f64) -> f64 {
        Attribute::from_protocol_id(attribute_id)
            .map(|attribute| self.living_state.get_attribute_value(attribute))
            .unwrap_or(default_value)
    }

    pub fn update_attributes_packet(&self) -> UpdateAttributesPacket {
        self.living_state
            .update_attributes_packet(self.entity.get_entity_id())
    }

    pub fn has_attributes(&self) -> bool {
        self.living_state.has_attributes()
    }

    pub fn add_effect(&mut self, effect: TimedPotionEffect) -> EntityEffectPacket {
        self.living_state
            .add_effect(self.entity.get_entity_id(), effect)
    }

    pub(crate) fn tick_living_state(&mut self) -> Vec<TimedPotionEffect> {
        self.living_state.tick_fire_ticks();
        self.living_state.tick_item_pickup_cooldown();
        self.living_state.expire_effects_at(self.entity.get_ticks())
    }

    pub fn remove_effect(
        &mut self,
        effect_key: &RegistryKey<MobEffect>,
    ) -> Option<RemoveEntityEffectPacket> {
        self.living_state
            .remove_effect(self.entity.get_entity_id(), effect_key)
    }

    pub fn has_effect(&self, effect_key: &RegistryKey<MobEffect>) -> bool {
        self.living_state.has_effect(effect_key)
    }

    pub fn get_effect(&self, effect_key: &RegistryKey<MobEffect>) -> Option<&TimedPotionEffect> {
        self.living_state.get_effect(effect_key)
    }

    pub fn get_effect_level(&self, effect_key: &RegistryKey<MobEffect>) -> Option<i32> {
        self.get_effect(effect_key)
            .map(TimedPotionEffect::get_amplifier)
    }

    pub fn get_active_effects(&self) -> Vec<&TimedPotionEffect> {
        self.living_state.get_active_effects()
    }

    pub fn clear_effects(&mut self) -> Vec<RemoveEntityEffectPacket> {
        self.living_state.clear_effects(self.entity.get_entity_id())
    }

    pub fn get_effect_packets(&self) -> Vec<EntityEffectPacket> {
        self.living_state
            .get_effect_packets(self.entity.get_entity_id())
    }

    pub fn swing_main_hand(&self) -> EntityAnimationPacket {
        self.get_animation_packet(EntityAnimation::SwingMainArm)
    }

    pub fn swing_off_hand(&self) -> EntityAnimationPacket {
        self.get_animation_packet(EntityAnimation::SwingOffHand)
    }

    pub fn swing_main_hand_from_client(&self, _from_client: bool) -> EntityAnimationPacket {
        self.swing_main_hand()
    }

    pub fn swing_off_hand_from_client(&self, _from_client: bool) -> EntityAnimationPacket {
        self.swing_off_hand()
    }

    pub fn get_animation_packet(&self, animation: EntityAnimation) -> EntityAnimationPacket {
        EntityAnimationPacket {
            entity_id: self.entity.get_entity_id().get_value(),
            animation,
        }
    }

    pub const fn is_flying_with_elytra(&self) -> bool {
        self.living_state.is_flying_with_elytra()
    }

    pub fn set_flying_with_elytra(&mut self, is_flying_with_elytra: bool) {
        self.living_state
            .set_flying_with_elytra(is_flying_with_elytra);
        self.entity
            .get_metadata_mut()
            .set_flag(&definitions::is_flying_with_elytra(), is_flying_with_elytra);
    }

    pub fn get_team(&self) -> Option<&str> {
        self.living_state.get_team()
    }

    pub fn set_team(&mut self, team: Option<String>) {
        self.living_state.set_team(team);
    }

    pub fn set_scoreboard_team(
        &mut self,
        mut previous_team: Option<&mut Team>,
        mut new_team: Option<&mut Team>,
    ) -> Vec<spinel_core::network::clientbound::play::set_player_team::SetPlayerTeamPacket> {
        let member = self.entity.get_uuid().to_string();
        let mut packets = Vec::new();
        if let Some(previous_team_name) = self.living_state.get_team().map(str::to_owned) {
            self.living_state.set_team(None);
            let should_remove_previous_member = new_team
                .as_ref()
                .is_none_or(|current_team| current_team.name() != previous_team_name);
            if should_remove_previous_member {
                if let Some(previous_team) = previous_team.as_mut() {
                    if let Some(packet) = previous_team.remove_member(&member) {
                        packets.push(packet);
                    }
                } else {
                    packets.push(
                        spinel_core::network::clientbound::play::set_player_team::SetPlayerTeamPacket {
                            team_name: previous_team_name,
                            action: spinel_core::network::clientbound::play::set_player_team::TeamAction::RemoveEntities(vec![member.clone()]),
                        },
                    );
                }
            }
        }
        if let Some(current_team) = new_team.as_mut() {
            self.living_state
                .set_team(Some(current_team.name().to_owned()));
            if let Some(packet) = current_team.add_member(member) {
                packets.push(packet);
            }
        }
        packets
    }

    pub const fn get_living_metadata(&self) -> &MetadataHolder {
        self.entity.get_metadata()
    }
}
impl Deref for LivingEntity {
    type Target = GenericEntity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl DerefMut for LivingEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
