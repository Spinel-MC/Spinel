use crate::entity::metadata::MetadataHolder;
use crate::entity::{EntityId, EntityLeash, EntityPosition, EntityView};
use spinel_network::types::{Vector3d, Velocity};
use spinel_registry::{EntityBoundingBox, EntityType};
use std::collections::BTreeSet;
use uuid::Uuid;

pub(crate) struct EntityState {
    entity_id: EntityId,
    uuid: Uuid,
    entity_type: EntityType,
    bounding_box: EntityBoundingBox,
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
    metadata: MetadataHolder,
    is_ordinary_on_fire: bool,
    has_visual_fire: bool,
    falling_block_state: i32,
    fishing_hook_owner_entity_id: Option<EntityId>,
}

impl EntityState {
    pub(crate) fn new(entity_id: EntityId, uuid: Uuid, entity_type: EntityType) -> Self {
        Self {
            entity_id,
            uuid,
            entity_type,
            bounding_box: entity_type.get_bounding_box(),
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
            metadata: MetadataHolder::default(),
            is_ordinary_on_fire: false,
            has_visual_fire: false,
            falling_block_state: spinel_registry::vanilla_world_blocks::Block::STONE.state_id(),
            fishing_hook_owner_entity_id: None,
        }
    }

    pub(crate) const fn get_entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub(crate) const fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub(crate) const fn get_entity_type(&self) -> EntityType {
        self.entity_type
    }

    pub(crate) fn set_entity_type(&mut self, entity_type: EntityType) {
        self.entity_type = entity_type;
    }

    pub(crate) const fn get_bounding_box(&self) -> EntityBoundingBox {
        self.bounding_box
    }

    pub(crate) fn set_bounding_box(&mut self, bounding_box: EntityBoundingBox) {
        self.bounding_box = bounding_box;
    }

    pub(crate) const fn get_view(&self) -> &EntityView {
        &self.view
    }

    pub(crate) const fn get_view_mut(&mut self) -> &mut EntityView {
        &mut self.view
    }

    pub(crate) const fn get_position(&self) -> EntityPosition {
        self.position
    }

    pub(crate) const fn get_previous_position(&self) -> EntityPosition {
        self.previous_position
    }

    pub(crate) fn set_position(&mut self, position: EntityPosition) {
        self.previous_position = self.position;
        self.position = position.clamped_to_entity_bounds();
    }

    pub(crate) const fn get_velocity(&self) -> Velocity {
        self.velocity
    }

    pub(crate) fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }

    pub(crate) const fn get_vehicle(&self) -> Option<EntityId> {
        self.vehicle
    }

    pub(crate) fn set_vehicle(&mut self, vehicle_id: EntityId) {
        self.vehicle = Some(vehicle_id);
    }

    pub(crate) fn clear_vehicle(&mut self) {
        self.vehicle = None;
    }

    pub(crate) fn get_passengers(&self) -> &BTreeSet<EntityId> {
        &self.passengers
    }

    pub(crate) fn add_passenger(&mut self, passenger_id: EntityId) -> bool {
        self.passengers.insert(passenger_id)
    }

    pub(crate) fn remove_passenger(&mut self, passenger_id: EntityId) -> bool {
        self.passengers.remove(&passenger_id)
    }

    pub(crate) fn get_leash(&self) -> &EntityLeash {
        &self.leash
    }

    pub(crate) fn get_leash_mut(&mut self) -> &mut EntityLeash {
        &mut self.leash
    }

    pub(crate) const fn get_world(&self) -> Option<Uuid> {
        self.world
    }

    pub(crate) fn set_world(&mut self, world: Option<Uuid>) {
        self.world = world;
    }

    pub(crate) const fn is_removed(&self) -> bool {
        self.removed
    }

    pub(crate) fn set_removed(&mut self, removed: bool) {
        self.removed = removed;
    }

    pub(crate) const fn get_ticks(&self) -> u64 {
        self.ticks
    }

    pub(crate) fn advance_tick(&mut self) {
        self.ticks += 1;
    }

    pub(crate) const fn get_falling_block_state(&self) -> i32 {
        self.falling_block_state
    }

    pub(crate) fn set_falling_block_state(&mut self, block_state: i32) {
        self.falling_block_state = block_state;
    }

    pub(crate) const fn get_fishing_hook_owner_entity_id(&self) -> Option<EntityId> {
        self.fishing_hook_owner_entity_id
    }

    pub(crate) fn set_fishing_hook_owner_entity_id(&mut self, owner_entity_id: Option<EntityId>) {
        self.fishing_hook_owner_entity_id = owner_entity_id;
    }

    pub(crate) const fn get_metadata(&self) -> &MetadataHolder {
        &self.metadata
    }

    pub(crate) fn get_metadata_mut(&mut self) -> &mut MetadataHolder {
        &mut self.metadata
    }

    pub(crate) const fn is_ordinary_on_fire(&self) -> bool {
        self.is_ordinary_on_fire
    }

    pub(crate) fn set_ordinary_on_fire(&mut self, is_ordinary_on_fire: bool) -> bool {
        if self.is_ordinary_on_fire == is_ordinary_on_fire {
            return false;
        }
        self.is_ordinary_on_fire = is_ordinary_on_fire;
        true
    }

    pub(crate) const fn has_visual_fire(&self) -> bool {
        self.has_visual_fire
    }

    pub(crate) fn set_visual_fire(&mut self, has_visual_fire: bool) -> bool {
        if self.has_visual_fire == has_visual_fire {
            return false;
        }
        self.has_visual_fire = has_visual_fire;
        true
    }
}
