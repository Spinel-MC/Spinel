use crate::entity::metadata::{MetadataHolder, definitions};
use crate::entity::{
    Damage, EntityAttributeState, EntityEventNode, EntityId, EntityIdentity, EntityPointers,
    EntitySnapshot, EntityView, EquipmentSlot, LivingState, TimedPotionEffect,
};
use crate::network::client::instance::Client;
use crate::scheduler::{ContextScheduler, Task, TaskSchedule};
use crate::scoreboard::Team;
use crate::world::ChunkPosition;
use spinel_core::network::clientbound::play::entity_animation::{
    EntityAnimation, EntityAnimationPacket,
};
use spinel_core::network::clientbound::play::entity_effect::EntityEffectPacket;
use spinel_core::network::clientbound::play::entity_head_look::EntityHeadLookPacket;
use spinel_core::network::clientbound::play::entity_position::EntityPositionPacket;
use spinel_core::network::clientbound::play::entity_position_and_rotation::EntityPositionAndRotationPacket;
use spinel_core::network::clientbound::play::entity_rotation::EntityRotationPacket;
use spinel_core::network::clientbound::play::entity_status::EntityStatusPacket;
use spinel_core::network::clientbound::play::entity_teleport::EntityTeleportPacket;
use spinel_core::network::clientbound::play::entity_velocity::EntityVelocityPacket;
use spinel_core::network::clientbound::play::player_info_update::PlayerInfoUpdatePacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::remove_entity_effect::RemoveEntityEffectPacket;
use spinel_core::network::clientbound::play::set_entity_data::SetEntityDataPacket;
use spinel_core::network::clientbound::play::set_equipment::{
    EntityEquipmentEntry, SetEquipmentPacket,
};
use spinel_core::network::clientbound::play::set_passengers::SetPassengersPacket;
use spinel_core::network::clientbound::play::spawn_entity::{EntityAngle, SpawnEntityPacket};
use spinel_core::network::clientbound::play::update_attributes::UpdateAttributesPacket;
use spinel_nbt::{TagHandler, Taggable};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{IntList, Slot, TeleportFlags, Vector3d, Velocity};
use spinel_registry::{
    Attribute, DataComponentMap, DataComponentType, DataComponentValue, EntityBoundingBox,
    EntityType, ItemStack,
};
use spinel_utils::component::events::{HoverEntity, HoverEvent};
use spinel_utils::component::text::TextComponent;
use std::collections::BTreeSet;
use std::io;
use uuid::Uuid;

const MAX_ENTITY_COORDINATE: f64 = 2_000_000_000.0;

pub struct GenericEntity {
    entity_id: EntityId,
    uuid: Uuid,
    entity_type: EntityType,
    bounding_box: EntityBoundingBox,
    metadata: MetadataHolder,
    view: EntityView,
    position: EntityPosition,
    previous_position: EntityPosition,
    velocity: Velocity,
    equipment: EntityEquipment,
    vehicle: Option<EntityId>,
    passengers: BTreeSet<EntityId>,
    leashed_entities: BTreeSet<EntityId>,
    leash_holder: Option<EntityId>,
    world: Option<Uuid>,
    removed: bool,
    ticks: u64,
    scheduler: ContextScheduler<GenericEntity>,
    tag_handler: TagHandler,
    data_components: DataComponentMap,
    living: LivingState,
    event_node: EntityEventNode,
    aerodynamics: EntityAerodynamics,
    gravity_tick_count: u64,
    synchronization_ticks: u64,
    has_entity_collision: bool,
    prevents_block_placement: bool,
    delayed_remove_ticks: Option<u64>,
    expired_effects: Vec<TimedPotionEffect>,
}

#[derive(Clone, Debug)]
pub struct EntityTeleport {
    position: EntityPosition,
    velocity: Velocity,
    chunks: Option<Vec<i64>>,
    flags: TeleportFlags,
}

#[derive(Clone, Debug, PartialEq)]
struct EntityEquipment {
    main_hand: ItemStack,
    off_hand: ItemStack,
    boots: ItemStack,
    leggings: ItemStack,
    chestplate: ItemStack,
    helmet: ItemStack,
    body: ItemStack,
    saddle: ItemStack,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityAerodynamics {
    horizontal_air_resistance: f64,
    vertical_air_resistance: f64,
    gravity: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EntityPosition {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    head_yaw: f32,
}

impl GenericEntity {
    pub fn new(entity_type: EntityType) -> Self {
        Self::with_uuid(entity_type, Uuid::new_v4())
    }

    pub fn with_uuid(entity_type: EntityType, uuid: Uuid) -> Self {
        let entity_id = EntityId::next();
        Self {
            entity_id,
            uuid,
            entity_type,
            bounding_box: entity_type.bounding_box(),
            metadata: MetadataHolder::default(),
            view: EntityView::new(entity_id),
            position: EntityPosition::default(),
            previous_position: EntityPosition::default(),
            velocity: Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            equipment: EntityEquipment::default(),
            vehicle: None,
            passengers: BTreeSet::new(),
            leashed_entities: BTreeSet::new(),
            leash_holder: None,
            world: None,
            removed: false,
            ticks: 0,
            scheduler: ContextScheduler::new(),
            tag_handler: TagHandler::new_handler(),
            data_components: DataComponentMap::new(),
            living: LivingState::new(entity_type.bounding_box()),
            event_node: EntityEventNode::default(),
            aerodynamics: EntityAerodynamics::default(),
            gravity_tick_count: 0,
            synchronization_ticks: 1,
            has_entity_collision: true,
            prevents_block_placement: true,
            delayed_remove_ticks: None,
            expired_effects: Vec::new(),
        }
    }

    pub const fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub const fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub const fn identity(&self) -> EntityIdentity {
        EntityIdentity::new(self.uuid)
    }

    pub const fn pointers(&self) -> EntityPointers {
        EntityPointers::new(self.uuid, self.entity_id)
    }

    pub const fn entity_type(&self) -> EntityType {
        self.entity_type
    }

    pub fn as_hover_event(&self) -> HoverEvent {
        HoverEvent::ShowEntity(HoverEntity {
            entity_type: self.entity_type.key().to_string(),
            id: self.uuid.to_string(),
            name: self.custom_name().map(Box::new),
        })
    }

    pub fn event_node(&mut self) -> &mut EntityEventNode {
        &mut self.event_node
    }

    pub fn update_snapshot(&self, updater: impl FnOnce(&mut EntitySnapshot)) -> EntitySnapshot {
        let mut snapshot = EntitySnapshot::new(
            self.entity_id,
            self.uuid,
            self.entity_type,
            self.position,
            self.velocity,
            self.world,
            self.removed,
            self.metadata.clone(),
            self.custom_name(),
        );
        updater(&mut snapshot);
        snapshot
    }

    pub const fn world(&self) -> Option<Uuid> {
        self.world
    }

    pub(crate) fn set_world(&mut self, world: Uuid) {
        self.world = Some(world);
    }

    pub const fn bounding_box(&self) -> EntityBoundingBox {
        self.bounding_box
    }

    pub fn set_bounding_box(&mut self, bounding_box: EntityBoundingBox) {
        self.bounding_box = bounding_box;
        self.living.set_bounding_box(bounding_box);
    }

    pub fn set_bounding_box_dimensions(&mut self, width: f64, height: f64, depth: f64) {
        self.set_bounding_box(EntityBoundingBox::new(width, height, depth));
    }

    pub const fn metadata(&self) -> &MetadataHolder {
        &self.metadata
    }

    pub const fn metadata_mut(&mut self) -> &mut MetadataHolder {
        &mut self.metadata
    }

    pub fn component<T>(&self, component: DataComponentType<T>) -> Option<T>
    where
        T: DataComponentValue,
    {
        self.data_components.get_component(component)
    }

    pub fn set_component<T>(&mut self, component: DataComponentType<T>, value: T)
    where
        T: DataComponentValue,
    {
        self.data_components.set(component, value);
    }

    pub const fn data_components(&self) -> &DataComponentMap {
        &self.data_components
    }

    pub const fn view(&self) -> &EntityView {
        &self.view
    }

    pub const fn view_mut(&mut self) -> &mut EntityView {
        &mut self.view
    }

    pub fn viewers(&self) -> std::collections::BTreeSet<EntityId> {
        self.view.viewers()
    }

    pub fn add_viewer(&mut self, viewer_id: EntityId) -> bool {
        self.view.manual_add(viewer_id)
    }

    pub fn remove_viewer(&mut self, viewer_id: EntityId) -> bool {
        self.view.manual_remove(viewer_id)
    }

    pub fn is_viewer(&self, viewer_id: EntityId) -> bool {
        self.view.is_viewer(viewer_id)
    }

    pub const fn position(&self) -> EntityPosition {
        self.position
    }

    pub const fn previous_position(&self) -> EntityPosition {
        self.previous_position
    }

    pub fn set_position(&mut self, position: EntityPosition) {
        self.previous_position = self.position;
        self.position = position.clamped_to_minestom_entity_bounds();
    }

    pub fn refresh_position(&mut self, position: EntityPosition) {
        self.set_position(position);
    }

    pub fn refresh_position_with_packet_controls(
        &mut self,
        position: EntityPosition,
        _ignore_view: bool,
        _send_packets: bool,
    ) {
        self.refresh_position(position);
    }

    pub fn refresh_position_ignoring_view(&mut self, position: EntityPosition, ignore_view: bool) {
        self.refresh_position_with_packet_controls(position, ignore_view, true);
    }

    pub fn teleport(&mut self, position: EntityPosition) -> EntityTeleport {
        self.teleport_with_velocity_chunks_and_flags(
            position,
            Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            None,
            TeleportFlags::absolute(),
        )
    }

    pub fn teleport_with_velocity(
        &mut self,
        position: EntityPosition,
        velocity: Velocity,
    ) -> EntityTeleport {
        self.teleport_with_velocity_chunks_and_flags(
            position,
            velocity,
            None,
            TeleportFlags::absolute(),
        )
    }

    pub fn teleport_with_chunks_and_flags(
        &mut self,
        position: EntityPosition,
        chunks: impl Into<Option<Vec<i64>>>,
        flags: TeleportFlags,
    ) -> EntityTeleport {
        self.teleport_with_velocity_chunks_and_flags(
            position,
            Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            chunks,
            flags,
        )
    }

    pub fn teleport_with_velocity_chunks_and_flags(
        &mut self,
        position: EntityPosition,
        velocity: Velocity,
        chunks: impl Into<Option<Vec<i64>>>,
        flags: TeleportFlags,
    ) -> EntityTeleport {
        self.set_position(position);
        self.set_velocity(velocity);
        EntityTeleport::new(self.position, self.velocity, chunks.into(), flags)
    }

    pub const fn head_rotation(&self) -> f32 {
        self.position.head_yaw()
    }

    pub fn set_view(&mut self, yaw: f32, pitch: f32, head_yaw: f32) {
        self.set_position(
            EntityPosition::new(
                self.position.x(),
                self.position.y(),
                self.position.z(),
                yaw,
                pitch,
            )
            .with_head_yaw(head_yaw),
        );
    }

    pub fn look_at_position(&mut self, target: EntityPosition) {
        let delta_x = target.x() - self.position.x();
        let delta_y = target.y() - self.position.y();
        let delta_z = target.z() - self.position.z();
        let horizontal_distance = (delta_x.mul_add(delta_x, delta_z * delta_z)).sqrt();
        let yaw = delta_z.atan2(delta_x).to_degrees() as f32 - 90.0;
        let pitch = -(delta_y.atan2(horizontal_distance).to_degrees() as f32);

        self.set_view(yaw, pitch, yaw);
    }

    pub fn look_at_entity(&mut self, entity: &Self) {
        self.look_at_position(entity.position());
    }

    pub fn chunk(&self) -> ChunkPosition {
        ChunkPosition::new(
            (self.position.x().floor() as i32).div_euclid(16),
            (self.position.z().floor() as i32).div_euclid(16),
        )
    }

    pub fn distance_to_position(&self, position: EntityPosition) -> f64 {
        self.distance_squared_to_position(position).sqrt()
    }

    pub fn distance_to_entity(&self, entity: &GenericEntity) -> f64 {
        self.distance_to_position(entity.position())
    }

    pub fn distance_squared_to_position(&self, position: EntityPosition) -> f64 {
        self.position.distance_squared(position)
    }

    pub fn distance_squared_to_entity(&self, entity: &GenericEntity) -> f64 {
        self.distance_squared_to_position(entity.position())
    }

    pub const fn velocity(&self) -> Velocity {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }

    pub fn has_velocity(&self) -> bool {
        self.velocity.0.x != 0.0 || self.velocity.0.y != 0.0 || self.velocity.0.z != 0.0
    }

    pub const fn vehicle(&self) -> Option<EntityId> {
        self.vehicle
    }

    pub fn add_passenger(&mut self, passenger_id: EntityId) -> bool {
        self.passengers.insert(passenger_id)
    }

    pub fn remove_passenger(&mut self, passenger_id: EntityId) -> bool {
        self.passengers.remove(&passenger_id)
    }

    pub fn has_passenger(&self) -> bool {
        !self.passengers.is_empty()
    }

    pub fn passengers(&self) -> &BTreeSet<EntityId> {
        &self.passengers
    }

    pub fn passenger_packet(&self) -> SetPassengersPacket {
        SetPassengersPacket {
            vehicle_entity_id: self.entity_id.value(),
            passenger_entity_ids: IntList(
                self.passengers
                    .iter()
                    .map(|passenger_id| passenger_id.value())
                    .collect(),
            ),
        }
    }

    pub fn leashed_entities(&self) -> &BTreeSet<EntityId> {
        &self.leashed_entities
    }

    pub const fn leash_holder(&self) -> Option<EntityId> {
        self.leash_holder
    }

    pub fn set_leash_holder(&mut self, leash_holder: Option<EntityId>) {
        self.leash_holder = leash_holder;
    }

    pub fn add_leashed_entity(&mut self, entity_id: EntityId) -> bool {
        self.leashed_entities.insert(entity_id)
    }

    pub fn remove_leashed_entity(&mut self, entity_id: EntityId) -> bool {
        self.leashed_entities.remove(&entity_id)
    }

    pub fn status_packet(&self, status: i8) -> EntityStatusPacket {
        EntityStatusPacket {
            entity_id: self.entity_id.value(),
            status,
        }
    }

    pub fn is_on_fire(&self) -> bool {
        self.metadata.flag(&definitions::is_on_fire())
    }

    pub fn set_on_fire(&mut self, on_fire: bool) {
        self.metadata.set_flag(&definitions::is_on_fire(), on_fire);
    }

    pub fn is_invisible(&self) -> bool {
        self.metadata.flag(&definitions::is_invisible())
    }

    pub fn set_invisible(&mut self, invisible: bool) {
        self.metadata
            .set_flag(&definitions::is_invisible(), invisible);
    }

    pub fn is_glowing(&self) -> bool {
        self.metadata.flag(&definitions::has_glowing_effect())
    }

    pub fn set_glowing(&mut self, glowing: bool) {
        self.metadata
            .set_flag(&definitions::has_glowing_effect(), glowing);
    }

    pub fn pose(&self) -> i32 {
        match self.metadata.value(&definitions::pose()) {
            MetadataValue::Pose(pose) => pose,
            _ => 0,
        }
    }

    pub fn set_pose(&mut self, pose: i32) {
        self.metadata
            .set(&definitions::pose(), MetadataValue::Pose(pose));
        self.event_node.dispatch("EntityPoseEvent", self.entity_id);
    }

    pub fn custom_name(&self) -> Option<TextComponent> {
        match self.metadata.value(&definitions::custom_name()) {
            MetadataValue::OptionalText(custom_name) => custom_name,
            _ => None,
        }
    }

    pub fn set_custom_name(&mut self, custom_name: Option<TextComponent>) {
        self.metadata.set(
            &definitions::custom_name(),
            MetadataValue::OptionalText(custom_name),
        );
    }

    pub fn is_custom_name_visible(&self) -> bool {
        match self.metadata.value(&definitions::custom_name_visible()) {
            MetadataValue::Boolean(custom_name_visible) => custom_name_visible,
            _ => false,
        }
    }

    pub fn set_custom_name_visible(&mut self, custom_name_visible: bool) {
        self.metadata.set(
            &definitions::custom_name_visible(),
            MetadataValue::Boolean(custom_name_visible),
        );
    }

    pub fn is_silent(&self) -> bool {
        match self.metadata.value(&definitions::is_silent()) {
            MetadataValue::Boolean(silent) => silent,
            _ => false,
        }
    }

    pub fn set_silent(&mut self, silent: bool) {
        self.metadata
            .set(&definitions::is_silent(), MetadataValue::Boolean(silent));
    }

    pub fn has_no_gravity(&self) -> bool {
        match self.metadata.value(&definitions::has_no_gravity()) {
            MetadataValue::Boolean(no_gravity) => no_gravity,
            _ => false,
        }
    }

    pub fn set_no_gravity(&mut self, no_gravity: bool) {
        self.metadata.set(
            &definitions::has_no_gravity(),
            MetadataValue::Boolean(no_gravity),
        );
    }

    pub fn eye_height(&self) -> f64 {
        if self.pose() == 2 {
            return 0.2;
        }
        self.entity_type.eye_height()
    }

    pub fn equipment(&self, equipment_slot: EquipmentSlot) -> &ItemStack {
        self.equipment.item_stack(equipment_slot)
    }

    pub fn set_equipment(&mut self, equipment_slot: EquipmentSlot, item_stack: ItemStack) {
        let previous_item_stack = self.equipment.item_stack(equipment_slot).clone();
        self.equipment.set_item_stack(equipment_slot, item_stack);
        let current_item_stack = self.equipment.item_stack(equipment_slot);
        self.living.attributes_mut().update_equipment_attributes(
            &previous_item_stack,
            current_item_stack,
            equipment_slot,
        );
    }

    pub const fn arrow_count(&self) -> i32 {
        self.living.arrow_count()
    }

    pub fn set_arrow_count(&mut self, arrow_count: i32) {
        self.living.set_arrow_count(arrow_count);
    }

    pub const fn fire_ticks(&self) -> i32 {
        self.living.fire_ticks()
    }

    pub fn set_fire_ticks(&mut self, fire_ticks: i32) {
        self.living.set_fire_ticks(fire_ticks);
        self.set_on_fire(self.living.fire_ticks() > 0);
    }

    pub(crate) fn set_fire_ticks_after_cancelled_extinguish(&mut self, fire_ticks: i32) {
        self.living.set_fire_ticks(fire_ticks);
    }

    pub const fn health(&self) -> f32 {
        self.living.health()
    }

    pub fn set_health(&mut self, health: f32) {
        self.living.set_health(health);
    }

    pub const fn max_health(&self) -> f32 {
        self.living.max_health()
    }

    pub fn set_max_health(&mut self, max_health: f32) {
        self.living.set_max_health(max_health);
    }

    pub fn heal(&mut self) {
        self.living.heal();
    }

    pub const fn is_dead(&self) -> bool {
        self.living.is_dead()
    }

    pub const fn is_invulnerable(&self) -> bool {
        self.living.is_invulnerable()
    }

    pub fn set_invulnerable(&mut self, invulnerable: bool) {
        self.living.set_invulnerable(invulnerable);
    }

    pub fn damage(&mut self, damage_source: impl Into<String>, amount: f32) -> bool {
        let was_damaged = self.living.apply_untyped_damage(damage_source, amount);
        if was_damaged && self.health() <= 0.0 {
            self.kill();
        }
        was_damaged
    }

    pub(crate) fn apply_damage(&mut self, damage: Damage) {
        self.living.apply_damage(damage);
    }

    pub fn kill(&mut self) -> bool {
        if !self.living.kill() {
            return false;
        }
        self.set_pose(6);
        true
    }

    pub fn is_immune_to_damage(&self, _damage_source: &str) -> bool {
        self.living.is_invulnerable()
    }

    pub fn last_damage(&self) -> Option<&Damage> {
        self.living.last_damage()
    }

    pub fn last_damage_source(&self) -> Option<&str> {
        self.living.last_damage_source()
    }

    pub const fn item_pickup_cooldown(&self) -> u32 {
        self.living.item_pickup_cooldown()
    }

    pub const fn can_pickup_item(&self) -> bool {
        self.living.can_pickup_item()
    }

    pub fn set_can_pickup_item(&mut self, can_pickup_item: bool) {
        self.living.set_can_pickup_item(can_pickup_item);
    }

    pub fn set_item_pickup_cooldown(&mut self, item_pickup_cooldown: u32) {
        self.living.set_item_pickup_cooldown(item_pickup_cooldown);
    }

    pub const fn expanded_bounding_box(&self) -> EntityBoundingBox {
        self.living.expanded_bounding_box()
    }

    pub fn attribute(
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
        self.living.attribute(attribute)
    }

    pub fn attributes(&self) -> Vec<&EntityAttributeState> {
        self.living.attributes()
    }

    pub fn attribute_value(&self, attribute_id: i32, default_value: f64) -> f64 {
        Attribute::from_protocol_id(attribute_id)
            .map(|attribute| self.living.attribute_value(attribute))
            .unwrap_or(default_value)
    }

    pub fn update_attributes_packet(&self) -> UpdateAttributesPacket {
        self.living.update_attributes_packet(self.entity_id)
    }

    pub fn add_effect(&mut self, effect: TimedPotionEffect) -> EntityEffectPacket {
        self.living.add_effect(self.entity_id, effect)
    }

    pub(crate) fn take_expired_effects(&mut self) -> Vec<TimedPotionEffect> {
        std::mem::take(&mut self.expired_effects)
    }

    pub fn remove_effect(&mut self, effect_id: i32) -> Option<RemoveEntityEffectPacket> {
        self.living.remove_effect(self.entity_id, effect_id)
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
        self.living.clear_effects(self.entity_id)
    }

    pub fn effect_packets(&self) -> Vec<EntityEffectPacket> {
        self.living.effect_packets(self.entity_id)
    }

    pub fn swing_main_hand(&self) -> EntityAnimationPacket {
        self.animation_packet(EntityAnimation::SwingMainArm)
    }

    pub fn swing_off_hand(&self) -> EntityAnimationPacket {
        self.animation_packet(EntityAnimation::SwingOffHand)
    }

    pub fn swing_main_hand_from_client(&self, _from_client: bool) -> EntityAnimationPacket {
        self.swing_main_hand()
    }

    pub fn swing_off_hand_from_client(&self, _from_client: bool) -> EntityAnimationPacket {
        self.swing_off_hand()
    }

    pub fn animation_packet(&self, animation: EntityAnimation) -> EntityAnimationPacket {
        EntityAnimationPacket {
            entity_id: self.entity_id.value(),
            animation,
        }
    }

    pub const fn is_flying_with_elytra(&self) -> bool {
        self.living.is_flying_with_elytra()
    }

    pub fn set_flying_with_elytra(&mut self, is_flying_with_elytra: bool) {
        self.living.set_flying_with_elytra(is_flying_with_elytra);
        self.metadata
            .set_flag(&definitions::is_flying_with_elytra(), is_flying_with_elytra);
    }

    pub fn team(&self) -> Option<&str> {
        self.living.team()
    }

    pub fn set_team(&mut self, team: Option<String>) {
        self.living.set_team(team);
    }

    pub fn set_scoreboard_team(
        &mut self,
        mut previous_team: Option<&mut Team>,
        mut new_team: Option<&mut Team>,
    ) -> Vec<spinel_core::network::clientbound::play::set_player_team::SetPlayerTeamPacket> {
        let member = self.uuid.to_string();
        let mut packets = Vec::new();
        if let Some(previous_team_name) = self.living.team().map(str::to_owned) {
            self.living.set_team(None);
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
            self.living.set_team(Some(current_team.name().to_owned()));
            if let Some(packet) = current_team.add_member(member) {
                packets.push(packet);
            }
        }
        packets
    }

    pub const fn living_metadata(&self) -> &MetadataHolder {
        &self.metadata
    }

    pub fn take_knockback(&mut self, strength: f32, x: f64, z: f64) {
        self.velocity.0.x += x * f64::from(strength);
        self.velocity.0.z += z * f64::from(strength);
    }

    pub const fn aerodynamics(&self) -> EntityAerodynamics {
        self.aerodynamics
    }

    pub fn set_aerodynamics(&mut self, aerodynamics: EntityAerodynamics) {
        self.aerodynamics = aerodynamics;
    }

    pub const fn gravity_tick_count(&self) -> u64 {
        self.gravity_tick_count
    }

    pub fn increment_gravity_tick_count(&mut self) {
        self.gravity_tick_count += 1;
    }

    pub fn reset_gravity_tick_count(&mut self) {
        self.gravity_tick_count = 0;
    }

    pub const fn synchronization_ticks(&self) -> u64 {
        self.synchronization_ticks
    }

    pub fn set_synchronization_ticks(&mut self, synchronization_ticks: u64) {
        self.synchronization_ticks = synchronization_ticks;
    }

    pub fn synchronize_next_tick(&mut self) {
        self.synchronization_ticks = 1;
    }

    pub const fn has_entity_collision(&self) -> bool {
        self.has_entity_collision
    }

    pub fn set_entity_collision(&mut self, has_entity_collision: bool) {
        self.has_entity_collision = has_entity_collision;
    }

    pub const fn prevents_block_placement(&self) -> bool {
        self.prevents_block_placement
    }

    pub fn set_prevents_block_placement(&mut self, prevents_block_placement: bool) {
        self.prevents_block_placement = prevents_block_placement;
    }

    pub fn relative_start(&self) -> Vector3d {
        let half_width = self.bounding_box.width() / 2.0;
        let half_depth = self.bounding_box.depth() / 2.0;
        Vector3d {
            x: self.position.x() - half_width,
            y: self.position.y(),
            z: self.position.z() - half_depth,
        }
    }

    pub fn relative_end(&self) -> Vector3d {
        let half_width = self.bounding_box.width() / 2.0;
        let half_depth = self.bounding_box.depth() / 2.0;
        Vector3d {
            x: self.position.x() + half_width,
            y: self.position.y() + self.bounding_box.height(),
            z: self.position.z() + half_depth,
        }
    }

    pub fn intersects_box_at(&self, position: Vector3d, bounding_box: EntityBoundingBox) -> bool {
        boxes_intersect(
            self.relative_start(),
            self.relative_end(),
            box_start(position, bounding_box),
            box_end(position, bounding_box),
        )
    }

    pub fn intersects_box_swept(
        &self,
        ray_start: Vector3d,
        ray_direction: Vector3d,
        shape_position: Vector3d,
        moving_box: EntityBoundingBox,
    ) -> bool {
        let ray_end = Vector3d {
            x: ray_start.x + ray_direction.x,
            y: ray_start.y + ray_direction.y,
            z: ray_start.z + ray_direction.z,
        };
        boxes_intersect(
            swept_start(ray_start, ray_end),
            swept_end(ray_start, ray_end, moving_box),
            box_start(shape_position, moving_box),
            box_end(shape_position, moving_box),
        )
    }

    pub fn schedule_remove_ticks(&mut self, ticks: u64) {
        self.delayed_remove_ticks = Some(ticks);
    }

    pub fn schedule_remove_after_ticks(&mut self, delay_ticks: u64) {
        self.delayed_remove_ticks = Some(self.ticks.saturating_add(delay_ticks));
    }

    pub fn schedule_remove_after_duration(&mut self, duration: std::time::Duration) {
        let duration_millis = duration.as_millis();
        let delay_ticks = u64::try_from(duration_millis.div_ceil(50)).unwrap_or(u64::MAX);
        self.schedule_remove_after_ticks(delay_ticks);
    }

    pub fn enter_bed(&mut self, position: EntityPosition) {
        self.living.set_bed_position(Some(position));
        self.metadata
            .set(&definitions::pose(), MetadataValue::Pose(2));
    }

    pub fn leave_bed(&mut self) -> EntityAnimationPacket {
        self.living.set_bed_position(None);
        self.metadata
            .set(&definitions::pose(), MetadataValue::Pose(0));
        self.animation_packet(EntityAnimation::LeaveBed)
    }

    pub const fn bed_position(&self) -> Option<EntityPosition> {
        self.living.bed_position()
    }

    pub const fn is_removed(&self) -> bool {
        self.removed
    }

    pub const fn ticks(&self) -> u64 {
        self.ticks
    }

    pub const fn alive_ticks(&self) -> u64 {
        self.ticks
    }

    pub const fn is_active(&self) -> bool {
        self.world.is_some() && !self.removed
    }

    pub fn scheduler(&mut self) -> &mut ContextScheduler<GenericEntity> {
        &mut self.scheduler
    }

    pub fn schedule_next_tick(
        &mut self,
        callback: impl FnMut(&mut GenericEntity) -> TaskSchedule + Send + 'static,
    ) -> Task {
        self.scheduler.schedule_next_tick(callback)
    }

    pub fn switch_entity_type(&mut self, entity_type: EntityType) {
        self.entity_type = entity_type;
        self.metadata = MetadataHolder::default();
    }

    pub fn remove(&mut self) {
        self.removed = true;
    }

    pub fn spawn_packet(&self) -> SpawnEntityPacket {
        SpawnEntityPacket {
            entity_id: self.entity_id.value(),
            uuid: self.uuid,
            entity_type: self.entity_type.id(),
            x: self.position.x,
            y: self.position.y,
            z: self.position.z,
            velocity: self.velocity,
            pitch: EntityAngle(self.position.pitch),
            yaw: EntityAngle(self.position.yaw),
            head_yaw: EntityAngle(self.position.head_yaw),
            data: 0,
        }
    }

    pub fn metadata_packet(&self) -> SetEntityDataPacket {
        SetEntityDataPacket::new(self.entity_id.value(), self.metadata.entries())
    }

    pub fn equipment_packet(&self) -> SetEquipmentPacket {
        SetEquipmentPacket::new(self.entity_id.value(), self.equipment.entries())
    }

    pub fn velocity_packet(&self) -> EntityVelocityPacket {
        EntityVelocityPacket {
            entity_id: self.entity_id.value(),
            velocity: self.velocity,
        }
    }

    pub fn position_delta_packet(
        &self,
        previous_position: EntityPosition,
        on_ground: bool,
    ) -> EntityPositionPacket {
        EntityPositionPacket {
            entity_id: self.entity_id.value(),
            delta_x: EntityPositionPacket::delta(self.position.x, previous_position.x),
            delta_y: EntityPositionPacket::delta(self.position.y, previous_position.y),
            delta_z: EntityPositionPacket::delta(self.position.z, previous_position.z),
            on_ground,
        }
    }

    pub fn position_and_rotation_delta_packet(
        &self,
        previous_position: EntityPosition,
        on_ground: bool,
    ) -> EntityPositionAndRotationPacket {
        EntityPositionAndRotationPacket {
            entity_id: self.entity_id.value(),
            delta_x: EntityPositionPacket::delta(self.position.x, previous_position.x),
            delta_y: EntityPositionPacket::delta(self.position.y, previous_position.y),
            delta_z: EntityPositionPacket::delta(self.position.z, previous_position.z),
            yaw: EntityAngle(self.position.yaw),
            pitch: EntityAngle(self.position.pitch),
            on_ground,
        }
    }

    pub fn rotation_packet(&self, on_ground: bool) -> EntityRotationPacket {
        EntityRotationPacket {
            entity_id: self.entity_id.value(),
            yaw: EntityAngle(self.position.yaw),
            pitch: EntityAngle(self.position.pitch),
            on_ground,
        }
    }

    pub fn head_look_packet(&self) -> EntityHeadLookPacket {
        EntityHeadLookPacket {
            entity_id: self.entity_id.value(),
            head_yaw: EntityAngle(self.position.head_yaw),
        }
    }

    pub fn teleport_packet(&self) -> EntityTeleportPacket {
        EntityTeleportPacket {
            entity_id: self.entity_id.value(),
            position: self.position.as_vector(),
            delta: Vector3d {
                x: self.velocity.0.x,
                y: self.velocity.0.y,
                z: self.velocity.0.z,
            },
            yaw: self.position.yaw,
            pitch: self.position.pitch,
            flags: 0,
            on_ground: false,
        }
    }

    pub fn dirty_metadata_packet(&mut self) -> Option<SetEntityDataPacket> {
        let dirty_entries = self.metadata.drain_dirty_entries();
        if dirty_entries.is_empty() {
            return None;
        }
        Some(SetEntityDataPacket::new(
            self.entity_id.value(),
            dirty_entries,
        ))
    }

    pub fn remove_packet(&self) -> RemoveEntitiesPacket {
        RemoveEntitiesPacket::new(vec![self.entity_id.value()])
    }

    pub fn update_new_viewer(&self, client: &mut Client) -> io::Result<()> {
        if self.entity_type == EntityType::PLAYER {
            PlayerInfoUpdatePacket::add_listed_player(
                self.uuid,
                format!("test_player_{}", self.entity_id.value()),
            )
            .dispatch(client)?;
        }
        self.spawn_packet().dispatch(client)?;
        if self.has_velocity() {
            self.velocity_packet().dispatch(client)?;
        }
        self.metadata_packet().dispatch(client)?;
        self.equipment_packet().dispatch(client)?;
        self.head_look_packet().dispatch(client)?;
        if self.living.has_attributes() {
            self.update_attributes_packet().dispatch(client)?;
        }
        self.effect_packets()
            .into_iter()
            .try_for_each(|packet| packet.dispatch(client))
    }

    pub fn update_old_viewer(&self, client: &mut Client) -> io::Result<()> {
        self.leashed_entities.iter().try_for_each(|entity_id| {
            spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket {
                attached_entity_id: entity_id.value(),
                holding_entity_id: -1,
            }
            .dispatch(client)
        })?;
        self.remove_packet().dispatch(client)
    }

    pub const fn is_occluded(&self) -> bool {
        false
    }

    pub fn tick(&mut self) {
        if self.removed {
            return;
        }
        self.process_scheduler_tick_start();
        self.ticks += 1;
        self.process_living_tick();
        self.process_scheduler_tick_end();
    }

    pub fn update(&mut self, _time: u64) {
        self.tick();
    }

    pub fn trigger_status(&self, status: i8) -> EntityStatusPacket {
        self.status_packet(status)
    }

    fn process_living_tick(&mut self) {
        if self.living.fire_ticks() > 0 {
            self.living.tick_fire_ticks();
            self.set_on_fire(self.living.fire_ticks() > 0);
        }
        self.living.tick_item_pickup_cooldown();
        if self
            .delayed_remove_ticks
            .is_some_and(|remove_tick| remove_tick <= self.ticks)
        {
            self.remove();
        }
        self.expired_effects
            .extend(self.living.expire_effects_at(self.ticks));
    }

    fn process_scheduler_tick_start(&mut self) {
        let mut scheduler = std::mem::take(&mut self.scheduler);
        scheduler.process_tick(self);
        self.scheduler = scheduler;
    }

    fn process_scheduler_tick_end(&mut self) {
        let mut scheduler = std::mem::take(&mut self.scheduler);
        scheduler.process_tick_end(self);
        self.scheduler = scheduler;
    }
}

impl EntityEquipment {
    fn item_stack(&self, equipment_slot: EquipmentSlot) -> &ItemStack {
        match equipment_slot {
            EquipmentSlot::MainHand => &self.main_hand,
            EquipmentSlot::OffHand => &self.off_hand,
            EquipmentSlot::Boots => &self.boots,
            EquipmentSlot::Leggings => &self.leggings,
            EquipmentSlot::Chestplate => &self.chestplate,
            EquipmentSlot::Helmet => &self.helmet,
            EquipmentSlot::Body => &self.body,
            EquipmentSlot::Saddle => &self.saddle,
        }
    }

    fn set_item_stack(&mut self, equipment_slot: EquipmentSlot, item_stack: ItemStack) {
        match equipment_slot {
            EquipmentSlot::MainHand => self.main_hand = item_stack,
            EquipmentSlot::OffHand => self.off_hand = item_stack,
            EquipmentSlot::Boots => self.boots = item_stack,
            EquipmentSlot::Leggings => self.leggings = item_stack,
            EquipmentSlot::Chestplate => self.chestplate = item_stack,
            EquipmentSlot::Helmet => self.helmet = item_stack,
            EquipmentSlot::Body => self.body = item_stack,
            EquipmentSlot::Saddle => self.saddle = item_stack,
        }
    }

    fn entries(&self) -> Vec<EntityEquipmentEntry> {
        [
            EquipmentSlot::MainHand,
            EquipmentSlot::OffHand,
            EquipmentSlot::Boots,
            EquipmentSlot::Leggings,
            EquipmentSlot::Chestplate,
            EquipmentSlot::Helmet,
            EquipmentSlot::Body,
        ]
        .into_iter()
        .map(|equipment_slot| EntityEquipmentEntry {
            slot: equipment_slot.entity_equipment_slot(),
            item: Slot::from_item_stack(self.item_stack(equipment_slot)),
        })
        .collect()
    }
}

impl Default for EntityEquipment {
    fn default() -> Self {
        Self {
            main_hand: ItemStack::air(),
            off_hand: ItemStack::air(),
            boots: ItemStack::air(),
            leggings: ItemStack::air(),
            chestplate: ItemStack::air(),
            helmet: ItemStack::air(),
            body: ItemStack::air(),
            saddle: ItemStack::air(),
        }
    }
}

impl Taggable for GenericEntity {
    fn tag_handler(&self) -> &TagHandler {
        &self.tag_handler
    }

    fn tag_handler_mut(&mut self) -> &mut TagHandler {
        &mut self.tag_handler
    }
}

impl Default for EntityAerodynamics {
    fn default() -> Self {
        Self {
            horizontal_air_resistance: 0.91,
            vertical_air_resistance: 0.98,
            gravity: 0.08,
        }
    }
}

impl EntityAerodynamics {
    pub const fn new(
        horizontal_air_resistance: f64,
        vertical_air_resistance: f64,
        gravity: f64,
    ) -> Self {
        Self {
            horizontal_air_resistance,
            vertical_air_resistance,
            gravity,
        }
    }

    pub const fn horizontal_air_resistance(self) -> f64 {
        self.horizontal_air_resistance
    }

    pub const fn vertical_air_resistance(self) -> f64 {
        self.vertical_air_resistance
    }

    pub const fn gravity(self) -> f64 {
        self.gravity
    }
}

impl EntityTeleport {
    pub const fn new(
        position: EntityPosition,
        velocity: Velocity,
        chunks: Option<Vec<i64>>,
        flags: TeleportFlags,
    ) -> Self {
        Self {
            position,
            velocity,
            chunks,
            flags,
        }
    }

    pub const fn position(&self) -> EntityPosition {
        self.position
    }

    pub const fn velocity(&self) -> Velocity {
        self.velocity
    }

    pub fn chunks(&self) -> Option<&[i64]> {
        self.chunks.as_deref()
    }

    pub const fn flags(&self) -> TeleportFlags {
        self.flags
    }
}

impl EntityPosition {
    pub const fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Self {
        Self {
            x,
            y,
            z,
            yaw,
            pitch,
            head_yaw: yaw,
        }
    }

    pub const fn x(self) -> f64 {
        self.x
    }

    pub const fn y(self) -> f64 {
        self.y
    }

    pub const fn z(self) -> f64 {
        self.z
    }

    pub const fn yaw(self) -> f32 {
        self.yaw
    }

    pub const fn pitch(self) -> f32 {
        self.pitch
    }

    pub const fn head_yaw(self) -> f32 {
        self.head_yaw
    }

    pub const fn with_head_yaw(self, head_yaw: f32) -> Self {
        Self { head_yaw, ..self }
    }

    pub const fn as_vector(self) -> Vector3d {
        Vector3d {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub const fn offset(self, x: f64, y: f64, z: f64) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
            yaw: self.yaw,
            pitch: self.pitch,
            head_yaw: self.head_yaw,
        }
    }

    pub fn distance_squared(self, other: Self) -> f64 {
        let delta_x = self.x - other.x;
        let delta_y = self.y - other.y;
        let delta_z = self.z - other.z;
        delta_x * delta_x + delta_y * delta_y + delta_z * delta_z
    }

    pub fn clamped_to_minestom_entity_bounds(self) -> Self {
        Self {
            x: clamp_entity_coordinate(self.x),
            y: clamp_entity_coordinate(self.y),
            z: clamp_entity_coordinate(self.z),
            yaw: self.yaw,
            pitch: self.pitch,
            head_yaw: self.head_yaw,
        }
    }
}

fn clamp_entity_coordinate(coordinate: f64) -> f64 {
    coordinate.clamp(-MAX_ENTITY_COORDINATE, MAX_ENTITY_COORDINATE)
}

fn box_start(position: Vector3d, bounding_box: EntityBoundingBox) -> Vector3d {
    Vector3d {
        x: position.x - bounding_box.width() / 2.0,
        y: position.y,
        z: position.z - bounding_box.depth() / 2.0,
    }
}

fn box_end(position: Vector3d, bounding_box: EntityBoundingBox) -> Vector3d {
    Vector3d {
        x: position.x + bounding_box.width() / 2.0,
        y: position.y + bounding_box.height(),
        z: position.z + bounding_box.depth() / 2.0,
    }
}

fn swept_start(ray_start: Vector3d, ray_end: Vector3d) -> Vector3d {
    Vector3d {
        x: ray_start.x.min(ray_end.x),
        y: ray_start.y.min(ray_end.y),
        z: ray_start.z.min(ray_end.z),
    }
}

fn swept_end(ray_start: Vector3d, ray_end: Vector3d, moving_box: EntityBoundingBox) -> Vector3d {
    Vector3d {
        x: ray_start.x.max(ray_end.x) + moving_box.width(),
        y: ray_start.y.max(ray_end.y) + moving_box.height(),
        z: ray_start.z.max(ray_end.z) + moving_box.depth(),
    }
}

fn boxes_intersect(
    first_start: Vector3d,
    first_end: Vector3d,
    second_start: Vector3d,
    second_end: Vector3d,
) -> bool {
    first_start.x <= second_end.x
        && first_end.x >= second_start.x
        && first_start.y <= second_end.y
        && first_end.y >= second_start.y
        && first_start.z <= second_end.z
        && first_end.z >= second_start.z
}

#[cfg(test)]
mod tests {
    use super::{EntityAerodynamics, EntityPosition, GenericEntity, TimedPotionEffect};
    use crate::entity::metadata::definitions;
    use crate::entity::{EntityId, EquipmentSlot};
    use crate::world::ChunkPosition;
    use spinel_core::network::clientbound::play::entity_animation::EntityAnimation;
    use spinel_core::network::clientbound::play::update_attributes::EntityAttributeModifier;
    use spinel_nbt::{Tag, TagReadable, TagWritable};
    use spinel_network::types::entity_metadata::MetadataValue;
    use spinel_network::types::{Identifier, Vector3d};
    use spinel_registry::{DataComponentType, EntityBoundingBox, EntityType, ItemStack, Material};
    use spinel_utils::component::Component;

    #[test]
    fn generic_entity_owns_minestom_entity_identity_and_type() {
        let entity = GenericEntity::new(EntityType::ZOMBIE);

        assert!(entity.entity_id().value() > 0);
        assert_eq!(entity.entity_type(), EntityType::ZOMBIE);
        assert_eq!(entity.bounding_box(), EntityType::ZOMBIE.bounding_box());
        assert_eq!(entity.identity().uuid(), entity.uuid());
        assert_eq!(entity.pointers().uuid(), entity.uuid());
        assert_eq!(entity.pointers().entity_id(), entity.entity_id());
        assert_eq!(entity.pointers().identity(), entity.identity());
    }

    #[test]
    fn generic_entity_switch_type_preserves_bounding_box_like_minestom() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        let original_bounding_box = entity.bounding_box();

        entity.switch_entity_type(EntityType::ARROW);

        assert_eq!(entity.entity_type(), EntityType::ARROW);
        assert_eq!(entity.bounding_box(), original_bounding_box);
    }

    #[test]
    fn generic_entity_viewer_add_and_remove_match_minestom_no_op_return_values() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        let viewer_id = EntityId::next();

        assert!(entity.add_viewer(viewer_id));
        assert!(!entity.add_viewer(viewer_id));
        assert!(entity.is_viewer(viewer_id));
        assert!(entity.remove_viewer(viewer_id));
        assert!(!entity.remove_viewer(viewer_id));
        assert!(!entity.is_viewer(viewer_id));
    }

    #[test]
    fn generic_entity_bounding_box_distance_and_position_api_match_minestom_shape() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        let mut other_entity = GenericEntity::new(EntityType::ZOMBIE);

        entity.set_bounding_box_dimensions(1.0, 2.0, 3.0);
        entity.set_position(EntityPosition::new(1.0, 2.0, 3.0, 45.0, 15.0));
        entity.refresh_position(EntityPosition::new(4.0, 6.0, 3.0, 90.0, 30.0));
        entity.set_view(180.0, 60.0, 120.0);
        other_entity.set_position(EntityPosition::new(4.0, 9.0, 7.0, 0.0, 0.0));

        assert_eq!(entity.bounding_box(), EntityBoundingBox::new(1.0, 2.0, 3.0));
        assert_eq!(
            entity.previous_position(),
            EntityPosition::new(4.0, 6.0, 3.0, 90.0, 30.0)
        );
        assert_eq!(entity.position().yaw(), 180.0);
        assert_eq!(entity.position().pitch(), 60.0);
        assert_eq!(entity.head_rotation(), 120.0);
        assert_eq!(entity.distance_squared_to_entity(&other_entity), 25.0);
        assert_eq!(entity.distance_to_entity(&other_entity), 5.0);
    }

    #[test]
    fn generic_entity_look_at_and_chunk_api_match_minestom_shape() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        let mut target = GenericEntity::new(EntityType::ZOMBIE);

        entity.set_position(EntityPosition::new(-17.0, 64.0, 31.0, 0.0, 0.0));
        target.set_position(EntityPosition::new(-17.0, 64.0, 47.0, 0.0, 0.0));
        entity.look_at_entity(&target);

        assert_eq!(entity.chunk(), ChunkPosition::new(-2, 1));
        assert_eq!(entity.position().yaw(), 0.0);
        assert_eq!(entity.position().pitch(), -0.0);
        assert_eq!(entity.head_rotation(), 0.0);

        entity.look_at_position(EntityPosition::new(-1.0, 64.0, 31.0, 0.0, 0.0));

        assert_eq!(entity.position().yaw(), -90.0);
        assert_eq!(entity.head_rotation(), -90.0);
    }

    #[test]
    fn generic_entity_tag_handler_matches_minestom_entity_surface() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        let owner_tag = Tag::<String>::string("owner");

        entity.set_tag(&owner_tag, Some("spinel".to_string()));

        assert_eq!(entity.get_tag(&owner_tag), Some("spinel".to_string()));
    }

    #[test]
    fn generic_entity_passenger_leash_and_status_api_match_minestom_shape() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        let passenger_id = EntityId::next();
        let leash_holder_id = EntityId::next();
        let leashed_entity_id = EntityId::next();

        assert_eq!(entity.vehicle(), None);
        assert!(entity.add_passenger(passenger_id));
        assert!(!entity.add_passenger(passenger_id));
        assert!(entity.has_passenger());
        assert!(entity.passengers().contains(&passenger_id));

        let passenger_packet = entity.passenger_packet();
        assert_eq!(
            passenger_packet.vehicle_entity_id,
            entity.entity_id().value()
        );
        assert_eq!(
            passenger_packet.passenger_entity_ids.0,
            vec![passenger_id.value()]
        );

        entity.set_leash_holder(Some(leash_holder_id));
        assert_eq!(entity.leash_holder(), Some(leash_holder_id));
        assert!(entity.add_leashed_entity(leashed_entity_id));
        assert!(entity.leashed_entities().contains(&leashed_entity_id));
        assert!(entity.remove_leashed_entity(leashed_entity_id));
        assert!(entity.remove_passenger(passenger_id));
        assert!(!entity.has_passenger());

        let status_packet = entity.status_packet(3);
        let triggered_status_packet = entity.trigger_status(3);
        assert_eq!(status_packet.entity_id, entity.entity_id().value());
        assert_eq!(status_packet.status, 3);
        assert_eq!(triggered_status_packet.entity_id, status_packet.entity_id);
        assert_eq!(triggered_status_packet.status, status_packet.status);
    }

    #[test]
    fn generic_entity_base_metadata_api_matches_minestom_shape() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        let custom_name = Component::text("Name").build();

        assert!(!entity.is_on_fire());
        assert!(!entity.is_invisible());
        assert!(!entity.is_glowing());
        assert_eq!(entity.pose(), 0);
        assert_eq!(entity.custom_name(), None);
        assert!(!entity.is_custom_name_visible());
        assert!(!entity.is_silent());
        assert!(!entity.has_no_gravity());

        entity.set_on_fire(true);
        entity.set_invisible(true);
        entity.set_glowing(true);
        entity.set_pose(2);
        entity.set_custom_name(Some(custom_name.clone()));
        entity.set_custom_name_visible(true);
        entity.set_silent(true);
        entity.set_no_gravity(true);

        assert!(entity.is_on_fire());
        assert!(entity.is_invisible());
        assert!(entity.is_glowing());
        assert_eq!(entity.pose(), 2);
        assert_eq!(entity.custom_name(), Some(custom_name));
        assert!(entity.is_custom_name_visible());
        assert!(entity.is_silent());
        assert!(entity.has_no_gravity());
        assert_eq!(entity.eye_height(), 0.2);
        assert!(entity.dirty_metadata_packet().is_some());
    }

    #[test]
    fn generic_entity_data_components_match_minestom_entity_component_surface() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        let component =
            DataComponentType::<String>::new(9000, Identifier::vanilla_static("test_component"));

        assert_eq!(entity.component(component.clone()), None);
        entity.set_component(component.clone(), "value".to_string());

        assert_eq!(entity.component(component), Some("value".to_string()));
    }

    #[test]
    fn generic_living_state_damage_fire_and_death_match_minestom_surface() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);

        entity.set_arrow_count(4);
        entity.set_fire_ticks(2);
        entity.set_item_pickup_cooldown(2);
        assert!(entity.is_on_fire());
        assert_eq!(entity.arrow_count(), 4);
        assert_eq!(entity.fire_ticks(), 2);
        assert_eq!(entity.item_pickup_cooldown(), 2);

        assert!(entity.damage("minecraft:generic", 5.0));
        assert_eq!(entity.health(), 15.0);
        assert_eq!(entity.last_damage_source(), Some("minecraft:generic"));
        assert!(!entity.is_dead());

        entity.set_invulnerable(true);
        assert!(!entity.damage("minecraft:generic", 5.0));
        assert!(entity.is_immune_to_damage("minecraft:generic"));
        entity.set_invulnerable(false);
        assert!(entity.damage("minecraft:generic", 100.0));
        assert!(entity.is_dead());

        entity.heal();
        assert_eq!(entity.health(), entity.max_health());
        assert!(entity.is_dead());

        entity.tick();

        assert_eq!(entity.fire_ticks(), 1);
        assert_eq!(entity.item_pickup_cooldown(), 1);
    }

    #[test]
    fn generic_living_attributes_effects_animation_and_bed_api_match_minestom_surface() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        let modifier = EntityAttributeModifier::base_attack_speed(-2.0);

        entity.attribute(4, 4.0).add_modifier(modifier.clone());
        assert_eq!(entity.attribute_value(4, 4.0), 2.0);
        assert_eq!(entity.attributes().len(), 1);
        assert_eq!(entity.update_attributes_packet().attributes.len(), 1);
        assert_eq!(
            entity
                .attribute(4, 4.0)
                .remove_modifier(&modifier.modifier_id)
                .unwrap(),
            modifier
        );
        assert_eq!(entity.attribute_value(4, 4.0), 4.0);

        let effect_packet = entity.add_effect(TimedPotionEffect::new(1, 2, 2, 6, entity.ticks()));
        assert_eq!(effect_packet.effect_id, 1);
        assert!(entity.has_effect(1));
        assert_eq!(entity.effect_level(1), Some(2));
        assert_eq!(entity.active_effects().len(), 1);
        entity.tick();
        assert!(entity.has_effect(1));
        entity.tick();
        assert!(!entity.has_effect(1));

        entity.add_effect(TimedPotionEffect::new(2, 0, 20, 0, entity.ticks()));
        assert_eq!(entity.remove_effect(2).unwrap().effect_id, 2);
        entity.add_effect(TimedPotionEffect::new(3, 0, 20, 0, entity.ticks()));
        assert_eq!(entity.clear_effects().len(), 1);

        assert_eq!(
            entity.swing_main_hand().animation,
            EntityAnimation::SwingMainArm
        );
        assert_eq!(
            entity.swing_off_hand().animation,
            EntityAnimation::SwingOffHand
        );
        entity.enter_bed(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
        assert!(entity.bed_position().is_some());
        assert_eq!(entity.leave_bed().animation, EntityAnimation::LeaveBed);
        assert!(entity.bed_position().is_none());
    }

    #[test]
    fn generic_living_motion_team_and_collision_api_match_minestom_surface() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        let aerodynamics = EntityAerodynamics::new(0.8, 0.9, 0.04);

        assert_eq!(entity.expanded_bounding_box().width(), 1.6);
        entity.set_flying_with_elytra(true);
        assert!(entity.is_flying_with_elytra());
        entity.set_team(Some("red".to_string()));
        assert_eq!(entity.team(), Some("red"));
        assert!(!entity.living_metadata().entries().is_empty());

        entity.set_aerodynamics(aerodynamics);
        assert_eq!(entity.aerodynamics(), aerodynamics);
        entity.increment_gravity_tick_count();
        assert_eq!(entity.gravity_tick_count(), 1);
        entity.reset_gravity_tick_count();
        assert_eq!(entity.gravity_tick_count(), 0);

        entity.set_synchronization_ticks(5);
        assert_eq!(entity.synchronization_ticks(), 5);
        entity.synchronize_next_tick();
        assert_eq!(entity.synchronization_ticks(), 1);

        assert!(entity.has_entity_collision());
        entity.set_entity_collision(false);
        assert!(!entity.has_entity_collision());
        assert!(entity.prevents_block_placement());
        entity.set_prevents_block_placement(false);
        assert!(!entity.prevents_block_placement());

        assert_eq!(
            entity.relative_start(),
            Vector3d {
                x: -0.3,
                y: 0.0,
                z: -0.3
            }
        );
        assert_eq!(
            entity.relative_end(),
            Vector3d {
                x: 0.3,
                y: 1.95,
                z: 0.3
            }
        );
        assert!(entity.intersects_box_at(
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            EntityBoundingBox::new(0.6, 1.95, 0.6),
        ));
        assert!(!entity.intersects_box_at(
            Vector3d {
                x: 4.0,
                y: 0.0,
                z: 0.0
            },
            EntityBoundingBox::new(0.6, 1.95, 0.6),
        ));
        assert!(entity.intersects_box_swept(
            Vector3d {
                x: -1.0,
                y: 0.0,
                z: 0.0
            },
            Vector3d {
                x: 2.0,
                y: 0.0,
                z: 0.0
            },
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            EntityBoundingBox::new(0.6, 1.95, 0.6),
        ));

        entity.take_knockback(2.0, 0.5, -0.25);
        assert_eq!(entity.velocity().0.x, 1.0);
        assert_eq!(entity.velocity().0.z, -0.5);

        entity.schedule_remove_after_ticks(1);
        entity.tick();
        assert!(entity.is_removed());

        let mut duration_entity = GenericEntity::new(EntityType::ZOMBIE);
        duration_entity.schedule_remove_after_duration(std::time::Duration::from_millis(51));
        duration_entity.tick();
        assert!(!duration_entity.is_removed());
        duration_entity.tick();
        assert!(duration_entity.is_removed());
    }

    #[test]
    fn removed_generic_entity_does_not_tick() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);

        assert!(!entity.is_active());
        assert_eq!(entity.alive_ticks(), 0);

        entity.tick();

        assert_eq!(entity.ticks(), 1);
        assert_eq!(entity.alive_ticks(), 1);

        entity.update(20);

        assert_eq!(entity.ticks(), 2);
        assert_eq!(entity.alive_ticks(), 2);

        entity.remove();
        entity.tick();

        assert_eq!(entity.ticks(), 2);
        assert_eq!(entity.alive_ticks(), 2);
    }

    #[test]
    fn generic_entity_builds_spawn_and_metadata_packets_from_owned_state() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        entity.set_position(EntityPosition::new(1.0, 2.0, 3.0, 45.0, 90.0));
        entity
            .metadata_mut()
            .set(&definitions::air_ticks(), MetadataValue::VarInt(10));

        let spawn_packet = entity.spawn_packet();
        let metadata_packet = entity.dirty_metadata_packet().unwrap();

        assert_eq!(spawn_packet.entity_id, entity.entity_id().value());
        assert_eq!(spawn_packet.entity_type, EntityType::ZOMBIE.id());
        assert_eq!(spawn_packet.x, 1.0);
        assert_eq!(metadata_packet.entity_id, entity.entity_id().value());
        assert_eq!(metadata_packet.entries.0.len(), 1);
        assert!(entity.dirty_metadata_packet().is_none());
    }

    #[test]
    fn generic_entity_builds_minestom_equipment_packet_from_owned_equipment() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        entity.set_equipment(
            EquipmentSlot::MainHand,
            ItemStack::of(Material::DIAMOND_SWORD),
        );
        entity.set_equipment(
            EquipmentSlot::Helmet,
            ItemStack::of(Material::DIAMOND_HELMET),
        );

        let equipment_packet = entity.equipment_packet();

        assert_eq!(equipment_packet.entity_id, entity.entity_id().value());
        assert_eq!(equipment_packet.equipment.0.len(), 7);
        assert_eq!(
            equipment_packet.equipment.0[0]
                .item
                .to_item_stack()
                .material(),
            &Material::DIAMOND_SWORD
        );
        assert_eq!(
            equipment_packet.equipment.0[5]
                .item
                .to_item_stack()
                .material(),
            &Material::DIAMOND_HELMET
        );
    }

    #[test]
    fn generic_entity_builds_velocity_head_look_and_teleport_packets_from_owned_state() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        entity.set_position(EntityPosition::new(1.0, 2.0, 3.0, 45.0, 90.0).with_head_yaw(30.0));
        entity.set_velocity(spinel_network::types::Velocity(
            spinel_network::types::Vector3d {
                x: 0.25,
                y: 0.0,
                z: -0.25,
            },
        ));

        let velocity_packet = entity.velocity_packet();
        let head_look_packet = entity.head_look_packet();
        let teleport_packet = entity.teleport_packet();

        assert_eq!(velocity_packet.entity_id, entity.entity_id().value());
        assert_eq!(head_look_packet.entity_id, entity.entity_id().value());
        assert_eq!(head_look_packet.head_yaw.0, 30.0);
        assert_eq!(teleport_packet.entity_id, entity.entity_id().value());
        assert_eq!(teleport_packet.position, entity.position().as_vector());
    }

    #[test]
    fn generic_entity_builds_minestom_relative_movement_packets_from_previous_position() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        let previous_position = EntityPosition::new(1.0, 2.0, 3.0, 0.0, 0.0);
        entity.set_position(EntityPosition::new(1.5, 2.0, 2.5, 90.0, 45.0));

        let position_packet = entity.position_delta_packet(previous_position, true);
        let position_and_rotation_packet =
            entity.position_and_rotation_delta_packet(previous_position, false);
        let rotation_packet = entity.rotation_packet(true);

        assert_eq!(position_packet.delta_x, 2048);
        assert_eq!(position_packet.delta_z, -2048);
        assert!(position_packet.on_ground);
        assert_eq!(position_and_rotation_packet.delta_x, 2048);
        assert_eq!(position_and_rotation_packet.yaw.0, 90.0);
        assert!(!position_and_rotation_packet.on_ground);
        assert_eq!(rotation_packet.pitch.0, 45.0);
        assert!(rotation_packet.on_ground);
    }

    #[test]
    fn generic_entity_position_clamps_to_minestom_coordinate_limit() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);

        entity.set_position(EntityPosition::new(
            3_000_000_000.0,
            -3_000_000_000.0,
            12.0,
            0.0,
            0.0,
        ));

        assert_eq!(entity.position().x(), 2_000_000_000.0);
        assert_eq!(entity.position().y(), -2_000_000_000.0);
        assert_eq!(entity.position().z(), 12.0);
    }
}
