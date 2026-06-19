use crate::entity::metadata::{
    AxolotlVariant, CopperGolemState, CopperGolemWeatherState, CreeperState, EntityMeta,
    FoxVariant, HorseVariant, LlamaVariant, MetadataHolder, MooshroomVariant, PandaGene,
    ParrotColor, PufferfishState, RabbitVariant, SalmonSize, SnifferState, SpellcasterIllagerSpell,
    TropicalFishPattern, TropicalFishVariant, VillagerData, definitions,
};
use crate::entity::physics::{
    EntityMovement, EntityMovementPacket, EntityPhysicsResult, knockback_velocity,
    simulate_movement,
};
use crate::entity::{
    Damage, EntityAttributeState, EntityCollisionRules, EntityEventNode, EntityId, EntityIdentity,
    EntityLeash, EntityPointers, EntitySnapshot, EntitySynchronization, EntityTeleport, EntityView,
    EquipmentSlot, LivingState, PlayerHand, TimedPotionEffect,
};
use crate::network::client::instance::Client;
use crate::scheduler::{ContextScheduler, Task, TaskSchedule};
use crate::scoreboard::Team;
use crate::world::{ChunkPosition, WorldSnapshot};
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
use spinel_core::network::clientbound::play::set_equipment::SetEquipmentPacket;
use spinel_core::network::clientbound::play::set_passengers::SetPassengersPacket;
use spinel_core::network::clientbound::play::spawn_entity::{EntityAngle, SpawnEntityPacket};
use spinel_core::network::clientbound::play::update_attributes::UpdateAttributesPacket;
use spinel_nbt::{TagHandler, Taggable};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{
    IntList, MainHand, Particle, Position, Quaternionf, ResolvableProfile, Slot, TeleportFlags,
    Vector3d, Vector3f, Velocity,
};
use spinel_registry::{
    Attribute, BlockFaceDirection, BlockState, DataComponentMap, DataComponentType,
    DataComponentValue, EntityBoundingBox, EntityType, ItemStack, VillagerType,
};
use spinel_utils::color::DyeColor;
use spinel_utils::component::events::{HoverEntity, HoverEvent};
use spinel_utils::component::text::TextComponent;
use std::collections::BTreeSet;
use std::io;
use uuid::Uuid;

const MAX_ENTITY_COORDINATE: f64 = 2_000_000_000.0;
const SERVER_TICKS_PER_SECOND: f64 = 20.0;

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
    vehicle: Option<EntityId>,
    passengers: BTreeSet<EntityId>,
    leash: EntityLeash,
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
    previous_physics_result: Option<EntityPhysicsResult>,
    synchronization: EntitySynchronization,
    on_ground: bool,
    has_physics: bool,
    has_entity_collision: bool,
    prevents_block_placement: bool,
    delayed_remove_ticks: Option<u64>,
    expired_effects: Vec<TimedPotionEffect>,
    falling_block_state: i32,
    fishing_hook_owner_entity_id: Option<EntityId>,
}

enum MovementSynchronizationTiming {
    BeforeEntityTick,
    AfterEntityTick,
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
        let collision_rules = EntityCollisionRules::from_entity_type(entity_type);
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
            vehicle: None,
            passengers: BTreeSet::new(),
            leash: EntityLeash::new(),
            world: None,
            removed: false,
            ticks: 0,
            scheduler: ContextScheduler::new(),
            tag_handler: TagHandler::new_handler(),
            data_components: DataComponentMap::new(),
            living: LivingState::new(entity_type),
            event_node: EntityEventNode::default(),
            aerodynamics: EntityAerodynamics::from_entity_type(entity_type),
            gravity_tick_count: 0,
            previous_physics_result: None,
            synchronization: EntitySynchronization::new(EntityPosition::default()),
            on_ground: false,
            has_physics: true,
            has_entity_collision: collision_rules.has_entity_collision(),
            prevents_block_placement: collision_rules.prevents_block_placement(),
            delayed_remove_ticks: None,
            expired_effects: Vec::new(),
            falling_block_state: spinel_registry::vanilla_world_blocks::Block::STONE.state_id(),
            fishing_hook_owner_entity_id: None,
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

    pub fn entity_meta_mut(&mut self) -> EntityMeta<'_> {
        EntityMeta::new(self)
    }

    pub fn component<T>(&self, component: DataComponentType<T>) -> Option<T>
    where
        T: DataComponentValue,
    {
        if component.id()
            == spinel_registry::data_components::vanilla_components::VILLAGER_VARIANT.id()
        {
            return self.villager_variant_component().and_then(|villager_type| {
                T::from_component_nbt(&villager_type.to_component_nbt())
            });
        }
        if component.id() == spinel_registry::data_components::vanilla_components::CAT_COLLAR.id()
            && self.entity_type == EntityType::CAT
        {
            return T::from_component_nbt(&self.cat_collar_color().to_component_nbt());
        }
        if component.id() == spinel_registry::data_components::vanilla_components::WOLF_COLLAR.id()
            && self.entity_type == EntityType::WOLF
        {
            return T::from_component_nbt(&self.wolf_collar_color().to_component_nbt());
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::SHULKER_COLOR.id()
            && self.entity_type == EntityType::SHULKER
        {
            return T::from_component_nbt(&self.shulker_color().to_component_nbt());
        }
        if component.id() == spinel_registry::data_components::vanilla_components::FOX_VARIANT.id()
            && self.entity_type == EntityType::FOX
        {
            return T::from_component_nbt(&self.fox_variant().to_component_nbt());
        }
        if component.id() == spinel_registry::data_components::vanilla_components::SALMON_SIZE.id()
            && self.entity_type == EntityType::SALMON
        {
            return T::from_component_nbt(&self.salmon_size().to_component_nbt());
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::PARROT_VARIANT.id()
            && self.entity_type == EntityType::PARROT
        {
            return T::from_component_nbt(&self.parrot_color().to_component_nbt());
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::TROPICAL_FISH_PATTERN.id()
            && self.entity_type == EntityType::TROPICAL_FISH
        {
            return T::from_component_nbt(
                &self.tropical_fish_variant().pattern().to_component_nbt(),
            );
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::TROPICAL_FISH_BASE_COLOR.id()
            && self.entity_type == EntityType::TROPICAL_FISH
        {
            return T::from_component_nbt(
                &self.tropical_fish_variant().base_color().to_component_nbt(),
            );
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::TROPICAL_FISH_PATTERN_COLOR
                .id()
            && self.entity_type == EntityType::TROPICAL_FISH
        {
            return T::from_component_nbt(
                &self
                    .tropical_fish_variant()
                    .pattern_color()
                    .to_component_nbt(),
            );
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::MOOSHROOM_VARIANT.id()
            && self.entity_type == EntityType::MOOSHROOM
        {
            return T::from_component_nbt(&self.mooshroom_variant().to_component_nbt());
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::RABBIT_VARIANT.id()
            && self.entity_type == EntityType::RABBIT
        {
            return T::from_component_nbt(&self.rabbit_variant().to_component_nbt());
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::HORSE_VARIANT.id()
            && self.entity_type == EntityType::HORSE
        {
            return T::from_component_nbt(&self.horse_variant().color().to_component_nbt());
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::LLAMA_VARIANT.id()
            && (self.entity_type == EntityType::LLAMA
                || self.entity_type == EntityType::TRADER_LLAMA)
        {
            return T::from_component_nbt(&self.llama_variant().to_component_nbt());
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::AXOLOTL_VARIANT.id()
            && self.entity_type == EntityType::AXOLOTL
        {
            return T::from_component_nbt(&self.axolotl_variant().to_component_nbt());
        }
        self.data_components.get_component(component)
    }

    pub fn set_component<T>(&mut self, component: DataComponentType<T>, value: T)
    where
        T: DataComponentValue,
    {
        if component.id()
            == spinel_registry::data_components::vanilla_components::VILLAGER_VARIANT.id()
        {
            let Some(villager_type) = VillagerType::from_component_nbt(&value.to_component_nbt())
            else {
                return;
            };
            self.set_villager_variant_component(villager_type);
            return;
        }
        if component.id() == spinel_registry::data_components::vanilla_components::CAT_COLLAR.id()
            && self.entity_type == EntityType::CAT
        {
            let Some(color) = DyeColor::from_component_nbt(&value.to_component_nbt()) else {
                return;
            };
            self.set_cat_collar_color(color);
            return;
        }
        if component.id() == spinel_registry::data_components::vanilla_components::WOLF_COLLAR.id()
            && self.entity_type == EntityType::WOLF
        {
            let Some(color) = DyeColor::from_component_nbt(&value.to_component_nbt()) else {
                return;
            };
            self.set_wolf_collar_color(color);
            return;
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::SHULKER_COLOR.id()
            && self.entity_type == EntityType::SHULKER
        {
            let Some(color) = DyeColor::from_component_nbt(&value.to_component_nbt()) else {
                return;
            };
            self.set_shulker_color(color);
            return;
        }
        if component.id() == spinel_registry::data_components::vanilla_components::FOX_VARIANT.id()
            && self.entity_type == EntityType::FOX
        {
            let Some(variant) = FoxVariant::from_component_nbt(&value.to_component_nbt()) else {
                return;
            };
            self.set_fox_variant(variant);
            return;
        }
        if component.id() == spinel_registry::data_components::vanilla_components::SALMON_SIZE.id()
            && self.entity_type == EntityType::SALMON
        {
            let Some(size) = SalmonSize::from_component_nbt(&value.to_component_nbt()) else {
                return;
            };
            self.set_salmon_size(size);
            return;
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::PARROT_VARIANT.id()
            && self.entity_type == EntityType::PARROT
        {
            let Some(color) = ParrotColor::from_component_nbt(&value.to_component_nbt()) else {
                return;
            };
            self.set_parrot_color(color);
            return;
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::TROPICAL_FISH_PATTERN.id()
            && self.entity_type == EntityType::TROPICAL_FISH
        {
            let Some(pattern) = TropicalFishPattern::from_component_nbt(&value.to_component_nbt())
            else {
                return;
            };
            let variant = self.tropical_fish_variant().with_pattern(pattern);
            self.set_tropical_fish_variant(variant);
            return;
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::TROPICAL_FISH_BASE_COLOR.id()
            && self.entity_type == EntityType::TROPICAL_FISH
        {
            let Some(color) = DyeColor::from_component_nbt(&value.to_component_nbt()) else {
                return;
            };
            let variant = self.tropical_fish_variant().with_base_color(color);
            self.set_tropical_fish_variant(variant);
            return;
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::TROPICAL_FISH_PATTERN_COLOR
                .id()
            && self.entity_type == EntityType::TROPICAL_FISH
        {
            let Some(color) = DyeColor::from_component_nbt(&value.to_component_nbt()) else {
                return;
            };
            let variant = self.tropical_fish_variant().with_pattern_color(color);
            self.set_tropical_fish_variant(variant);
            return;
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::MOOSHROOM_VARIANT.id()
            && self.entity_type == EntityType::MOOSHROOM
        {
            let Some(variant) = MooshroomVariant::from_component_nbt(&value.to_component_nbt())
            else {
                return;
            };
            self.set_mooshroom_variant(variant);
            return;
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::RABBIT_VARIANT.id()
            && self.entity_type == EntityType::RABBIT
        {
            let Some(variant) = RabbitVariant::from_component_nbt(&value.to_component_nbt()) else {
                return;
            };
            self.set_rabbit_variant(variant);
            return;
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::HORSE_VARIANT.id()
            && self.entity_type == EntityType::HORSE
        {
            let Some(color) =
                spinel_registry::HorseColor::from_component_nbt(&value.to_component_nbt())
            else {
                return;
            };
            let variant = self.horse_variant().with_color(color);
            self.set_horse_variant(variant);
            return;
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::LLAMA_VARIANT.id()
            && (self.entity_type == EntityType::LLAMA
                || self.entity_type == EntityType::TRADER_LLAMA)
        {
            let Some(variant) = LlamaVariant::from_component_nbt(&value.to_component_nbt()) else {
                return;
            };
            self.set_llama_variant(variant);
            return;
        }
        if component.id()
            == spinel_registry::data_components::vanilla_components::AXOLOTL_VARIANT.id()
            && self.entity_type == EntityType::AXOLOTL
        {
            let Some(variant) = AxolotlVariant::from_component_nbt(&value.to_component_nbt())
            else {
                return;
            };
            self.set_axolotl_variant(variant);
            return;
        }
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
        if self.world.is_none() {
            self.synchronization.record_position(self.position);
        }
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
            TeleportFlags::absolute().with(TeleportFlags::DELTA_COORD),
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
            flags.with(TeleportFlags::DELTA_COORD),
        )
    }

    pub fn teleport_with_velocity_chunks_and_flags(
        &mut self,
        position: EntityPosition,
        velocity: Velocity,
        chunks: impl Into<Option<Vec<i64>>>,
        flags: TeleportFlags,
    ) -> EntityTeleport {
        let teleport = EntityTeleport::resolve(
            self.position,
            self.velocity,
            position,
            velocity,
            chunks.into(),
            flags,
        );
        self.set_position(teleport.position());
        self.set_velocity(teleport.velocity());
        teleport
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

    pub(crate) fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }

    pub fn has_velocity(&self) -> bool {
        let velocity = self.velocity.0;
        if self.on_ground {
            return velocity.x != 0.0 || velocity.z != 0.0 || velocity.y > 0.0;
        }
        velocity.x != 0.0 || velocity.y != 0.0 || velocity.z != 0.0
    }

    pub const fn vehicle(&self) -> Option<EntityId> {
        self.vehicle
    }

    pub(crate) fn set_vehicle(&mut self, vehicle_id: EntityId) {
        if vehicle_id == self.entity_id {
            return;
        }
        self.vehicle = Some(vehicle_id);
    }

    pub(crate) fn clear_vehicle(&mut self) {
        self.vehicle = None;
    }

    pub(crate) fn add_passenger(&mut self, passenger_id: EntityId) -> bool {
        if passenger_id == self.entity_id {
            return false;
        }
        self.passengers.insert(passenger_id)
    }

    pub(crate) fn remove_passenger(&mut self, passenger_id: EntityId) -> bool {
        self.passengers.remove(&passenger_id)
    }

    pub fn has_passenger(&self) -> bool {
        !self.passengers.is_empty()
    }

    pub fn passengers(&self) -> &BTreeSet<EntityId> {
        &self.passengers
    }

    pub(crate) fn passenger_packet(&self) -> SetPassengersPacket {
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
        self.leash.leashed_entities()
    }

    pub const fn leash_holder(&self) -> Option<EntityId> {
        self.leash.holder()
    }

    pub(crate) fn set_leash_holder(&mut self, leash_holder: Option<EntityId>) {
        self.leash.set_holder(leash_holder);
    }

    pub(crate) fn add_leashed_entity(&mut self, entity_id: EntityId) -> bool {
        self.leash.add_leashed_entity(entity_id)
    }

    pub(crate) fn remove_leashed_entity(&mut self, entity_id: EntityId) -> bool {
        self.leash.remove_leashed_entity(entity_id)
    }

    pub(crate) fn attach_entity_packet(
        &self,
    ) -> spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket {
        self.leash.packet(self.entity_id)
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

    pub fn is_sneaking(&self) -> bool {
        self.metadata.flag(&definitions::is_crouching())
    }

    pub fn set_sneaking(&mut self, sneaking: bool) {
        self.metadata
            .set_flag(&definitions::is_crouching(), sneaking);
    }

    pub fn is_sprinting(&self) -> bool {
        self.metadata.flag(&definitions::is_sprinting())
    }

    pub fn set_sprinting(&mut self, sprinting: bool) {
        self.metadata
            .set_flag(&definitions::is_sprinting(), sprinting);
    }

    pub fn is_swimming(&self) -> bool {
        self.metadata.flag(&definitions::is_swimming())
    }

    pub fn set_swimming(&mut self, swimming: bool) {
        self.metadata
            .set_flag(&definitions::is_swimming(), swimming);
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

    pub fn air_ticks(&self) -> i32 {
        match self.metadata.value(&definitions::air_ticks()) {
            MetadataValue::VarInt(air_ticks) => air_ticks,
            _ => 300,
        }
    }

    pub fn set_air_ticks(&mut self, air_ticks: i32) {
        self.metadata
            .set(&definitions::air_ticks(), MetadataValue::VarInt(air_ticks));
    }

    pub fn is_hand_active(&self) -> bool {
        self.metadata
            .flag(&definitions::living_entity::is_hand_active())
    }

    pub fn set_hand_active(&mut self, hand_active: bool) {
        self.metadata
            .set_flag(&definitions::living_entity::is_hand_active(), hand_active);
    }

    pub fn active_hand(&self) -> PlayerHand {
        if self
            .metadata
            .flag(&definitions::living_entity::active_hand())
        {
            return PlayerHand::Off;
        }
        PlayerHand::Main
    }

    pub fn set_active_hand(&mut self, hand: PlayerHand) {
        self.metadata.set_flag(
            &definitions::living_entity::active_hand(),
            hand == PlayerHand::Off,
        );
    }

    pub fn is_in_riptide_spin_attack(&self) -> bool {
        self.metadata
            .flag(&definitions::living_entity::is_riptide_spin_attack())
    }

    pub fn set_in_riptide_spin_attack(&mut self, in_riptide_spin_attack: bool) {
        self.metadata.set_flag(
            &definitions::living_entity::is_riptide_spin_attack(),
            in_riptide_spin_attack,
        );
    }

    pub fn effect_particles(&self) -> Vec<Particle> {
        match self
            .metadata
            .value(&definitions::living_entity::potion_effect_particles())
        {
            MetadataValue::ParticleList(effect_particles) => effect_particles,
            _ => Vec::new(),
        }
    }

    pub fn set_effect_particles(&mut self, effect_particles: Vec<Particle>) {
        self.metadata.set(
            &definitions::living_entity::potion_effect_particles(),
            MetadataValue::ParticleList(effect_particles),
        );
    }

    pub fn is_potion_effect_ambient(&self) -> bool {
        match self
            .metadata
            .value(&definitions::living_entity::is_potion_effect_ambient())
        {
            MetadataValue::Boolean(potion_effect_ambient) => potion_effect_ambient,
            _ => false,
        }
    }

    pub fn set_potion_effect_ambient(&mut self, potion_effect_ambient: bool) {
        self.metadata.set(
            &definitions::living_entity::is_potion_effect_ambient(),
            MetadataValue::Boolean(potion_effect_ambient),
        );
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

    pub fn ticks_frozen(&self) -> i32 {
        match self.metadata.value(&definitions::ticks_frozen()) {
            MetadataValue::VarInt(ticks_frozen) => ticks_frozen,
            _ => 0,
        }
    }

    pub fn set_ticks_frozen(&mut self, ticks_frozen: i32) {
        self.metadata.set(
            &definitions::ticks_frozen(),
            MetadataValue::VarInt(ticks_frozen),
        );
    }

    pub fn bee_stinger_count(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::living_entity::number_of_bee_stingers())
        {
            MetadataValue::VarInt(bee_stinger_count) => bee_stinger_count,
            _ => 0,
        }
    }

    pub fn set_bee_stinger_count(&mut self, bee_stinger_count: i32) {
        self.metadata.set(
            &definitions::living_entity::number_of_bee_stingers(),
            MetadataValue::VarInt(bee_stinger_count.max(0)),
        );
    }

    pub fn bed_in_which_sleeping_position(&self) -> Option<Position> {
        match self
            .metadata
            .value(&definitions::living_entity::location_of_bed())
        {
            MetadataValue::OptionalPosition(bed_position) => bed_position,
            _ => None,
        }
    }

    pub fn set_bed_in_which_sleeping_position(&mut self, bed_position: Option<Position>) {
        self.metadata.set(
            &definitions::living_entity::location_of_bed(),
            MetadataValue::OptionalPosition(bed_position),
        );
    }

    pub fn is_no_ai(&self) -> bool {
        self.metadata.flag(&definitions::mob::no_ai())
    }

    pub fn set_no_ai(&mut self, no_ai: bool) {
        self.metadata.set_flag(&definitions::mob::no_ai(), no_ai);
    }

    pub fn is_left_handed(&self) -> bool {
        self.metadata.flag(&definitions::mob::is_left_handed())
    }

    pub fn set_left_handed(&mut self, left_handed: bool) {
        self.metadata
            .set_flag(&definitions::mob::is_left_handed(), left_handed);
    }

    pub fn is_aggressive(&self) -> bool {
        self.metadata.flag(&definitions::mob::is_aggressive())
    }

    pub fn set_aggressive(&mut self, aggressive: bool) {
        self.metadata
            .set_flag(&definitions::mob::is_aggressive(), aggressive);
    }

    pub fn is_baby(&self) -> bool {
        match self.metadata.value(&definitions::ageable_mob::is_baby()) {
            MetadataValue::Boolean(baby) => baby,
            _ => false,
        }
    }

    pub fn set_baby(&mut self, baby: bool) {
        if self.is_baby() == baby {
            return;
        }
        let bounding_box = self.bounding_box();
        let width = if baby {
            bounding_box.width() / 2.0
        } else {
            bounding_box.width() * 2.0
        };
        let height = if baby {
            bounding_box.height() / 2.0
        } else {
            bounding_box.height() * 2.0
        };
        self.set_bounding_box_dimensions(width, height, width);
        self.metadata.set(
            &definitions::ageable_mob::is_baby(),
            MetadataValue::Boolean(baby),
        );
    }

    pub fn is_hanging_bat(&self) -> bool {
        self.metadata.flag(&definitions::bat::is_hanging())
    }

    pub fn set_hanging_bat(&mut self, is_hanging: bool) {
        self.metadata
            .set_flag(&definitions::bat::is_hanging(), is_hanging);
    }

    pub fn is_dancing_allay(&self) -> bool {
        match self.metadata.value(&definitions::allay::is_dancing()) {
            MetadataValue::Boolean(is_dancing) => is_dancing,
            _ => false,
        }
    }

    pub fn set_dancing_allay(&mut self, is_dancing: bool) {
        self.metadata.set(
            &definitions::allay::is_dancing(),
            MetadataValue::Boolean(is_dancing),
        );
    }

    pub fn allay_can_duplicate(&self) -> bool {
        match self.metadata.value(&definitions::allay::can_duplicate()) {
            MetadataValue::Boolean(can_duplicate) => can_duplicate,
            _ => true,
        }
    }

    pub fn set_allay_can_duplicate(&mut self, can_duplicate: bool) {
        self.metadata.set(
            &definitions::allay::can_duplicate(),
            MetadataValue::Boolean(can_duplicate),
        );
    }

    pub fn sniffer_state(&self) -> SnifferState {
        match self.metadata.value(&definitions::sniffer::state()) {
            MetadataValue::SnifferState(state) => {
                SnifferState::from_protocol_id(state).unwrap_or_default()
            }
            _ => SnifferState::default(),
        }
    }

    pub fn set_sniffer_state(&mut self, state: SnifferState) {
        self.metadata.set(
            &definitions::sniffer::state(),
            MetadataValue::SnifferState(state.protocol_id()),
        );
    }

    pub fn sniffer_drop_seed_at_tick(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::sniffer::drop_seed_at_tick())
        {
            MetadataValue::VarInt(tick) => tick,
            _ => 0,
        }
    }

    pub fn set_sniffer_drop_seed_at_tick(&mut self, tick: i32) {
        self.metadata.set(
            &definitions::sniffer::drop_seed_at_tick(),
            MetadataValue::VarInt(tick),
        );
    }

    pub fn dolphin_treasure_position(&self) -> Position {
        match self
            .metadata
            .value(&definitions::dolphin::treasure_position())
        {
            MetadataValue::Position(position) => position,
            _ => Position { x: 0, y: 0, z: 0 },
        }
    }

    pub fn set_dolphin_treasure_position(&mut self, position: Position) {
        self.metadata.set(
            &definitions::dolphin::treasure_position(),
            MetadataValue::Position(position),
        );
    }

    pub fn dolphin_has_fish(&self) -> bool {
        match self.metadata.value(&definitions::dolphin::has_fish()) {
            MetadataValue::Boolean(has_fish) => has_fish,
            _ => false,
        }
    }

    pub fn set_dolphin_has_fish(&mut self, has_fish: bool) {
        self.metadata.set(
            &definitions::dolphin::has_fish(),
            MetadataValue::Boolean(has_fish),
        );
    }

    pub fn dolphin_moisture_level(&self) -> i32 {
        match self.metadata.value(&definitions::dolphin::moisture_level()) {
            MetadataValue::VarInt(moisture_level) => moisture_level,
            _ => 2400,
        }
    }

    pub fn set_dolphin_moisture_level(&mut self, moisture_level: i32) {
        self.metadata.set(
            &definitions::dolphin::moisture_level(),
            MetadataValue::VarInt(moisture_level),
        );
    }

    pub fn axolotl_variant(&self) -> AxolotlVariant {
        match self.metadata.value(&definitions::axolotl::variant()) {
            MetadataValue::VarInt(variant_id) => {
                AxolotlVariant::from_protocol_id(variant_id).unwrap_or_default()
            }
            _ => AxolotlVariant::default(),
        }
    }

    pub fn set_axolotl_variant(&mut self, variant: AxolotlVariant) {
        self.metadata.set(
            &definitions::axolotl::variant(),
            MetadataValue::VarInt(variant.protocol_id()),
        );
    }

    pub fn is_playing_dead_axolotl(&self) -> bool {
        match self
            .metadata
            .value(&definitions::axolotl::is_playing_dead())
        {
            MetadataValue::Boolean(is_playing_dead) => is_playing_dead,
            _ => false,
        }
    }

    pub fn set_playing_dead_axolotl(&mut self, is_playing_dead: bool) {
        self.metadata.set(
            &definitions::axolotl::is_playing_dead(),
            MetadataValue::Boolean(is_playing_dead),
        );
    }

    pub fn is_from_bucket_axolotl(&self) -> bool {
        match self.metadata.value(&definitions::axolotl::is_from_bucket()) {
            MetadataValue::Boolean(is_from_bucket) => is_from_bucket,
            _ => false,
        }
    }

    pub fn set_from_bucket_axolotl(&mut self, is_from_bucket: bool) {
        self.metadata.set(
            &definitions::axolotl::is_from_bucket(),
            MetadataValue::Boolean(is_from_bucket),
        );
    }

    pub fn is_fish_from_bucket(&self) -> bool {
        match self
            .metadata
            .value(&definitions::abstract_fish::from_bucket())
        {
            MetadataValue::Boolean(is_from_bucket) => is_from_bucket,
            _ => false,
        }
    }

    pub fn set_fish_from_bucket(&mut self, is_from_bucket: bool) {
        self.metadata.set(
            &definitions::abstract_fish::from_bucket(),
            MetadataValue::Boolean(is_from_bucket),
        );
    }

    pub fn pufferfish_state(&self) -> PufferfishState {
        match self.metadata.value(&definitions::puffer_fish::puff_state()) {
            MetadataValue::VarInt(state) => {
                PufferfishState::from_protocol_id(state).unwrap_or_default()
            }
            _ => PufferfishState::default(),
        }
    }

    pub fn set_pufferfish_state(&mut self, state: PufferfishState) {
        self.metadata.set(
            &definitions::puffer_fish::puff_state(),
            MetadataValue::VarInt(state.protocol_id()),
        );
        let size = state.bounding_box_size();
        self.set_bounding_box_dimensions(size, size, size);
    }

    pub fn salmon_size(&self) -> SalmonSize {
        match self.metadata.value(&definitions::salmon::size()) {
            MetadataValue::VarInt(size) => SalmonSize::from_protocol_id(size).unwrap_or_default(),
            _ => SalmonSize::default(),
        }
    }

    pub fn set_salmon_size(&mut self, size: SalmonSize) {
        self.metadata.set(
            &definitions::salmon::size(),
            MetadataValue::VarInt(size.protocol_id()),
        );
    }

    pub fn tropical_fish_variant(&self) -> TropicalFishVariant {
        match self.metadata.value(&definitions::tropical_fish::variant()) {
            MetadataValue::VarInt(variant) => {
                TropicalFishVariant::from_packed_id(variant).unwrap_or_default()
            }
            _ => TropicalFishVariant::default(),
        }
    }

    pub fn set_tropical_fish_variant(&mut self, variant: TropicalFishVariant) {
        self.metadata.set(
            &definitions::tropical_fish::variant(),
            MetadataValue::VarInt(variant.packed_id()),
        );
    }

    pub fn is_screaming_goat(&self) -> bool {
        match self.metadata.value(&definitions::goat::is_screaming()) {
            MetadataValue::Boolean(is_screaming) => is_screaming,
            _ => false,
        }
    }

    pub fn set_screaming_goat(&mut self, is_screaming: bool) {
        self.metadata.set(
            &definitions::goat::is_screaming(),
            MetadataValue::Boolean(is_screaming),
        );
    }

    pub fn goat_has_left_horn(&self) -> bool {
        match self.metadata.value(&definitions::goat::has_left_horn()) {
            MetadataValue::Boolean(has_left_horn) => has_left_horn,
            _ => true,
        }
    }

    pub fn set_goat_has_left_horn(&mut self, has_left_horn: bool) {
        self.metadata.set(
            &definitions::goat::has_left_horn(),
            MetadataValue::Boolean(has_left_horn),
        );
    }

    pub fn goat_has_right_horn(&self) -> bool {
        match self.metadata.value(&definitions::goat::has_right_horn()) {
            MetadataValue::Boolean(has_right_horn) => has_right_horn,
            _ => true,
        }
    }

    pub fn set_goat_has_right_horn(&mut self, has_right_horn: bool) {
        self.metadata.set(
            &definitions::goat::has_right_horn(),
            MetadataValue::Boolean(has_right_horn),
        );
    }

    pub fn pig_boost_time(&self) -> i32 {
        match self.metadata.value(&definitions::pig::boost_time()) {
            MetadataValue::VarInt(boost_time) => boost_time,
            _ => 0,
        }
    }

    pub fn set_pig_boost_time(&mut self, boost_time: i32) {
        self.metadata.set(
            &definitions::pig::boost_time(),
            MetadataValue::VarInt(boost_time),
        );
    }

    pub fn is_tamed_horse(&self) -> bool {
        self.metadata.flag(&definitions::abstract_horse::is_tame())
    }

    pub fn set_tamed_horse(&mut self, is_tamed: bool) {
        self.metadata
            .set_flag(&definitions::abstract_horse::is_tame(), is_tamed);
    }

    pub fn horse_has_bred(&self) -> bool {
        self.metadata.flag(&definitions::abstract_horse::has_bred())
    }

    pub fn set_horse_has_bred(&mut self, has_bred: bool) {
        self.metadata
            .set_flag(&definitions::abstract_horse::has_bred(), has_bred);
    }

    pub fn is_eating_horse(&self) -> bool {
        self.metadata
            .flag(&definitions::abstract_horse::is_eating())
    }

    pub fn set_eating_horse(&mut self, is_eating: bool) {
        self.metadata
            .set_flag(&definitions::abstract_horse::is_eating(), is_eating);
    }

    pub fn is_rearing_horse(&self) -> bool {
        self.metadata
            .flag(&definitions::abstract_horse::is_rearing())
    }

    pub fn set_rearing_horse(&mut self, is_rearing: bool) {
        self.metadata
            .set_flag(&definitions::abstract_horse::is_rearing(), is_rearing);
    }

    pub fn is_horse_mouth_open(&self) -> bool {
        self.metadata
            .flag(&definitions::abstract_horse::is_mouth_open())
    }

    pub fn set_horse_mouth_open(&mut self, is_mouth_open: bool) {
        self.metadata
            .set_flag(&definitions::abstract_horse::is_mouth_open(), is_mouth_open);
    }

    pub fn horse_variant(&self) -> HorseVariant {
        match self.metadata.value(&definitions::horse::variant()) {
            MetadataValue::VarInt(variant_id) => {
                HorseVariant::from_protocol_id(variant_id).unwrap_or_default()
            }
            _ => HorseVariant::default(),
        }
    }

    pub fn set_horse_variant(&mut self, variant: HorseVariant) {
        self.metadata.set(
            &definitions::horse::variant(),
            MetadataValue::VarInt(variant.protocol_id()),
        );
    }

    pub fn is_dashing_camel(&self) -> bool {
        match self.metadata.value(&definitions::camel::is_dashing()) {
            MetadataValue::Boolean(is_dashing) => is_dashing,
            _ => false,
        }
    }

    pub fn set_dashing_camel(&mut self, is_dashing: bool) {
        self.metadata.set(
            &definitions::camel::is_dashing(),
            MetadataValue::Boolean(is_dashing),
        );
    }

    pub fn camel_last_pose_change_tick(&self) -> i64 {
        match self
            .metadata
            .value(&definitions::camel::last_pose_change_tick())
        {
            MetadataValue::Long(tick) => tick,
            _ => 0,
        }
    }

    pub fn set_camel_last_pose_change_tick(&mut self, tick: i64) {
        self.metadata.set(
            &definitions::camel::last_pose_change_tick(),
            MetadataValue::Long(tick),
        );
    }

    pub fn chested_horse_has_chest(&self) -> bool {
        match self
            .metadata
            .value(&definitions::chested_horse::has_chest())
        {
            MetadataValue::Boolean(has_chest) => has_chest,
            _ => false,
        }
    }

    pub fn set_chested_horse_has_chest(&mut self, has_chest: bool) {
        self.metadata.set(
            &definitions::chested_horse::has_chest(),
            MetadataValue::Boolean(has_chest),
        );
    }

    pub fn llama_strength(&self) -> i32 {
        match self.metadata.value(&definitions::llama::strength()) {
            MetadataValue::VarInt(strength) => strength,
            _ => 0,
        }
    }

    pub fn set_llama_strength(&mut self, strength: i32) {
        self.metadata.set(
            &definitions::llama::strength(),
            MetadataValue::VarInt(strength),
        );
    }

    pub fn llama_carpet_color(&self) -> i32 {
        match self.metadata.value(&definitions::llama::carpet_color()) {
            MetadataValue::VarInt(color) => color,
            _ => -1,
        }
    }

    pub fn set_llama_carpet_color(&mut self, color: i32) {
        self.metadata.set(
            &definitions::llama::carpet_color(),
            MetadataValue::VarInt(color),
        );
    }

    pub fn llama_variant(&self) -> LlamaVariant {
        match self.metadata.value(&definitions::llama::variant()) {
            MetadataValue::VarInt(variant_id) => {
                LlamaVariant::from_protocol_id(variant_id).unwrap_or_default()
            }
            _ => LlamaVariant::default(),
        }
    }

    pub fn set_llama_variant(&mut self, variant: LlamaVariant) {
        self.metadata.set(
            &definitions::llama::variant(),
            MetadataValue::VarInt(variant.protocol_id()),
        );
    }

    pub fn fox_variant(&self) -> FoxVariant {
        match self.metadata.value(&definitions::fox::variant()) {
            MetadataValue::VarInt(variant_id) => {
                FoxVariant::from_protocol_id(variant_id).unwrap_or_default()
            }
            _ => FoxVariant::default(),
        }
    }

    pub fn set_fox_variant(&mut self, variant: FoxVariant) {
        self.metadata.set(
            &definitions::fox::variant(),
            MetadataValue::VarInt(variant.protocol_id()),
        );
    }

    pub fn is_sitting_fox(&self) -> bool {
        self.metadata.flag(&definitions::fox::is_sitting())
    }

    pub fn set_sitting_fox(&mut self, is_sitting: bool) {
        self.metadata
            .set_flag(&definitions::fox::is_sitting(), is_sitting);
    }

    pub fn is_sneaking_fox(&self) -> bool {
        self.metadata.flag(&definitions::fox::is_crouching())
    }

    pub fn set_sneaking_fox(&mut self, is_sneaking: bool) {
        self.metadata
            .set_flag(&definitions::fox::is_crouching(), is_sneaking);
    }

    pub fn is_interested_fox(&self) -> bool {
        self.metadata.flag(&definitions::fox::is_interested())
    }

    pub fn set_interested_fox(&mut self, is_interested: bool) {
        self.metadata
            .set_flag(&definitions::fox::is_interested(), is_interested);
    }

    pub fn is_pouncing_fox(&self) -> bool {
        self.metadata.flag(&definitions::fox::is_pouncing())
    }

    pub fn set_pouncing_fox(&mut self, is_pouncing: bool) {
        self.metadata
            .set_flag(&definitions::fox::is_pouncing(), is_pouncing);
    }

    pub fn is_sleeping_fox(&self) -> bool {
        self.metadata.flag(&definitions::fox::is_sleeping())
    }

    pub fn set_sleeping_fox(&mut self, is_sleeping: bool) {
        self.metadata
            .set_flag(&definitions::fox::is_sleeping(), is_sleeping);
    }

    pub fn is_faceplanted_fox(&self) -> bool {
        self.metadata.flag(&definitions::fox::is_faceplanted())
    }

    pub fn set_faceplanted_fox(&mut self, is_faceplanted: bool) {
        self.metadata
            .set_flag(&definitions::fox::is_faceplanted(), is_faceplanted);
    }

    pub fn is_defending_fox(&self) -> bool {
        self.metadata.flag(&definitions::fox::is_defending())
    }

    pub fn set_defending_fox(&mut self, is_defending: bool) {
        self.metadata
            .set_flag(&definitions::fox::is_defending(), is_defending);
    }

    pub fn fox_first_uuid(&self) -> Option<Uuid> {
        match self.metadata.value(&definitions::fox::first_uuid()) {
            MetadataValue::OptionalLivingEntityReference(uuid) => uuid,
            _ => None,
        }
    }

    pub fn set_fox_first_uuid(&mut self, uuid: Option<Uuid>) {
        self.metadata.set(
            &definitions::fox::first_uuid(),
            MetadataValue::OptionalLivingEntityReference(uuid),
        );
    }

    pub fn fox_second_uuid(&self) -> Option<Uuid> {
        match self.metadata.value(&definitions::fox::second_uuid()) {
            MetadataValue::OptionalLivingEntityReference(uuid) => uuid,
            _ => None,
        }
    }

    pub fn set_fox_second_uuid(&mut self, uuid: Option<Uuid>) {
        self.metadata.set(
            &definitions::fox::second_uuid(),
            MetadataValue::OptionalLivingEntityReference(uuid),
        );
    }

    pub fn is_trusting_ocelot(&self) -> bool {
        match self.metadata.value(&definitions::ocelot::is_trusting()) {
            MetadataValue::Boolean(is_trusting) => is_trusting,
            _ => false,
        }
    }

    pub fn set_trusting_ocelot(&mut self, is_trusting: bool) {
        self.metadata.set(
            &definitions::ocelot::is_trusting(),
            MetadataValue::Boolean(is_trusting),
        );
    }

    pub fn panda_breed_timer(&self) -> i32 {
        match self.metadata.value(&definitions::panda::breed_timer()) {
            MetadataValue::VarInt(timer) => timer,
            _ => 0,
        }
    }

    pub fn set_panda_breed_timer(&mut self, timer: i32) {
        self.metadata.set(
            &definitions::panda::breed_timer(),
            MetadataValue::VarInt(timer),
        );
    }

    pub fn panda_sneeze_timer(&self) -> i32 {
        match self.metadata.value(&definitions::panda::sneeze_timer()) {
            MetadataValue::VarInt(timer) => timer,
            _ => 0,
        }
    }

    pub fn set_panda_sneeze_timer(&mut self, timer: i32) {
        self.metadata.set(
            &definitions::panda::sneeze_timer(),
            MetadataValue::VarInt(timer),
        );
    }

    pub fn panda_eat_timer(&self) -> i32 {
        match self.metadata.value(&definitions::panda::eat_timer()) {
            MetadataValue::VarInt(timer) => timer,
            _ => 0,
        }
    }

    pub fn set_panda_eat_timer(&mut self, timer: i32) {
        self.metadata.set(
            &definitions::panda::eat_timer(),
            MetadataValue::VarInt(timer),
        );
    }

    pub fn panda_main_gene(&self) -> PandaGene {
        match self.metadata.value(&definitions::panda::main_gene()) {
            MetadataValue::Byte(gene_id) => {
                PandaGene::from_protocol_id(gene_id as i32).unwrap_or_default()
            }
            _ => PandaGene::default(),
        }
    }

    pub fn set_panda_main_gene(&mut self, gene: PandaGene) {
        self.metadata.set(
            &definitions::panda::main_gene(),
            MetadataValue::Byte(gene.protocol_id() as i8),
        );
    }

    pub fn panda_hidden_gene(&self) -> PandaGene {
        match self.metadata.value(&definitions::panda::hidden_gene()) {
            MetadataValue::Byte(gene_id) => {
                PandaGene::from_protocol_id(gene_id as i32).unwrap_or_default()
            }
            _ => PandaGene::default(),
        }
    }

    pub fn set_panda_hidden_gene(&mut self, gene: PandaGene) {
        self.metadata.set(
            &definitions::panda::hidden_gene(),
            MetadataValue::Byte(gene.protocol_id() as i8),
        );
    }

    pub fn is_sneezing_panda(&self) -> bool {
        self.metadata.flag(&definitions::panda::is_sneezing())
    }

    pub fn set_sneezing_panda(&mut self, is_sneezing: bool) {
        self.metadata
            .set_flag(&definitions::panda::is_sneezing(), is_sneezing);
    }

    pub fn is_rolling_panda(&self) -> bool {
        self.metadata.flag(&definitions::panda::is_rolling())
    }

    pub fn set_rolling_panda(&mut self, is_rolling: bool) {
        self.metadata
            .set_flag(&definitions::panda::is_rolling(), is_rolling);
    }

    pub fn is_sitting_panda(&self) -> bool {
        self.metadata.flag(&definitions::panda::is_sitting())
    }

    pub fn set_sitting_panda(&mut self, is_sitting: bool) {
        self.metadata
            .set_flag(&definitions::panda::is_sitting(), is_sitting);
    }

    pub fn is_on_back_panda(&self) -> bool {
        self.metadata.flag(&definitions::panda::is_on_back())
    }

    pub fn set_on_back_panda(&mut self, is_on_back: bool) {
        self.metadata
            .set_flag(&definitions::panda::is_on_back(), is_on_back);
    }

    pub fn rabbit_variant(&self) -> RabbitVariant {
        match self.metadata.value(&definitions::rabbit::kind()) {
            MetadataValue::VarInt(variant_id) => {
                RabbitVariant::from_protocol_id(variant_id).unwrap_or_default()
            }
            _ => RabbitVariant::default(),
        }
    }

    pub fn set_rabbit_variant(&mut self, variant: RabbitVariant) {
        self.metadata.set(
            &definitions::rabbit::kind(),
            MetadataValue::VarInt(variant.protocol_id()),
        );
    }

    pub fn mooshroom_variant(&self) -> MooshroomVariant {
        match self.metadata.value(&definitions::mooshroom::variant()) {
            MetadataValue::VarInt(variant_id) => {
                MooshroomVariant::from_protocol_id(variant_id).unwrap_or_default()
            }
            _ => MooshroomVariant::default(),
        }
    }

    pub fn set_mooshroom_variant(&mut self, variant: MooshroomVariant) {
        self.metadata.set(
            &definitions::mooshroom::variant(),
            MetadataValue::VarInt(variant.protocol_id()),
        );
    }

    pub fn parrot_color(&self) -> ParrotColor {
        match self.metadata.value(&definitions::parrot::variant()) {
            MetadataValue::VarInt(color_id) => {
                ParrotColor::from_protocol_id(color_id).unwrap_or_default()
            }
            _ => ParrotColor::default(),
        }
    }

    pub fn set_parrot_color(&mut self, color: ParrotColor) {
        self.metadata.set(
            &definitions::parrot::variant(),
            MetadataValue::VarInt(color.protocol_id()),
        );
    }

    pub fn turtle_has_egg(&self) -> bool {
        match self.metadata.value(&definitions::turtle::has_egg()) {
            MetadataValue::Boolean(has_egg) => has_egg,
            _ => false,
        }
    }

    pub fn set_turtle_has_egg(&mut self, has_egg: bool) {
        self.metadata.set(
            &definitions::turtle::has_egg(),
            MetadataValue::Boolean(has_egg),
        );
    }

    pub fn is_laying_egg_turtle(&self) -> bool {
        match self.metadata.value(&definitions::turtle::is_laying_egg()) {
            MetadataValue::Boolean(is_laying_egg) => is_laying_egg,
            _ => false,
        }
    }

    pub fn set_laying_egg_turtle(&mut self, is_laying_egg: bool) {
        self.metadata.set(
            &definitions::turtle::is_laying_egg(),
            MetadataValue::Boolean(is_laying_egg),
        );
    }

    pub fn is_standing_polar_bear(&self) -> bool {
        match self
            .metadata
            .value(&definitions::polar_bear::is_standing_up())
        {
            MetadataValue::Boolean(is_standing) => is_standing,
            _ => false,
        }
    }

    pub fn set_standing_polar_bear(&mut self, is_standing: bool) {
        self.metadata.set(
            &definitions::polar_bear::is_standing_up(),
            MetadataValue::Boolean(is_standing),
        );
    }

    pub fn hoglin_is_immune_to_zombification(&self) -> bool {
        match self
            .metadata
            .value(&definitions::hoglin::is_immune_to_zombification())
        {
            MetadataValue::Boolean(is_immune) => is_immune,
            _ => false,
        }
    }

    pub fn set_hoglin_immune_to_zombification(&mut self, is_immune: bool) {
        self.metadata.set(
            &definitions::hoglin::is_immune_to_zombification(),
            MetadataValue::Boolean(is_immune),
        );
    }

    pub fn strider_boost_time(&self) -> i32 {
        match self.metadata.value(&definitions::strider::fungus_boost()) {
            MetadataValue::VarInt(boost_time) => boost_time,
            _ => 0,
        }
    }

    pub fn set_strider_boost_time(&mut self, boost_time: i32) {
        self.metadata.set(
            &definitions::strider::fungus_boost(),
            MetadataValue::VarInt(boost_time),
        );
    }

    pub fn is_shaking_strider(&self) -> bool {
        match self.metadata.value(&definitions::strider::is_shaking()) {
            MetadataValue::Boolean(is_shaking) => is_shaking,
            _ => false,
        }
    }

    pub fn set_shaking_strider(&mut self, is_shaking: bool) {
        self.metadata.set(
            &definitions::strider::is_shaking(),
            MetadataValue::Boolean(is_shaking),
        );
    }

    pub fn is_sitting_tameable_animal(&self) -> bool {
        self.metadata
            .flag(&definitions::tameable_animal::is_sitting())
    }

    pub fn set_sitting_tameable_animal(&mut self, is_sitting: bool) {
        self.metadata
            .set_flag(&definitions::tameable_animal::is_sitting(), is_sitting);
    }

    pub fn is_tamed_animal(&self) -> bool {
        self.metadata
            .flag(&definitions::tameable_animal::is_tamed())
    }

    pub fn set_tamed_animal(&mut self, is_tamed: bool) {
        self.metadata
            .set_flag(&definitions::tameable_animal::is_tamed(), is_tamed);
    }

    pub fn tameable_animal_owner(&self) -> Option<Uuid> {
        match self.metadata.value(&definitions::tameable_animal::owner()) {
            MetadataValue::OptionalLivingEntityReference(owner) => owner,
            _ => None,
        }
    }

    pub fn set_tameable_animal_owner(&mut self, owner: Option<Uuid>) {
        self.metadata.set(
            &definitions::tameable_animal::owner(),
            MetadataValue::OptionalLivingEntityReference(owner),
        );
    }

    pub fn is_dashing_nautilus(&self) -> bool {
        match self
            .metadata
            .value(&definitions::abstract_nautilus::is_dashing())
        {
            MetadataValue::Boolean(is_dashing) => is_dashing,
            _ => false,
        }
    }

    pub fn set_dashing_nautilus(&mut self, is_dashing: bool) {
        self.metadata.set(
            &definitions::abstract_nautilus::is_dashing(),
            MetadataValue::Boolean(is_dashing),
        );
    }

    pub fn happy_ghast_is_leash_holder(&self) -> bool {
        match self
            .metadata
            .value(&definitions::happy_ghast::is_leash_holder())
        {
            MetadataValue::Boolean(is_leash_holder) => is_leash_holder,
            _ => false,
        }
    }

    pub fn set_happy_ghast_leash_holder(&mut self, is_leash_holder: bool) {
        self.metadata.set(
            &definitions::happy_ghast::is_leash_holder(),
            MetadataValue::Boolean(is_leash_holder),
        );
    }

    pub fn happy_ghast_stays_still(&self) -> bool {
        match self
            .metadata
            .value(&definitions::happy_ghast::stays_still())
        {
            MetadataValue::Boolean(stays_still) => stays_still,
            _ => false,
        }
    }

    pub fn set_happy_ghast_stays_still(&mut self, stays_still: bool) {
        self.metadata.set(
            &definitions::happy_ghast::stays_still(),
            MetadataValue::Boolean(stays_still),
        );
    }

    pub fn is_lying_cat(&self) -> bool {
        match self.metadata.value(&definitions::cat::is_lying()) {
            MetadataValue::Boolean(is_lying) => is_lying,
            _ => false,
        }
    }

    pub fn set_lying_cat(&mut self, is_lying: bool) {
        self.metadata.set(
            &definitions::cat::is_lying(),
            MetadataValue::Boolean(is_lying),
        );
    }

    pub fn is_relaxed_cat(&self) -> bool {
        match self.metadata.value(&definitions::cat::is_relaxed()) {
            MetadataValue::Boolean(is_relaxed) => is_relaxed,
            _ => false,
        }
    }

    pub fn set_relaxed_cat(&mut self, is_relaxed: bool) {
        self.metadata.set(
            &definitions::cat::is_relaxed(),
            MetadataValue::Boolean(is_relaxed),
        );
    }

    pub fn cat_collar_color(&self) -> DyeColor {
        match self.metadata.value(&definitions::cat::collar_color()) {
            MetadataValue::VarInt(color_id) => DyeColor::ALL
                .get(color_id as usize)
                .copied()
                .unwrap_or(DyeColor::Red),
            _ => DyeColor::Red,
        }
    }

    pub fn set_cat_collar_color(&mut self, color: DyeColor) {
        let color_id = DyeColor::ALL
            .iter()
            .position(|candidate| candidate == &color)
            .unwrap_or(14) as i32;
        self.metadata.set(
            &definitions::cat::collar_color(),
            MetadataValue::VarInt(color_id),
        );
    }

    pub fn is_begging_wolf(&self) -> bool {
        match self.metadata.value(&definitions::wolf::is_begging()) {
            MetadataValue::Boolean(is_begging) => is_begging,
            _ => false,
        }
    }

    pub fn set_begging_wolf(&mut self, is_begging: bool) {
        self.metadata.set(
            &definitions::wolf::is_begging(),
            MetadataValue::Boolean(is_begging),
        );
    }

    pub fn wolf_collar_color(&self) -> DyeColor {
        match self.metadata.value(&definitions::wolf::collar_color()) {
            MetadataValue::VarInt(color_id) => DyeColor::ALL
                .get(color_id as usize)
                .copied()
                .unwrap_or(DyeColor::Red),
            _ => DyeColor::Red,
        }
    }

    pub fn set_wolf_collar_color(&mut self, color: DyeColor) {
        let color_id = DyeColor::ALL
            .iter()
            .position(|candidate| candidate == &color)
            .unwrap_or(14) as i32;
        self.metadata.set(
            &definitions::wolf::collar_color(),
            MetadataValue::VarInt(color_id),
        );
    }

    pub fn wolf_anger_time(&self) -> i64 {
        match self.metadata.value(&definitions::wolf::anger_time()) {
            MetadataValue::Long(anger_time) => anger_time,
            _ => -1,
        }
    }

    pub fn set_wolf_anger_time(&mut self, anger_time: i64) {
        self.metadata.set(
            &definitions::wolf::anger_time(),
            MetadataValue::Long(anger_time),
        );
    }

    pub fn sheep_color(&self) -> DyeColor {
        let color_id = self.metadata.byte(&definitions::sheep::color_id());
        DyeColor::ALL
            .get(color_id as usize)
            .copied()
            .unwrap_or(DyeColor::White)
    }

    pub fn set_sheep_color(&mut self, color: DyeColor) {
        let color_id = DyeColor::ALL
            .iter()
            .position(|candidate| candidate == &color)
            .unwrap_or(0) as i8;
        self.metadata
            .set_byte(&definitions::sheep::color_id(), color_id);
    }

    pub fn is_sheared_sheep(&self) -> bool {
        self.metadata.flag(&definitions::sheep::is_sheared())
    }

    pub fn set_sheared_sheep(&mut self, is_sheared: bool) {
        self.metadata
            .set_flag(&definitions::sheep::is_sheared(), is_sheared);
    }

    pub fn is_blaze_on_fire(&self) -> bool {
        self.metadata.flag(&definitions::blaze::is_on_fire())
    }

    pub fn set_blaze_on_fire(&mut self, is_on_fire: bool) {
        self.metadata
            .set_flag(&definitions::blaze::is_on_fire(), is_on_fire);
    }

    pub fn is_sheared_bogged(&self) -> bool {
        match self.metadata.value(&definitions::bogged::is_sheared()) {
            MetadataValue::Boolean(is_sheared) => is_sheared,
            _ => false,
        }
    }

    pub fn set_sheared_bogged(&mut self, is_sheared: bool) {
        self.metadata.set(
            &definitions::bogged::is_sheared(),
            MetadataValue::Boolean(is_sheared),
        );
    }

    pub fn piglin_is_immune_to_zombification(&self) -> bool {
        match self
            .metadata
            .value(&definitions::base_piglin::is_immune_to_zombification())
        {
            MetadataValue::Boolean(is_immune) => is_immune,
            _ => false,
        }
    }

    pub fn set_piglin_immune_to_zombification(&mut self, is_immune: bool) {
        self.metadata.set(
            &definitions::base_piglin::is_immune_to_zombification(),
            MetadataValue::Boolean(is_immune),
        );
    }

    pub fn is_baby_piglin(&self) -> bool {
        match self.metadata.value(&definitions::piglin::is_baby()) {
            MetadataValue::Boolean(is_baby) => is_baby,
            _ => false,
        }
    }

    pub fn set_baby_piglin(&mut self, is_baby: bool) {
        self.metadata.set(
            &definitions::piglin::is_baby(),
            MetadataValue::Boolean(is_baby),
        );
    }

    pub fn piglin_is_charging_crossbow(&self) -> bool {
        match self
            .metadata
            .value(&definitions::piglin::is_charging_crossbow())
        {
            MetadataValue::Boolean(is_charging) => is_charging,
            _ => false,
        }
    }

    pub fn set_piglin_charging_crossbow(&mut self, is_charging: bool) {
        self.metadata.set(
            &definitions::piglin::is_charging_crossbow(),
            MetadataValue::Boolean(is_charging),
        );
    }

    pub fn is_dancing_piglin(&self) -> bool {
        match self.metadata.value(&definitions::piglin::is_dancing()) {
            MetadataValue::Boolean(is_dancing) => is_dancing,
            _ => false,
        }
    }

    pub fn set_dancing_piglin(&mut self, is_dancing: bool) {
        self.metadata.set(
            &definitions::piglin::is_dancing(),
            MetadataValue::Boolean(is_dancing),
        );
    }

    pub fn creaking_can_move(&self) -> bool {
        match self.metadata.value(&definitions::creaking::can_move()) {
            MetadataValue::Boolean(can_move) => can_move,
            _ => true,
        }
    }

    pub fn set_creaking_can_move(&mut self, can_move: bool) {
        self.metadata.set(
            &definitions::creaking::can_move(),
            MetadataValue::Boolean(can_move),
        );
    }

    pub fn is_active_creaking(&self) -> bool {
        match self.metadata.value(&definitions::creaking::is_active()) {
            MetadataValue::Boolean(is_active) => is_active,
            _ => false,
        }
    }

    pub fn set_active_creaking(&mut self, is_active: bool) {
        self.metadata.set(
            &definitions::creaking::is_active(),
            MetadataValue::Boolean(is_active),
        );
    }

    pub fn is_tearing_down_creaking(&self) -> bool {
        match self
            .metadata
            .value(&definitions::creaking::is_tearing_down())
        {
            MetadataValue::Boolean(is_tearing_down) => is_tearing_down,
            _ => false,
        }
    }

    pub fn set_tearing_down_creaking(&mut self, is_tearing_down: bool) {
        self.metadata.set(
            &definitions::creaking::is_tearing_down(),
            MetadataValue::Boolean(is_tearing_down),
        );
    }

    pub fn creaking_home_position(&self) -> Option<Position> {
        match self.metadata.value(&definitions::creaking::home_position()) {
            MetadataValue::OptionalPosition(position) => position,
            _ => None,
        }
    }

    pub fn set_creaking_home_position(&mut self, position: Option<Position>) {
        self.metadata.set(
            &definitions::creaking::home_position(),
            MetadataValue::OptionalPosition(position),
        );
    }

    pub fn creeper_state(&self) -> CreeperState {
        match self.metadata.value(&definitions::creeper::state()) {
            MetadataValue::VarInt(state) => CreeperState::from_protocol_id(state),
            _ => CreeperState::default(),
        }
    }

    pub fn set_creeper_state(&mut self, state: CreeperState) {
        self.metadata.set(
            &definitions::creeper::state(),
            MetadataValue::VarInt(state.protocol_id()),
        );
    }

    pub fn is_charged_creeper(&self) -> bool {
        match self.metadata.value(&definitions::creeper::is_charged()) {
            MetadataValue::Boolean(is_charged) => is_charged,
            _ => false,
        }
    }

    pub fn set_charged_creeper(&mut self, is_charged: bool) {
        self.metadata.set(
            &definitions::creeper::is_charged(),
            MetadataValue::Boolean(is_charged),
        );
    }

    pub fn is_ignited_creeper(&self) -> bool {
        match self.metadata.value(&definitions::creeper::is_ignited()) {
            MetadataValue::Boolean(is_ignited) => is_ignited,
            _ => false,
        }
    }

    pub fn set_ignited_creeper(&mut self, is_ignited: bool) {
        self.metadata.set(
            &definitions::creeper::is_ignited(),
            MetadataValue::Boolean(is_ignited),
        );
    }

    pub fn is_screaming_enderman(&self) -> bool {
        match self.metadata.value(&definitions::enderman::is_screaming()) {
            MetadataValue::Boolean(is_screaming) => is_screaming,
            _ => false,
        }
    }

    pub fn enderman_carried_block(&self) -> Option<BlockState> {
        match self.metadata.value(&definitions::enderman::carried_block()) {
            MetadataValue::OptionalBlockState(0) => None,
            MetadataValue::OptionalBlockState(block_state_id) => {
                BlockState::from_state_id(block_state_id)
            }
            _ => None,
        }
    }

    pub fn set_enderman_carried_block(&mut self, carried_block: Option<BlockState>) {
        self.metadata.set(
            &definitions::enderman::carried_block(),
            MetadataValue::OptionalBlockState(carried_block.map_or(0, BlockState::state_id)),
        );
    }

    pub fn set_screaming_enderman(&mut self, is_screaming: bool) {
        self.metadata.set(
            &definitions::enderman::is_screaming(),
            MetadataValue::Boolean(is_screaming),
        );
    }

    pub fn is_staring_enderman(&self) -> bool {
        match self.metadata.value(&definitions::enderman::is_staring()) {
            MetadataValue::Boolean(is_staring) => is_staring,
            _ => false,
        }
    }

    pub fn set_staring_enderman(&mut self, is_staring: bool) {
        self.metadata.set(
            &definitions::enderman::is_staring(),
            MetadataValue::Boolean(is_staring),
        );
    }

    pub fn phantom_size(&self) -> i32 {
        match self.metadata.value(&definitions::phantom::size()) {
            MetadataValue::VarInt(size) => size,
            _ => 0,
        }
    }

    pub fn set_phantom_size(&mut self, size: i32) {
        self.metadata
            .set(&definitions::phantom::size(), MetadataValue::VarInt(size));
    }

    pub(crate) fn guardian_target_entity_id(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::guardian::target_entity_id())
        {
            MetadataValue::VarInt(entity_id) => entity_id,
            _ => 0,
        }
    }

    pub(crate) fn set_guardian_target_entity_id(&mut self, entity_id: i32) {
        self.metadata.set(
            &definitions::guardian::target_entity_id(),
            MetadataValue::VarInt(entity_id),
        );
    }

    pub fn raider_is_celebrating(&self) -> bool {
        match self.metadata.value(&definitions::raider::is_celebrating()) {
            MetadataValue::Boolean(is_celebrating) => is_celebrating,
            _ => false,
        }
    }

    pub fn set_raider_celebrating(&mut self, is_celebrating: bool) {
        self.metadata.set(
            &definitions::raider::is_celebrating(),
            MetadataValue::Boolean(is_celebrating),
        );
    }

    pub fn pillager_is_charging_crossbow(&self) -> bool {
        match self.metadata.value(&definitions::pillager::is_charging()) {
            MetadataValue::Boolean(is_charging) => is_charging,
            _ => false,
        }
    }

    pub fn set_pillager_charging_crossbow(&mut self, is_charging: bool) {
        self.metadata.set(
            &definitions::pillager::is_charging(),
            MetadataValue::Boolean(is_charging),
        );
    }

    pub fn witch_is_drinking_potion(&self) -> bool {
        match self
            .metadata
            .value(&definitions::witch::is_drinking_potion())
        {
            MetadataValue::Boolean(is_drinking) => is_drinking,
            _ => false,
        }
    }

    pub fn set_witch_drinking_potion(&mut self, is_drinking: bool) {
        self.metadata.set(
            &definitions::witch::is_drinking_potion(),
            MetadataValue::Boolean(is_drinking),
        );
    }

    pub fn is_climbing_spider(&self) -> bool {
        self.metadata.flag(&definitions::spider::is_climbing())
    }

    pub fn set_climbing_spider(&mut self, is_climbing: bool) {
        self.metadata
            .set_flag(&definitions::spider::is_climbing(), is_climbing);
    }

    pub fn warden_anger_level(&self) -> i32 {
        match self.metadata.value(&definitions::warden::anger_level()) {
            MetadataValue::VarInt(anger_level) => anger_level,
            _ => 0,
        }
    }

    pub fn set_warden_anger_level(&mut self, anger_level: i32) {
        self.metadata.set(
            &definitions::warden::anger_level(),
            MetadataValue::VarInt(anger_level),
        );
    }

    pub fn wither_center_head_entity_id(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::wither::center_head_target())
        {
            MetadataValue::VarInt(entity_id) => entity_id,
            _ => 0,
        }
    }

    pub fn set_wither_center_head_entity_id(&mut self, entity_id: i32) {
        self.metadata.set(
            &definitions::wither::center_head_target(),
            MetadataValue::VarInt(entity_id),
        );
    }

    pub fn wither_left_head_entity_id(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::wither::left_head_target())
        {
            MetadataValue::VarInt(entity_id) => entity_id,
            _ => 0,
        }
    }

    pub fn set_wither_left_head_entity_id(&mut self, entity_id: i32) {
        self.metadata.set(
            &definitions::wither::left_head_target(),
            MetadataValue::VarInt(entity_id),
        );
    }

    pub fn wither_right_head_entity_id(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::wither::right_head_target())
        {
            MetadataValue::VarInt(entity_id) => entity_id,
            _ => 0,
        }
    }

    pub fn set_wither_right_head_entity_id(&mut self, entity_id: i32) {
        self.metadata.set(
            &definitions::wither::right_head_target(),
            MetadataValue::VarInt(entity_id),
        );
    }

    pub fn wither_invulnerable_time(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::wither::invulnerable_time())
        {
            MetadataValue::VarInt(invulnerable_time) => invulnerable_time,
            _ => 0,
        }
    }

    pub fn set_wither_invulnerable_time(&mut self, invulnerable_time: i32) {
        self.metadata.set(
            &definitions::wither::invulnerable_time(),
            MetadataValue::VarInt(invulnerable_time),
        );
    }

    pub fn is_baby_zoglin(&self) -> bool {
        match self.metadata.value(&definitions::zoglin::is_baby()) {
            MetadataValue::Boolean(is_baby) => is_baby,
            _ => false,
        }
    }

    pub fn set_baby_zoglin(&mut self, is_baby: bool) {
        self.metadata.set(
            &definitions::zoglin::is_baby(),
            MetadataValue::Boolean(is_baby),
        );
    }

    pub fn is_baby_zombie(&self) -> bool {
        match self.metadata.value(&definitions::zombie::is_baby()) {
            MetadataValue::Boolean(is_baby) => is_baby,
            _ => false,
        }
    }

    pub fn set_baby_zombie(&mut self, is_baby: bool) {
        self.metadata.set(
            &definitions::zombie::is_baby(),
            MetadataValue::Boolean(is_baby),
        );
    }

    pub fn zombie_is_becoming_drowned(&self) -> bool {
        match self
            .metadata
            .value(&definitions::zombie::is_becoming_drowned())
        {
            MetadataValue::Boolean(is_becoming_drowned) => is_becoming_drowned,
            _ => false,
        }
    }

    pub fn set_zombie_becoming_drowned(&mut self, is_becoming_drowned: bool) {
        self.metadata.set(
            &definitions::zombie::is_becoming_drowned(),
            MetadataValue::Boolean(is_becoming_drowned),
        );
    }

    pub fn villager_head_shake_timer(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::abstract_villager::head_shake_timer())
        {
            MetadataValue::VarInt(timer) => timer,
            _ => 0,
        }
    }

    pub fn villager_data(&self) -> VillagerData {
        match self.metadata.value(&definitions::villager::data()) {
            MetadataValue::VillagerData(villager_type_id, profession_id, level_id) => {
                VillagerData::from_protocol_ids(villager_type_id, profession_id, level_id)
                    .unwrap_or_default()
            }
            _ => VillagerData::default(),
        }
    }

    pub fn set_villager_data(&mut self, villager_data: VillagerData) {
        self.metadata.set(
            &definitions::villager::data(),
            MetadataValue::VillagerData(
                villager_data.villager_type().protocol_id(),
                villager_data.profession().protocol_id(),
                villager_data.level().protocol_id(),
            ),
        );
    }

    fn villager_variant_component(&self) -> Option<VillagerType> {
        match self.entity_type {
            EntityType::VILLAGER => Some(self.villager_data().villager_type().clone()),
            EntityType::ZOMBIE_VILLAGER => {
                Some(self.zombie_villager_data().villager_type().clone())
            }
            _ => None,
        }
    }

    fn set_villager_variant_component(&mut self, villager_type: VillagerType) {
        match self.entity_type {
            EntityType::VILLAGER => {
                self.set_villager_data(self.villager_data().with_type(villager_type));
            }
            EntityType::ZOMBIE_VILLAGER => {
                self.set_zombie_villager_data(self.zombie_villager_data().with_type(villager_type));
            }
            _ => {}
        }
    }

    pub fn set_villager_head_shake_timer(&mut self, timer: i32) {
        self.metadata.set(
            &definitions::abstract_villager::head_shake_timer(),
            MetadataValue::VarInt(timer),
        );
    }

    pub fn zombie_villager_is_converting(&self) -> bool {
        match self
            .metadata
            .value(&definitions::zombie_villager::is_converting())
        {
            MetadataValue::Boolean(is_converting) => is_converting,
            _ => false,
        }
    }

    pub fn zombie_villager_data(&self) -> VillagerData {
        match self
            .metadata
            .value(&definitions::zombie_villager::villager_data())
        {
            MetadataValue::VillagerData(villager_type_id, profession_id, level_id) => {
                VillagerData::from_protocol_ids(villager_type_id, profession_id, level_id)
                    .unwrap_or_default()
            }
            _ => VillagerData::default(),
        }
    }

    pub fn set_zombie_villager_data(&mut self, villager_data: VillagerData) {
        self.metadata.set(
            &definitions::zombie_villager::villager_data(),
            MetadataValue::VillagerData(
                villager_data.villager_type().protocol_id(),
                villager_data.profession().protocol_id(),
                villager_data.level().protocol_id(),
            ),
        );
    }

    pub fn set_zombie_villager_converting(&mut self, is_converting: bool) {
        self.metadata.set(
            &definitions::zombie_villager::is_converting(),
            MetadataValue::Boolean(is_converting),
        );
    }

    pub fn is_player_created_iron_golem(&self) -> bool {
        self.metadata
            .flag(&definitions::iron_golem::is_player_created())
    }

    pub fn set_player_created_iron_golem(&mut self, is_player_created: bool) {
        self.metadata.set_flag(
            &definitions::iron_golem::is_player_created(),
            is_player_created,
        );
    }

    pub fn snow_golem_has_pumpkin_hat(&self) -> bool {
        self.metadata
            .flag(&definitions::snow_golem::has_pumpkin_hat())
    }

    pub fn set_snow_golem_has_pumpkin_hat(&mut self, has_pumpkin_hat: bool) {
        self.metadata
            .set_flag(&definitions::snow_golem::has_pumpkin_hat(), has_pumpkin_hat);
    }

    pub fn copper_golem_weather_state(&self) -> CopperGolemWeatherState {
        match self
            .metadata
            .value(&definitions::copper_golem::weather_state())
        {
            MetadataValue::WeatherState(state_id) => {
                CopperGolemWeatherState::from_protocol_id(state_id).unwrap_or_default()
            }
            _ => CopperGolemWeatherState::default(),
        }
    }

    pub fn set_copper_golem_weather_state(&mut self, state: CopperGolemWeatherState) {
        self.metadata.set(
            &definitions::copper_golem::weather_state(),
            MetadataValue::WeatherState(state.protocol_id()),
        );
    }

    pub fn copper_golem_state(&self) -> CopperGolemState {
        match self.metadata.value(&definitions::copper_golem::state()) {
            MetadataValue::CopperGolemState(state_id) => {
                CopperGolemState::from_protocol_id(state_id).unwrap_or_default()
            }
            _ => CopperGolemState::default(),
        }
    }

    pub fn set_copper_golem_state(&mut self, state: CopperGolemState) {
        self.metadata.set(
            &definitions::copper_golem::state(),
            MetadataValue::CopperGolemState(state.protocol_id()),
        );
    }

    pub fn shulker_shield_height(&self) -> i8 {
        match self.metadata.value(&definitions::shulker::shield_height()) {
            MetadataValue::Byte(shield_height) => shield_height,
            _ => 0,
        }
    }

    pub fn shulker_attach_face(&self) -> BlockFaceDirection {
        match self.metadata.value(&definitions::shulker::attach_face()) {
            MetadataValue::Direction(direction_id) => {
                BlockFaceDirection::from_protocol_id(direction_id)
                    .unwrap_or(BlockFaceDirection::Down)
            }
            _ => BlockFaceDirection::Down,
        }
    }

    pub fn set_shulker_attach_face(&mut self, attach_face: BlockFaceDirection) {
        self.metadata.set(
            &definitions::shulker::attach_face(),
            MetadataValue::Direction(attach_face.protocol_id()),
        );
    }

    pub fn set_shulker_shield_height(&mut self, shield_height: i8) {
        self.metadata.set(
            &definitions::shulker::shield_height(),
            MetadataValue::Byte(shield_height),
        );
    }

    pub fn shulker_color(&self) -> DyeColor {
        match self.metadata.value(&definitions::shulker::color()) {
            MetadataValue::Byte(16) => DyeColor::Purple,
            MetadataValue::Byte(color_id) => DyeColor::ALL
                .get(color_id as usize)
                .copied()
                .unwrap_or(DyeColor::Purple),
            _ => DyeColor::Purple,
        }
    }

    pub fn set_shulker_color(&mut self, color: DyeColor) {
        let color_id = DyeColor::ALL
            .iter()
            .position(|candidate| candidate == &color)
            .unwrap_or(10) as i8;
        self.metadata.set(
            &definitions::shulker::color(),
            MetadataValue::Byte(color_id),
        );
    }

    pub fn spellcaster_illager_spell(&self) -> SpellcasterIllagerSpell {
        match self
            .metadata
            .value(&definitions::spellcaster_illager::spell())
        {
            MetadataValue::Byte(spell_id) => {
                SpellcasterIllagerSpell::from_protocol_id(spell_id).unwrap_or_default()
            }
            _ => SpellcasterIllagerSpell::default(),
        }
    }

    pub fn set_spellcaster_illager_spell(&mut self, spell: SpellcasterIllagerSpell) {
        self.metadata.set(
            &definitions::spellcaster_illager::spell(),
            MetadataValue::Byte(spell.protocol_id()),
        );
    }

    pub fn shaking_ticks(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::abstract_vehicle::shaking_power())
        {
            MetadataValue::VarInt(shaking_ticks) => shaking_ticks,
            _ => 0,
        }
    }

    pub fn set_shaking_ticks(&mut self, shaking_ticks: i32) {
        self.metadata.set(
            &definitions::abstract_vehicle::shaking_power(),
            MetadataValue::VarInt(shaking_ticks),
        );
    }

    pub fn shaking_direction(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::abstract_vehicle::shaking_direction())
        {
            MetadataValue::VarInt(shaking_direction) => shaking_direction,
            _ => 1,
        }
    }

    pub fn set_shaking_direction(&mut self, shaking_direction: i32) {
        self.metadata.set(
            &definitions::abstract_vehicle::shaking_direction(),
            MetadataValue::VarInt(shaking_direction),
        );
    }

    pub fn shaking_multiplier(&self) -> f32 {
        match self
            .metadata
            .value(&definitions::abstract_vehicle::shaking_multiplier())
        {
            MetadataValue::Float(shaking_multiplier) => shaking_multiplier,
            _ => 0.0,
        }
    }

    pub fn set_shaking_multiplier(&mut self, shaking_multiplier: f32) {
        self.metadata.set(
            &definitions::abstract_vehicle::shaking_multiplier(),
            MetadataValue::Float(shaking_multiplier),
        );
    }

    pub fn main_hand(&self) -> MainHand {
        match self.metadata.value(&definitions::avatar::main_hand()) {
            MetadataValue::MainHand(main_hand) => main_hand,
            _ => MainHand::Right,
        }
    }

    pub fn set_main_hand(&mut self, main_hand: MainHand) {
        self.metadata.set(
            &definitions::avatar::main_hand(),
            MetadataValue::MainHand(main_hand),
        );
    }

    pub fn is_cape_enabled(&self) -> bool {
        self.metadata.flag(&definitions::avatar::is_cape_enabled())
    }

    pub fn set_cape_enabled(&mut self, cape_enabled: bool) {
        self.metadata
            .set_flag(&definitions::avatar::is_cape_enabled(), cape_enabled);
    }

    pub fn is_jacket_enabled(&self) -> bool {
        self.metadata
            .flag(&definitions::avatar::is_jacket_enabled())
    }

    pub fn set_jacket_enabled(&mut self, jacket_enabled: bool) {
        self.metadata
            .set_flag(&definitions::avatar::is_jacket_enabled(), jacket_enabled);
    }

    pub fn is_left_sleeve_enabled(&self) -> bool {
        self.metadata
            .flag(&definitions::avatar::is_left_sleeve_enabled())
    }

    pub fn set_left_sleeve_enabled(&mut self, left_sleeve_enabled: bool) {
        self.metadata.set_flag(
            &definitions::avatar::is_left_sleeve_enabled(),
            left_sleeve_enabled,
        );
    }

    pub fn is_right_sleeve_enabled(&self) -> bool {
        self.metadata
            .flag(&definitions::avatar::is_right_sleeve_enabled())
    }

    pub fn set_right_sleeve_enabled(&mut self, right_sleeve_enabled: bool) {
        self.metadata.set_flag(
            &definitions::avatar::is_right_sleeve_enabled(),
            right_sleeve_enabled,
        );
    }

    pub fn is_left_leg_enabled(&self) -> bool {
        self.metadata
            .flag(&definitions::avatar::is_left_pants_leg_enabled())
    }

    pub fn set_left_leg_enabled(&mut self, left_leg_enabled: bool) {
        self.metadata.set_flag(
            &definitions::avatar::is_left_pants_leg_enabled(),
            left_leg_enabled,
        );
    }

    pub fn is_right_leg_enabled(&self) -> bool {
        self.metadata
            .flag(&definitions::avatar::is_right_pants_leg_enabled())
    }

    pub fn set_right_leg_enabled(&mut self, right_leg_enabled: bool) {
        self.metadata.set_flag(
            &definitions::avatar::is_right_pants_leg_enabled(),
            right_leg_enabled,
        );
    }

    pub fn is_hat_enabled(&self) -> bool {
        self.metadata.flag(&definitions::avatar::is_hat_enabled())
    }

    pub fn set_hat_enabled(&mut self, hat_enabled: bool) {
        self.metadata
            .set_flag(&definitions::avatar::is_hat_enabled(), hat_enabled);
    }

    pub fn displayed_skin_parts(&self) -> i8 {
        match self
            .metadata
            .value(&definitions::avatar::displayed_model_parts_flags())
        {
            MetadataValue::Byte(displayed_skin_parts) => displayed_skin_parts,
            _ => 0,
        }
    }

    pub fn set_displayed_skin_parts(&mut self, displayed_skin_parts: i8) {
        self.metadata.set(
            &definitions::avatar::displayed_model_parts_flags(),
            MetadataValue::Byte(displayed_skin_parts),
        );
    }

    pub fn additional_hearts(&self) -> f32 {
        match self
            .metadata
            .value(&definitions::player::additional_hearts())
        {
            MetadataValue::Float(additional_hearts) => additional_hearts,
            _ => 0.0,
        }
    }

    pub fn set_additional_hearts(&mut self, additional_hearts: f32) {
        self.metadata.set(
            &definitions::player::additional_hearts(),
            MetadataValue::Float(additional_hearts),
        );
    }

    pub fn score(&self) -> i32 {
        match self.metadata.value(&definitions::player::score()) {
            MetadataValue::VarInt(score) => score,
            _ => 0,
        }
    }

    pub fn set_score(&mut self, score: i32) {
        self.metadata
            .set(&definitions::player::score(), MetadataValue::VarInt(score));
    }

    pub fn left_shoulder_entity_data(&self) -> Option<i32> {
        match self
            .metadata
            .value(&definitions::player::left_shoulder_entity_data())
        {
            MetadataValue::OptionalVarInt(left_shoulder_entity_data) => left_shoulder_entity_data,
            _ => None,
        }
    }

    pub fn set_left_shoulder_entity_data(&mut self, left_shoulder_entity_data: Option<i32>) {
        self.metadata.set(
            &definitions::player::left_shoulder_entity_data(),
            MetadataValue::OptionalVarInt(left_shoulder_entity_data),
        );
    }

    pub fn right_shoulder_entity_data(&self) -> Option<i32> {
        match self
            .metadata
            .value(&definitions::player::right_shoulder_entity_data())
        {
            MetadataValue::OptionalVarInt(right_shoulder_entity_data) => right_shoulder_entity_data,
            _ => None,
        }
    }

    pub fn set_right_shoulder_entity_data(&mut self, right_shoulder_entity_data: Option<i32>) {
        self.metadata.set(
            &definitions::player::right_shoulder_entity_data(),
            MetadataValue::OptionalVarInt(right_shoulder_entity_data),
        );
    }

    pub fn profile(&self) -> ResolvableProfile {
        match self.metadata.value(&definitions::mannequin::profile()) {
            MetadataValue::ResolvableProfile(profile) => profile,
            _ => ResolvableProfile::default(),
        }
    }

    pub fn set_profile(&mut self, profile: ResolvableProfile) {
        self.metadata.set(
            &definitions::mannequin::profile(),
            MetadataValue::ResolvableProfile(profile),
        );
    }

    pub fn is_immovable(&self) -> bool {
        match self.metadata.value(&definitions::mannequin::immovable()) {
            MetadataValue::Boolean(immovable) => immovable,
            _ => false,
        }
    }

    pub fn set_immovable(&mut self, immovable: bool) {
        self.metadata.set(
            &definitions::mannequin::immovable(),
            MetadataValue::Boolean(immovable),
        );
    }

    pub fn description(&self) -> Option<TextComponent> {
        match self.metadata.value(&definitions::mannequin::description()) {
            MetadataValue::OptionalText(description) => description,
            _ => None,
        }
    }

    pub fn set_description(&mut self, description: Option<TextComponent>) {
        self.metadata.set(
            &definitions::mannequin::description(),
            MetadataValue::OptionalText(description),
        );
    }

    pub fn transformation_interpolation_start_delta(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::display::interpolation_delay())
        {
            MetadataValue::VarInt(interpolation_delay) => interpolation_delay,
            _ => 0,
        }
    }

    pub fn set_transformation_interpolation_start_delta(&mut self, interpolation_delay: i32) {
        self.metadata.set(
            &definitions::display::interpolation_delay(),
            MetadataValue::VarInt(interpolation_delay),
        );
    }

    pub fn transformation_interpolation_duration(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::display::transformation_interpolation_duration())
        {
            MetadataValue::VarInt(transformation_interpolation_duration) => {
                transformation_interpolation_duration
            }
            _ => 0,
        }
    }

    pub fn set_transformation_interpolation_duration(
        &mut self,
        transformation_interpolation_duration: i32,
    ) {
        self.metadata.set(
            &definitions::display::transformation_interpolation_duration(),
            MetadataValue::VarInt(transformation_interpolation_duration),
        );
    }

    pub fn position_rotation_interpolation_duration(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::display::position_rotation_interpolation_duration())
        {
            MetadataValue::VarInt(position_rotation_interpolation_duration) => {
                position_rotation_interpolation_duration
            }
            _ => 0,
        }
    }

    pub fn set_position_rotation_interpolation_duration(
        &mut self,
        position_rotation_interpolation_duration: i32,
    ) {
        self.metadata.set(
            &definitions::display::position_rotation_interpolation_duration(),
            MetadataValue::VarInt(position_rotation_interpolation_duration),
        );
    }

    pub fn display_translation(&self) -> Vector3f {
        match self.metadata.value(&definitions::display::translation()) {
            MetadataValue::Vector3f(translation) => translation,
            _ => Vector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }

    pub fn set_display_translation(&mut self, translation: Vector3f) {
        self.metadata.set(
            &definitions::display::translation(),
            MetadataValue::Vector3f(translation),
        );
    }

    pub fn display_scale(&self) -> Vector3f {
        match self.metadata.value(&definitions::display::scale()) {
            MetadataValue::Vector3f(scale) => scale,
            _ => Vector3f {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }
    }

    pub fn set_display_scale(&mut self, scale: Vector3f) {
        self.metadata.set(
            &definitions::display::scale(),
            MetadataValue::Vector3f(scale),
        );
    }

    pub fn left_rotation(&self) -> Quaternionf {
        match self.metadata.value(&definitions::display::rotation_left()) {
            MetadataValue::Quaternionf(rotation) => rotation,
            _ => Quaternionf {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        }
    }

    pub fn set_left_rotation(&mut self, rotation: Quaternionf) {
        self.metadata.set(
            &definitions::display::rotation_left(),
            MetadataValue::Quaternionf(rotation),
        );
    }

    pub fn right_rotation(&self) -> Quaternionf {
        match self.metadata.value(&definitions::display::rotation_right()) {
            MetadataValue::Quaternionf(rotation) => rotation,
            _ => Quaternionf {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        }
    }

    pub fn set_right_rotation(&mut self, rotation: Quaternionf) {
        self.metadata.set(
            &definitions::display::rotation_right(),
            MetadataValue::Quaternionf(rotation),
        );
    }

    pub fn billboard_render_constraints(&self) -> i8 {
        match self
            .metadata
            .value(&definitions::display::billboard_constraints())
        {
            MetadataValue::Byte(billboard_render_constraints) => billboard_render_constraints,
            _ => 0,
        }
    }

    pub fn set_billboard_render_constraints(&mut self, billboard_render_constraints: i8) {
        self.metadata.set(
            &definitions::display::billboard_constraints(),
            MetadataValue::Byte(billboard_render_constraints),
        );
    }

    pub fn brightness_override(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::display::brightness_override())
        {
            MetadataValue::VarInt(brightness_override) => brightness_override,
            _ => -1,
        }
    }

    pub fn set_brightness_override(&mut self, brightness_override: i32) {
        self.metadata.set(
            &definitions::display::brightness_override(),
            MetadataValue::VarInt(brightness_override),
        );
    }

    pub fn set_brightness(&mut self, block_light: i32, sky_light: i32) {
        self.set_brightness_override((block_light & 0xF) << 4 | (sky_light & 0xF) << 20);
    }

    pub fn block_light(&self) -> i32 {
        self.display_light_at_shift(4)
    }

    pub fn sky_light(&self) -> i32 {
        self.display_light_at_shift(20)
    }

    fn display_light_at_shift(&self, shift: i32) -> i32 {
        let brightness_override = self.brightness_override();
        if brightness_override <= 0 {
            return 0;
        }
        (brightness_override >> shift) & 0xF
    }

    pub fn display_view_range(&self) -> f32 {
        match self.metadata.value(&definitions::display::view_range()) {
            MetadataValue::Float(view_range) => view_range,
            _ => 1.0,
        }
    }

    pub fn set_display_view_range(&mut self, view_range: f32) {
        self.metadata.set(
            &definitions::display::view_range(),
            MetadataValue::Float(view_range),
        );
    }

    pub fn shadow_radius(&self) -> f32 {
        match self.metadata.value(&definitions::display::shadow_radius()) {
            MetadataValue::Float(shadow_radius) => shadow_radius,
            _ => 0.0,
        }
    }

    pub fn set_shadow_radius(&mut self, shadow_radius: f32) {
        self.metadata.set(
            &definitions::display::shadow_radius(),
            MetadataValue::Float(shadow_radius),
        );
    }

    pub fn shadow_strength(&self) -> f32 {
        match self
            .metadata
            .value(&definitions::display::shadow_strength())
        {
            MetadataValue::Float(shadow_strength) => shadow_strength,
            _ => 1.0,
        }
    }

    pub fn set_shadow_strength(&mut self, shadow_strength: f32) {
        self.metadata.set(
            &definitions::display::shadow_strength(),
            MetadataValue::Float(shadow_strength),
        );
    }

    pub fn display_width(&self) -> f32 {
        match self.metadata.value(&definitions::display::width()) {
            MetadataValue::Float(width) => width,
            _ => 0.0,
        }
    }

    pub fn set_display_width(&mut self, width: f32) {
        self.metadata
            .set(&definitions::display::width(), MetadataValue::Float(width));
    }

    pub fn display_height(&self) -> f32 {
        match self.metadata.value(&definitions::display::height()) {
            MetadataValue::Float(height) => height,
            _ => 0.0,
        }
    }

    pub fn set_display_height(&mut self, height: f32) {
        self.metadata.set(
            &definitions::display::height(),
            MetadataValue::Float(height),
        );
    }

    pub fn glow_color_override(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::display::glow_color_override())
        {
            MetadataValue::VarInt(glow_color_override) => glow_color_override,
            _ => -1,
        }
    }

    pub fn set_glow_color_override(&mut self, glow_color_override: i32) {
        self.metadata.set(
            &definitions::display::glow_color_override(),
            MetadataValue::VarInt(glow_color_override),
        );
    }

    pub fn displayed_block_state(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::block_display::displayed_block_state())
        {
            MetadataValue::BlockState(displayed_block_state) => displayed_block_state,
            _ => 0,
        }
    }

    pub fn set_displayed_block_state(&mut self, displayed_block_state: i32) {
        self.metadata.set(
            &definitions::block_display::displayed_block_state(),
            MetadataValue::BlockState(displayed_block_state),
        );
    }

    pub fn displayed_item(&self) -> Slot {
        match self
            .metadata
            .value(&definitions::item_display::displayed_item())
        {
            MetadataValue::Slot(displayed_item) => displayed_item,
            _ => Slot::from_item_stack(&ItemStack::air()),
        }
    }

    pub fn set_displayed_item(&mut self, displayed_item: Slot) {
        self.metadata.set(
            &definitions::item_display::displayed_item(),
            MetadataValue::Slot(displayed_item),
        );
    }

    pub fn display_context(&self) -> i8 {
        match self
            .metadata
            .value(&definitions::item_display::display_type())
        {
            MetadataValue::Byte(display_context) => display_context,
            _ => 0,
        }
    }

    pub fn set_display_context(&mut self, display_context: i8) {
        self.metadata.set(
            &definitions::item_display::display_type(),
            MetadataValue::Byte(display_context),
        );
    }

    pub fn display_text(&self) -> TextComponent {
        match self.metadata.value(&definitions::text_display::text()) {
            MetadataValue::Text(text) => text,
            _ => TextComponent::empty(),
        }
    }

    pub fn set_display_text(&mut self, text: TextComponent) {
        self.metadata.set(
            &definitions::text_display::text(),
            MetadataValue::Text(text),
        );
    }

    pub fn line_width(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::text_display::line_width())
        {
            MetadataValue::VarInt(line_width) => line_width,
            _ => 200,
        }
    }

    pub fn set_line_width(&mut self, line_width: i32) {
        self.metadata.set(
            &definitions::text_display::line_width(),
            MetadataValue::VarInt(line_width),
        );
    }

    pub fn background_color(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::text_display::background_color())
        {
            MetadataValue::VarInt(background_color) => background_color,
            _ => 0x40000000,
        }
    }

    pub fn set_background_color(&mut self, background_color: i32) {
        self.metadata.set(
            &definitions::text_display::background_color(),
            MetadataValue::VarInt(background_color),
        );
    }

    pub fn text_opacity(&self) -> i8 {
        match self
            .metadata
            .value(&definitions::text_display::text_opacity())
        {
            MetadataValue::Byte(text_opacity) => text_opacity,
            _ => -1,
        }
    }

    pub fn set_text_opacity(&mut self, text_opacity: i8) {
        self.metadata.set(
            &definitions::text_display::text_opacity(),
            MetadataValue::Byte(text_opacity),
        );
    }

    pub fn has_text_shadow(&self) -> bool {
        self.metadata.flag(&definitions::text_display::has_shadow())
    }

    pub fn set_text_shadow(&mut self, text_shadow: bool) {
        self.metadata
            .set_flag(&definitions::text_display::has_shadow(), text_shadow);
    }

    pub fn is_text_see_through(&self) -> bool {
        self.metadata
            .flag(&definitions::text_display::is_see_through())
    }

    pub fn set_text_see_through(&mut self, text_see_through: bool) {
        self.metadata.set_flag(
            &definitions::text_display::is_see_through(),
            text_see_through,
        );
    }

    pub fn uses_default_text_background(&self) -> bool {
        self.metadata
            .flag(&definitions::text_display::uses_default_background_color())
    }

    pub fn set_uses_default_text_background(&mut self, uses_default_text_background: bool) {
        self.metadata.set_flag(
            &definitions::text_display::uses_default_background_color(),
            uses_default_text_background,
        );
    }

    pub fn is_text_left_aligned(&self) -> bool {
        self.metadata
            .flag(&definitions::text_display::is_left_aligned())
    }

    pub fn set_text_left_aligned(&mut self, text_left_aligned: bool) {
        self.metadata.set_flag(
            &definitions::text_display::is_left_aligned(),
            text_left_aligned,
        );
    }

    pub fn is_text_right_aligned(&self) -> bool {
        self.metadata
            .flag(&definitions::text_display::is_right_aligned())
    }

    pub fn set_text_right_aligned(&mut self, text_right_aligned: bool) {
        self.metadata.set_flag(
            &definitions::text_display::is_right_aligned(),
            text_right_aligned,
        );
    }

    pub fn text_alignment(&self) -> i8 {
        self.metadata.byte(&definitions::text_display::alignment())
    }

    pub fn set_text_alignment(&mut self, text_alignment: i8) {
        self.metadata
            .set_byte(&definitions::text_display::alignment(), text_alignment);
    }

    pub fn interaction_width(&self) -> f32 {
        match self.metadata.value(&definitions::interaction::width()) {
            MetadataValue::Float(width) => width,
            _ => 1.0,
        }
    }

    pub fn set_interaction_width(&mut self, width: f32) {
        self.metadata.set(
            &definitions::interaction::width(),
            MetadataValue::Float(width),
        );
    }

    pub fn interaction_height(&self) -> f32 {
        match self.metadata.value(&definitions::interaction::height()) {
            MetadataValue::Float(height) => height,
            _ => 1.0,
        }
    }

    pub fn set_interaction_height(&mut self, height: f32) {
        self.metadata.set(
            &definitions::interaction::height(),
            MetadataValue::Float(height),
        );
    }

    pub fn has_interaction_response(&self) -> bool {
        match self.metadata.value(&definitions::interaction::responsive()) {
            MetadataValue::Boolean(response) => response,
            _ => false,
        }
    }

    pub fn set_interaction_response(&mut self, response: bool) {
        self.metadata.set(
            &definitions::interaction::responsive(),
            MetadataValue::Boolean(response),
        );
    }

    pub fn area_effect_cloud_radius(&self) -> f32 {
        match self
            .metadata
            .value(&definitions::area_effect_cloud::radius())
        {
            MetadataValue::Float(radius) => radius,
            _ => 0.5,
        }
    }

    pub fn set_area_effect_cloud_radius(&mut self, radius: f32) {
        self.metadata.set(
            &definitions::area_effect_cloud::radius(),
            MetadataValue::Float(radius),
        );
    }

    pub fn is_area_effect_cloud_waiting(&self) -> bool {
        match self
            .metadata
            .value(&definitions::area_effect_cloud::waiting())
        {
            MetadataValue::Boolean(waiting) => waiting,
            _ => false,
        }
    }

    pub fn set_area_effect_cloud_waiting(&mut self, waiting: bool) {
        self.metadata.set(
            &definitions::area_effect_cloud::waiting(),
            MetadataValue::Boolean(waiting),
        );
    }

    pub fn area_effect_cloud_particle(&self) -> Particle {
        match self
            .metadata
            .value(&definitions::area_effect_cloud::particle())
        {
            MetadataValue::Particle(particle) => particle,
            _ => Particle::effect(),
        }
    }

    pub fn set_area_effect_cloud_particle(&mut self, particle: Particle) {
        self.metadata.set(
            &definitions::area_effect_cloud::particle(),
            MetadataValue::Particle(particle),
        );
    }

    pub fn hooked_entity_id(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::fishing_hook::hooked_entity_id())
        {
            MetadataValue::VarInt(hooked_entity_id) => hooked_entity_id,
            _ => 0,
        }
    }

    pub fn set_hooked_entity_id(&mut self, hooked_entity_id: i32) {
        self.metadata.set(
            &definitions::fishing_hook::hooked_entity_id(),
            MetadataValue::VarInt(hooked_entity_id),
        );
    }

    pub fn is_fishing_hook_catchable(&self) -> bool {
        match self
            .metadata
            .value(&definitions::fishing_hook::is_catchable())
        {
            MetadataValue::Boolean(catchable) => catchable,
            _ => false,
        }
    }

    pub fn set_fishing_hook_catchable(&mut self, catchable: bool) {
        self.metadata.set(
            &definitions::fishing_hook::is_catchable(),
            MetadataValue::Boolean(catchable),
        );
    }

    pub fn is_left_paddle_turning(&self) -> bool {
        match self
            .metadata
            .value(&definitions::boat::is_left_paddle_turning())
        {
            MetadataValue::Boolean(left_paddle_turning) => left_paddle_turning,
            _ => false,
        }
    }

    pub fn set_left_paddle_turning(&mut self, left_paddle_turning: bool) {
        self.metadata.set(
            &definitions::boat::is_left_paddle_turning(),
            MetadataValue::Boolean(left_paddle_turning),
        );
    }

    pub fn is_right_paddle_turning(&self) -> bool {
        match self
            .metadata
            .value(&definitions::boat::is_right_paddle_turning())
        {
            MetadataValue::Boolean(right_paddle_turning) => right_paddle_turning,
            _ => false,
        }
    }

    pub fn set_right_paddle_turning(&mut self, right_paddle_turning: bool) {
        self.metadata.set(
            &definitions::boat::is_right_paddle_turning(),
            MetadataValue::Boolean(right_paddle_turning),
        );
    }

    pub fn splash_timer(&self) -> i32 {
        match self.metadata.value(&definitions::boat::splash_timer()) {
            MetadataValue::VarInt(splash_timer) => splash_timer,
            _ => 0,
        }
    }

    pub fn set_splash_timer(&mut self, splash_timer: i32) {
        self.metadata.set(
            &definitions::boat::splash_timer(),
            MetadataValue::VarInt(splash_timer),
        );
    }

    pub fn custom_minecart_block_state(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::abstract_minecart::custom_block_state())
        {
            MetadataValue::OptionalBlockState(custom_block_state) => custom_block_state,
            _ => 0,
        }
    }

    pub fn set_custom_minecart_block_state(&mut self, custom_block_state: i32) {
        self.metadata.set(
            &definitions::abstract_minecart::custom_block_state(),
            MetadataValue::OptionalBlockState(custom_block_state),
        );
    }

    pub fn custom_minecart_block_y_position(&self) -> i32 {
        match self
            .metadata
            .value(&definitions::abstract_minecart::custom_block_y_position())
        {
            MetadataValue::VarInt(custom_block_y_position) => custom_block_y_position,
            _ => 6,
        }
    }

    pub fn set_custom_minecart_block_y_position(&mut self, custom_block_y_position: i32) {
        self.metadata.set(
            &definitions::abstract_minecart::custom_block_y_position(),
            MetadataValue::VarInt(custom_block_y_position),
        );
    }

    pub fn has_furnace_minecart_fuel(&self) -> bool {
        match self
            .metadata
            .value(&definitions::furnace_minecart::has_fuel())
        {
            MetadataValue::Boolean(has_fuel) => has_fuel,
            _ => false,
        }
    }

    pub fn set_furnace_minecart_fuel(&mut self, has_fuel: bool) {
        self.metadata.set(
            &definitions::furnace_minecart::has_fuel(),
            MetadataValue::Boolean(has_fuel),
        );
    }

    pub fn command_block_minecart_command(&self) -> String {
        match self
            .metadata
            .value(&definitions::command_block_minecart::command())
        {
            MetadataValue::String(command) => command,
            _ => String::new(),
        }
    }

    pub fn set_command_block_minecart_command(&mut self, command: String) {
        self.metadata.set(
            &definitions::command_block_minecart::command(),
            MetadataValue::String(command),
        );
    }

    pub fn command_block_minecart_last_output(&self) -> TextComponent {
        match self
            .metadata
            .value(&definitions::command_block_minecart::last_output())
        {
            MetadataValue::Text(last_output) => last_output,
            _ => TextComponent::empty(),
        }
    }

    pub fn set_command_block_minecart_last_output(&mut self, last_output: TextComponent) {
        self.metadata.set(
            &definitions::command_block_minecart::last_output(),
            MetadataValue::Text(last_output),
        );
    }

    pub fn end_crystal_beam_target(&self) -> Option<Position> {
        match self
            .metadata
            .value(&definitions::end_crystal::beam_target())
        {
            MetadataValue::OptionalPosition(beam_target) => beam_target,
            _ => None,
        }
    }

    pub fn set_end_crystal_beam_target(&mut self, beam_target: Option<Position>) {
        self.metadata.set(
            &definitions::end_crystal::beam_target(),
            MetadataValue::OptionalPosition(beam_target),
        );
    }

    pub fn is_end_crystal_showing_bottom(&self) -> bool {
        match self
            .metadata
            .value(&definitions::end_crystal::show_bottom())
        {
            MetadataValue::Boolean(show_bottom) => show_bottom,
            _ => true,
        }
    }

    pub fn set_end_crystal_showing_bottom(&mut self, show_bottom: bool) {
        self.metadata.set(
            &definitions::end_crystal::show_bottom(),
            MetadataValue::Boolean(show_bottom),
        );
    }

    pub fn hanging_direction(&self) -> i32 {
        match self.metadata.value(&definitions::hanging::direction()) {
            MetadataValue::Direction(direction) => direction,
            _ => 3,
        }
    }

    pub fn set_hanging_direction(&mut self, direction: i32) {
        self.metadata.set(
            &definitions::hanging::direction(),
            MetadataValue::Direction(direction),
        );
    }

    pub fn item_frame_item(&self) -> Slot {
        match self.metadata.value(&definitions::item_frame::item()) {
            MetadataValue::Slot(item) => item,
            _ => Slot::from_item_stack(&ItemStack::air()),
        }
    }

    pub fn set_item_frame_item(&mut self, item: Slot) {
        self.metadata
            .set(&definitions::item_frame::item(), MetadataValue::Slot(item));
    }

    pub fn item_frame_rotation(&self) -> i32 {
        match self.metadata.value(&definitions::item_frame::rotation()) {
            MetadataValue::VarInt(rotation) => rotation,
            _ => 0,
        }
    }

    pub fn set_item_frame_rotation(&mut self, rotation: i32) {
        self.metadata.set(
            &definitions::item_frame::rotation(),
            MetadataValue::VarInt(rotation),
        );
    }

    pub fn painting_variant(&self) -> i32 {
        match self.metadata.value(&definitions::painting::variant()) {
            MetadataValue::PaintingVariant(variant) => variant,
            _ => 24,
        }
    }

    pub fn set_painting_variant(&mut self, variant: i32) {
        self.metadata.set(
            &definitions::painting::variant(),
            MetadataValue::PaintingVariant(variant),
        );
    }

    pub fn primed_tnt_fuse_time(&self) -> i32 {
        match self.metadata.value(&definitions::primed_tnt::fuse_time()) {
            MetadataValue::VarInt(fuse_time) => fuse_time,
            _ => 80,
        }
    }

    pub fn set_primed_tnt_fuse_time(&mut self, fuse_time: i32) {
        self.metadata.set(
            &definitions::primed_tnt::fuse_time(),
            MetadataValue::VarInt(fuse_time),
        );
    }

    pub fn primed_tnt_block_state(&self) -> i32 {
        match self.metadata.value(&definitions::primed_tnt::block_state()) {
            MetadataValue::BlockState(block_state) => block_state,
            _ => spinel_registry::vanilla_world_blocks::Block::TNT.state_id(),
        }
    }

    pub fn set_primed_tnt_block_state(&mut self, block_state: i32) {
        self.metadata.set(
            &definitions::primed_tnt::block_state(),
            MetadataValue::BlockState(block_state),
        );
    }

    pub fn ominous_item_spawner_item(&self) -> Slot {
        match self
            .metadata
            .value(&definitions::ominous_item_spawner::item())
        {
            MetadataValue::Slot(item) => item,
            _ => Slot::from_item_stack(&ItemStack::air()),
        }
    }

    pub fn set_ominous_item_spawner_item(&mut self, item: Slot) {
        self.metadata.set(
            &definitions::ominous_item_spawner::item(),
            MetadataValue::Slot(item),
        );
    }

    pub const fn falling_block_state(&self) -> i32 {
        self.falling_block_state
    }

    pub fn set_falling_block_state(&mut self, block_state: i32) {
        self.falling_block_state = block_state;
    }

    pub const fn fishing_hook_owner_entity_id(&self) -> Option<EntityId> {
        self.fishing_hook_owner_entity_id
    }

    pub fn set_fishing_hook_owner_entity_id(&mut self, owner_entity_id: Option<EntityId>) {
        self.fishing_hook_owner_entity_id = owner_entity_id;
    }

    pub fn slime_size(&self) -> i32 {
        match self.metadata.value(&definitions::slime::size()) {
            MetadataValue::VarInt(size) => size,
            _ => 1,
        }
    }

    pub fn set_slime_size(&mut self, size: i32) {
        let box_size = f64::from(0.51000005_f32 * size as f32);
        self.set_bounding_box_dimensions(box_size, box_size, box_size);
        self.metadata
            .set(&definitions::slime::size(), MetadataValue::VarInt(size));
    }

    pub fn ender_dragon_phase(&self) -> i32 {
        match self.metadata.value(&definitions::ender_dragon::phase()) {
            MetadataValue::VarInt(phase) => phase,
            _ => 10,
        }
    }

    pub fn set_ender_dragon_phase(&mut self, phase: i32) {
        self.metadata.set(
            &definitions::ender_dragon::phase(),
            MetadataValue::VarInt(phase),
        );
    }

    pub fn is_armor_stand_small(&self) -> bool {
        self.metadata.flag(&definitions::armor_stand::is_small())
    }

    pub fn set_armor_stand_small(&mut self, small: bool) {
        self.metadata
            .set_flag(&definitions::armor_stand::is_small(), small);
    }

    pub fn has_armor_stand_arms(&self) -> bool {
        self.metadata.flag(&definitions::armor_stand::has_arms())
    }

    pub fn set_armor_stand_arms(&mut self, has_arms: bool) {
        self.metadata
            .set_flag(&definitions::armor_stand::has_arms(), has_arms);
    }

    pub fn has_armor_stand_no_base_plate(&self) -> bool {
        self.metadata
            .flag(&definitions::armor_stand::has_no_base_plate())
    }

    pub fn set_armor_stand_no_base_plate(&mut self, has_no_base_plate: bool) {
        self.metadata.set_flag(
            &definitions::armor_stand::has_no_base_plate(),
            has_no_base_plate,
        );
    }

    pub fn is_armor_stand_marker(&self) -> bool {
        self.metadata.flag(&definitions::armor_stand::is_marker())
    }

    pub fn set_armor_stand_marker(&mut self, marker: bool) {
        self.metadata
            .set_flag(&definitions::armor_stand::is_marker(), marker);
    }

    pub fn armor_stand_head_rotation(&self) -> Vector3f {
        self.rotation_vector(&definitions::armor_stand::head_rotation())
    }

    pub fn set_armor_stand_head_rotation(&mut self, rotation: Vector3f) {
        self.set_rotation_vector(&definitions::armor_stand::head_rotation(), rotation);
    }

    pub fn armor_stand_body_rotation(&self) -> Vector3f {
        self.rotation_vector(&definitions::armor_stand::body_rotation())
    }

    pub fn set_armor_stand_body_rotation(&mut self, rotation: Vector3f) {
        self.set_rotation_vector(&definitions::armor_stand::body_rotation(), rotation);
    }

    pub fn armor_stand_left_arm_rotation(&self) -> Vector3f {
        self.rotation_vector(&definitions::armor_stand::left_arm_rotation())
    }

    pub fn set_armor_stand_left_arm_rotation(&mut self, rotation: Vector3f) {
        self.set_rotation_vector(&definitions::armor_stand::left_arm_rotation(), rotation);
    }

    pub fn armor_stand_right_arm_rotation(&self) -> Vector3f {
        self.rotation_vector(&definitions::armor_stand::right_arm_rotation())
    }

    pub fn set_armor_stand_right_arm_rotation(&mut self, rotation: Vector3f) {
        self.set_rotation_vector(&definitions::armor_stand::right_arm_rotation(), rotation);
    }

    pub fn armor_stand_left_leg_rotation(&self) -> Vector3f {
        self.rotation_vector(&definitions::armor_stand::left_leg_rotation())
    }

    pub fn set_armor_stand_left_leg_rotation(&mut self, rotation: Vector3f) {
        self.set_rotation_vector(&definitions::armor_stand::left_leg_rotation(), rotation);
    }

    pub fn armor_stand_right_leg_rotation(&self) -> Vector3f {
        self.rotation_vector(&definitions::armor_stand::right_leg_rotation())
    }

    pub fn set_armor_stand_right_leg_rotation(&mut self, rotation: Vector3f) {
        self.set_rotation_vector(&definitions::armor_stand::right_leg_rotation(), rotation);
    }

    fn rotation_vector(
        &self,
        definition: &crate::entity::metadata::MetadataDefinition,
    ) -> Vector3f {
        match self.metadata.value(definition) {
            MetadataValue::Rotation(x, y, z) => Vector3f { x, y, z },
            _ => Vector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }

    fn set_rotation_vector(
        &mut self,
        definition: &crate::entity::metadata::MetadataDefinition,
        rotation: Vector3f,
    ) {
        self.metadata.set(
            definition,
            MetadataValue::Rotation(rotation.x, rotation.y, rotation.z),
        );
    }

    pub fn eye_height(&self) -> f64 {
        if self.pose() == 2 {
            return 0.2;
        }
        self.entity_type.eye_height()
    }

    pub fn equipment(&self, equipment_slot: EquipmentSlot) -> &ItemStack {
        self.living.equipment(equipment_slot)
    }

    pub fn set_equipment(&mut self, equipment_slot: EquipmentSlot, item_stack: ItemStack) {
        self.living.set_equipment(equipment_slot, item_stack);
    }

    pub const fn arrow_count(&self) -> i32 {
        self.living.arrow_count()
    }

    pub fn set_arrow_count(&mut self, arrow_count: i32) {
        self.living.set_arrow_count(arrow_count);
        self.metadata.set(
            &definitions::living_entity::number_of_arrows(),
            MetadataValue::VarInt(self.living.arrow_count()),
        );
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
        self.metadata.set(
            &definitions::living_entity::health(),
            MetadataValue::Float(self.living.health()),
        );
        if self.living.health() <= 0.0 {
            self.kill();
        }
    }

    pub const fn max_health(&self) -> f32 {
        self.living.max_health()
    }

    pub fn set_max_health(&mut self, max_health: f32) {
        self.living.set_max_health(max_health);
    }

    pub fn heal(&mut self) {
        self.set_health(self.max_health());
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
        self.velocity = Velocity(Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
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

    pub fn has_attributes(&self) -> bool {
        self.living.has_attributes()
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
        let living_strength = if self.entity_type.should_send_attributes() {
            strength * (1.0 - self.living.attribute_value(Attribute::KNOCKBACK_RESISTANCE) as f32)
        } else {
            strength
        };
        self.velocity = knockback_velocity(self.velocity, self.on_ground, living_strength, x, z);
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

    pub(crate) fn movement_tick(&mut self, world: &WorldSnapshot) -> Option<EntityMovement> {
        if self.on_ground {
            self.reset_gravity_tick_count();
        } else {
            self.increment_gravity_tick_count();
        }
        if self.vehicle.is_some() {
            return None;
        }
        let velocity_per_tick = Velocity(Vector3d {
            x: self.velocity.0.x / SERVER_TICKS_PER_SECOND,
            y: self.velocity.0.y / SERVER_TICKS_PER_SECOND,
            z: self.velocity.0.z / SERVER_TICKS_PER_SECOND,
        });
        let physics = simulate_movement(
            self.position,
            velocity_per_tick,
            self.bounding_box,
            world,
            self.aerodynamics,
            self.has_no_gravity(),
            self.has_physics,
            self.on_ground,
            false,
            self.previous_physics_result,
        );
        self.previous_physics_result = Some(physics);
        if !world.is_chunk_loaded(ChunkPosition::from(physics.new_position())) {
            return None;
        }
        self.velocity = Velocity(Vector3d {
            x: physics.new_velocity_per_tick().0.x * SERVER_TICKS_PER_SECOND,
            y: physics.new_velocity_per_tick().0.y * SERVER_TICKS_PER_SECOND,
            z: physics.new_velocity_per_tick().0.z * SERVER_TICKS_PER_SECOND,
        });
        self.on_ground = physics.is_on_ground();
        let position = physics.new_position();
        let position_changed = position.x() != self.position.x()
            || position.y() != self.position.y()
            || position.z() != self.position.z();
        if !position_changed {
            return None;
        }
        self.set_position(position);
        self.position_movement_since_synchronization(
            MovementSynchronizationTiming::BeforeEntityTick,
        )
    }

    pub(crate) fn position_movement_after_tick(&mut self) -> Option<EntityMovement> {
        self.position_movement_since_synchronization(MovementSynchronizationTiming::AfterEntityTick)
    }

    fn position_movement_since_synchronization(
        &mut self,
        timing: MovementSynchronizationTiming,
    ) -> Option<EntityMovement> {
        if self.position == self.synchronization.last_position() {
            return None;
        }
        let scheduled_synchronization_is_imminent = match timing {
            MovementSynchronizationTiming::BeforeEntityTick => {
                self.synchronization.is_due_by_next_tick(self.ticks)
            }
            MovementSynchronizationTiming::AfterEntityTick => {
                self.synchronization.is_due(self.ticks, false)
            }
        };
        let should_suppress_movement_packet =
            self.entity_type.synchronizes_position_only() || scheduled_synchronization_is_imminent;
        if should_suppress_movement_packet {
            return Some(EntityMovement::new(
                self.entity_id,
                self.position,
                None,
                None,
            ));
        }
        let last_synced_position = self.synchronization.last_position();
        let packet = EntityMovementPacket::between(
            self.entity_id,
            last_synced_position,
            self.position,
            self.on_ground,
        );
        let view_changed = self.position.yaw() != last_synced_position.yaw()
            || self.position.pitch() != last_synced_position.pitch();
        let head_look_packet = match (&packet, view_changed) {
            (EntityMovementPacket::Position(_), true) => Some(self.head_look_packet()),
            _ => None,
        };
        self.synchronization.record_position(self.position);
        Some(EntityMovement::new(
            self.entity_id,
            self.position,
            Some(packet),
            head_look_packet,
        ))
    }

    pub const fn synchronization_ticks(&self) -> u64 {
        self.synchronization.interval_ticks()
    }

    pub fn set_synchronization_ticks(&mut self, synchronization_ticks: u64) {
        self.synchronization
            .set_interval_ticks(synchronization_ticks);
    }

    pub fn synchronize_next_tick(&mut self) {
        self.synchronization.synchronize_next_tick();
    }

    pub(crate) fn synchronize_position_packet(
        &mut self,
    ) -> spinel_core::network::clientbound::play::entity_position_sync::EntityPositionSyncPacket
    {
        self.synchronization
            .synchronize(self.entity_id, self.ticks, self.position, self.on_ground)
    }

    pub(crate) fn scheduled_position_sync_packet(
        &mut self,
    ) -> Option<
        spinel_core::network::clientbound::play::entity_position_sync::EntityPositionSyncPacket,
    > {
        self.synchronization
            .is_due(self.ticks, self.vehicle.is_some())
            .then(|| self.synchronize_position_packet())
    }

    pub const fn is_on_ground(&self) -> bool {
        self.on_ground
    }

    pub fn set_on_ground(&mut self, on_ground: bool) {
        self.on_ground = on_ground;
    }

    pub const fn has_physics(&self) -> bool {
        self.has_physics
    }

    pub fn set_has_physics(&mut self, has_physics: bool) {
        self.has_physics = has_physics;
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
        Vector3d {
            x: self.position.x() + self.bounding_box.minimum_x(),
            y: self.position.y() + self.bounding_box.minimum_y(),
            z: self.position.z() + self.bounding_box.minimum_z(),
        }
    }

    pub fn relative_end(&self) -> Vector3d {
        Vector3d {
            x: self.position.x() + self.bounding_box.maximum_x(),
            y: self.position.y() + self.bounding_box.maximum_y(),
            z: self.position.z() + self.bounding_box.maximum_z(),
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
        self.aerodynamics = EntityAerodynamics::from_entity_type(entity_type);
        self.refresh_collision_rules();
    }

    fn refresh_collision_rules(&mut self) {
        let collision_rules = EntityCollisionRules::from_entity_type(self.entity_type);
        self.has_entity_collision = collision_rules.has_entity_collision();
        self.prevents_block_placement = collision_rules.prevents_block_placement();
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
            velocity: self.spawn_packet_velocity(),
            pitch: EntityAngle(self.position.pitch),
            yaw: EntityAngle(self.position.yaw),
            head_yaw: EntityAngle(self.position.head_yaw),
            data: self.spawn_packet_data(),
        }
    }

    fn spawn_packet_data(&self) -> i32 {
        match self.entity_type.path() {
            "falling_block" => self.falling_block_state,
            "fishing_bobber" => self.fishing_hook_owner_entity_id.map_or(0, EntityId::value),
            "item_frame" | "glow_item_frame" | "painting" | "leash_knot" => {
                self.hanging_direction()
            }
            _ => 0,
        }
    }

    fn spawn_packet_velocity(&self) -> Velocity {
        match self.entity_type.path() {
            "llama_spit" | "shulker_bullet" => self.protocol_velocity(),
            _ => Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
        }
    }

    pub fn metadata_packet(&self) -> SetEntityDataPacket {
        SetEntityDataPacket::new(self.entity_id.value(), self.metadata.entries())
    }

    pub fn equipment_packet(&self) -> SetEquipmentPacket {
        SetEquipmentPacket::new(
            self.entity_id.value(),
            self.living.visible_equipment_entries(),
        )
    }

    pub fn velocity_packet(&self) -> EntityVelocityPacket {
        EntityVelocityPacket {
            entity_id: self.entity_id.value(),
            velocity: self.protocol_velocity(),
        }
    }

    pub(crate) fn protocol_velocity(&self) -> Velocity {
        Velocity(Vector3d {
            x: self.velocity.0.x / SERVER_TICKS_PER_SECOND,
            y: self.velocity.0.y / SERVER_TICKS_PER_SECOND,
            z: self.velocity.0.z / SERVER_TICKS_PER_SECOND,
        })
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
        let protocol_velocity = self.protocol_velocity();
        EntityTeleportPacket {
            entity_id: self.entity_id.value(),
            position: self.position.as_vector(),
            delta: Vector3d {
                x: protocol_velocity.0.x,
                y: protocol_velocity.0.y,
                z: protocol_velocity.0.z,
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
        for packet in self.effect_packets() {
            packet.dispatch(client)?;
        }
        Ok(())
    }

    pub fn update_old_viewer(&self, client: &mut Client) -> io::Result<()> {
        self.leash
            .leashed_entities()
            .iter()
            .try_for_each(|entity_id| {
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

    pub fn from_entity_type(entity_type: EntityType) -> Self {
        Self::new(
            entity_type.horizontal_air_resistance(),
            entity_type.vertical_air_resistance(),
            entity_type.acceleration(),
        )
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

    pub const fn with_view(self, yaw: f32, pitch: f32) -> Self {
        Self {
            yaw,
            pitch,
            head_yaw: yaw,
            ..self
        }
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
        x: position.x + bounding_box.minimum_x(),
        y: position.y + bounding_box.minimum_y(),
        z: position.z + bounding_box.minimum_z(),
    }
}

fn box_end(position: Vector3d, bounding_box: EntityBoundingBox) -> Vector3d {
    Vector3d {
        x: position.x + bounding_box.maximum_x(),
        y: position.y + bounding_box.maximum_y(),
        z: position.z + bounding_box.maximum_z(),
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
