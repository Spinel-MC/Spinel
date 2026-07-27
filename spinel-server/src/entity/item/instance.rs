use crate::entity::item::metadata::ItemEntityMeta;
use crate::entity::metadata::definitions;
use crate::entity::physics::{EntityMovement, simulate_collision};
use crate::entity::{EntityPosition, GenericEntity};
use crate::world::{BlockPosition, ChunkPosition, WorldSnapshot};
use spinel_core::network::clientbound::play::spawn_entity::SpawnEntityPacket;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{Slot, Vector3d, Velocity};
use spinel_registry::{EntityType, ItemStack};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use uuid::Uuid;

const NO_MERGE_DELAY: u64 = u64::MAX;
const SERVER_TICKS_PER_SECOND: f64 = 20.0;
const SERVER_TICK_MILLIS: u64 = 50;
const VANILLA_ITEM_GRAVITY: f64 = 0.04;
const VANILLA_ITEM_HORIZONTAL_RESISTANCE: f64 = 0.98;
const VANILLA_ITEM_VERTICAL_RESISTANCE: f64 = 0.98;
const VANILLA_ITEM_GROUND_BOUNCE: f64 = -0.5;
const VANILLA_MOVING_MERGE_INTERVAL_TICKS: u64 = 2;
const VANILLA_STATIONARY_MERGE_INTERVAL_TICKS: u64 = 40;
const VANILLA_ITEM_LIFETIME_TICKS: u64 = 6000;
static MERGE_DELAY_TICKS: AtomicU64 = AtomicU64::new(10);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemEntityPhysics {
    GenericPhysics,
    VanillaPhysics,
}

pub struct ItemEntity {
    entity: GenericEntity,
    item_stack: ItemStack,
    is_pickable: bool,
    is_mergeable: bool,
    merge_range: f32,
    spawned_at: Instant,
    pickup_delay: Duration,
    last_merge_check_tick: u64,
    previous_on_ground: bool,
    physics: ItemEntityPhysics,
    target: Option<Uuid>,
}

impl ItemEntity {
    pub fn new(item_stack: ItemStack) -> Self {
        Self::with_uuid(item_stack, Uuid::new_v4())
    }

    pub fn with_uuid(item_stack: ItemStack, uuid: Uuid) -> Self {
        let mut item_entity = Self {
            entity: GenericEntity::with_uuid(EntityType::ITEM, uuid),
            item_stack: ItemStack::air(),
            is_pickable: true,
            is_mergeable: true,
            merge_range: 1.0,
            spawned_at: Instant::now(),
            pickup_delay: Duration::ZERO,
            last_merge_check_tick: 0,
            previous_on_ground: false,
            physics: ItemEntityPhysics::GenericPhysics,
            target: None,
        };
        item_entity.set_bounding_box_dimensions(0.25, 0.25, 0.25);
        item_entity.set_item_metadata(item_stack);
        item_entity
    }

    pub fn get_item_stack(&self) -> &ItemStack {
        &self.item_stack
    }

    pub fn get_entity_meta_mut(&mut self) -> ItemEntityMeta<'_> {
        ItemEntityMeta::new(self)
    }

    pub(crate) fn set_item_metadata(&mut self, item_stack: ItemStack) {
        self.entity.get_metadata_mut().set(
            &definitions::get_item_stack(),
            MetadataValue::Slot(Slot::from_item_stack(&item_stack)),
        );
        self.item_stack = item_stack;
    }

    pub fn spawn_packet(&self) -> SpawnEntityPacket {
        let mut packet = self.entity.spawn_packet();
        packet.data = 1;
        packet.velocity = self.entity.get_protocol_velocity();
        packet
    }

    pub(crate) fn movement_tick(&mut self, world: &WorldSnapshot) -> Option<EntityMovement> {
        let movement = match self.physics {
            ItemEntityPhysics::GenericPhysics => self.entity.movement_tick(world),
            ItemEntityPhysics::VanillaPhysics => self.vanilla_movement_tick(world),
        };
        let has_landed = !self.previous_on_ground && self.entity.is_on_ground();
        if has_landed {
            self.entity.synchronize_next_tick();
        }
        self.previous_on_ground = self.entity.is_on_ground();
        movement
    }

    fn vanilla_movement_tick(&mut self, world: &WorldSnapshot) -> Option<EntityMovement> {
        if self.get_vehicle().is_some() {
            return None;
        }
        let position_before_movement = self.get_position();
        let velocity_per_tick = vanilla_item_velocity_per_tick(self);
        let collision = simulate_collision(
            position_before_movement,
            Velocity(velocity_per_tick),
            self.get_bounding_box(),
            world,
            None,
        );
        let position_after_movement = collision.get_new_position();
        if !world.is_chunk_loaded(ChunkPosition::from(position_after_movement)) {
            return None;
        }
        let horizontal_resistance = vanilla_item_horizontal_resistance(
            world,
            position_after_movement,
            collision.is_on_ground(),
        );
        let moved_velocity_per_tick = collision.get_new_velocity_per_tick().0;
        let vertical_velocity = match collision.is_on_ground() && moved_velocity_per_tick.y < 0.0 {
            true => {
                moved_velocity_per_tick.y
                    * VANILLA_ITEM_VERTICAL_RESISTANCE
                    * VANILLA_ITEM_GROUND_BOUNCE
            }
            false => moved_velocity_per_tick.y * VANILLA_ITEM_VERTICAL_RESISTANCE,
        };
        self.set_velocity(Velocity(Vector3d {
            x: moved_velocity_per_tick.x * horizontal_resistance * SERVER_TICKS_PER_SECOND,
            y: vertical_velocity * SERVER_TICKS_PER_SECOND,
            z: moved_velocity_per_tick.z * horizontal_resistance * SERVER_TICKS_PER_SECOND,
        }));
        self.set_on_ground(collision.is_on_ground());
        if position_before_movement == position_after_movement {
            return None;
        }
        self.set_position(position_after_movement);
        self.position_movement_before_tick()
    }

    pub fn is_pickable(&self) -> bool {
        self.is_pickable && self.spawned_at.elapsed() >= self.pickup_delay
    }

    pub fn set_pickable(&mut self, is_pickable: bool) {
        self.is_pickable = is_pickable;
    }

    pub const fn has_pickup_enabled(&self) -> bool {
        self.is_pickable
    }

    pub const fn is_mergeable(&self) -> bool {
        self.is_mergeable
    }

    pub fn set_mergeable(&mut self, is_mergeable: bool) {
        self.is_mergeable = is_mergeable;
    }

    pub const fn get_merge_range(&self) -> f32 {
        self.merge_range
    }

    pub fn set_merge_range(&mut self, merge_range: f32) {
        self.merge_range = merge_range;
    }

    pub const fn get_pickup_delay(&self) -> Duration {
        self.pickup_delay
    }

    pub fn set_pickup_delay(&mut self, pickup_delay: Duration) {
        self.pickup_delay = pickup_delay;
    }

    pub const fn get_physics(&self) -> ItemEntityPhysics {
        self.physics
    }

    pub fn set_physics(&mut self, physics: ItemEntityPhysics) {
        self.physics = physics;
    }

    pub const fn get_target(&self) -> Option<Uuid> {
        self.target
    }

    pub fn set_target(&mut self, target: Option<Uuid>) {
        self.target = target;
    }

    pub fn get_time_since_spawn(&self) -> Duration {
        self.spawned_at.elapsed()
    }

    pub fn get_merge_delay() -> Option<Duration> {
        let merge_delay_ticks = MERGE_DELAY_TICKS.load(Ordering::Relaxed);
        (merge_delay_ticks != NO_MERGE_DELAY)
            .then(|| Duration::from_millis(merge_delay_ticks.saturating_mul(SERVER_TICK_MILLIS)))
    }

    pub fn set_merge_delay(merge_delay: Option<Duration>) {
        let merge_delay_ticks = merge_delay.map_or(NO_MERGE_DELAY, |merge_delay| {
            let merge_delay_millis = u64::try_from(merge_delay.as_millis()).unwrap_or(u64::MAX);
            merge_delay_millis.div_ceil(SERVER_TICK_MILLIS)
        });
        MERGE_DELAY_TICKS.store(merge_delay_ticks, Ordering::Relaxed);
    }

    pub(crate) fn should_check_merge(&mut self, current_tick: u64) -> bool {
        if self.physics == ItemEntityPhysics::VanillaPhysics {
            return self.should_check_vanilla_merge();
        }
        if !self.is_mergeable() || !self.is_pickable() {
            return false;
        }
        let merge_delay_ticks = MERGE_DELAY_TICKS.load(Ordering::Relaxed);
        if merge_delay_ticks != NO_MERGE_DELAY
            && current_tick.saturating_sub(self.last_merge_check_tick) < merge_delay_ticks
        {
            return false;
        }
        self.last_merge_check_tick = current_tick;
        true
    }

    fn should_check_vanilla_merge(&self) -> bool {
        if !self.is_vanilla_mergeable() {
            return false;
        }
        let merge_interval_ticks = match self.moved_between_block_positions() {
            true => VANILLA_MOVING_MERGE_INTERVAL_TICKS,
            false => VANILLA_STATIONARY_MERGE_INTERVAL_TICKS,
        };
        self.ticks() % merge_interval_ticks == 0
    }

    pub fn is_vanilla_mergeable(&self) -> bool {
        self.has_pickup_enabled()
            && self.is_mergeable()
            && self.ticks() < VANILLA_ITEM_LIFETIME_TICKS
            && self.item_stack.amount() < self.item_stack.max_stack_size()
    }

    fn moved_between_block_positions(&self) -> bool {
        let position = self.get_position();
        let previous_position = self.get_previous_position();
        position.get_x().floor() != previous_position.get_x().floor()
            || position.get_y().floor() != previous_position.get_y().floor()
            || position.get_z().floor() != previous_position.get_z().floor()
    }

    pub fn spawn(&mut self) {
        self.spawned_at = Instant::now();
    }
}

impl Deref for ItemEntity {
    type Target = GenericEntity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl DerefMut for ItemEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}

fn vanilla_item_velocity_per_tick(item_entity: &ItemEntity) -> Vector3d {
    let velocity = item_entity.get_velocity().0;
    Vector3d {
        x: velocity.x / SERVER_TICKS_PER_SECOND,
        y: vanilla_item_vertical_velocity_per_tick(item_entity),
        z: velocity.z / SERVER_TICKS_PER_SECOND,
    }
}

fn vanilla_item_vertical_velocity_per_tick(item_entity: &ItemEntity) -> f64 {
    let velocity_y_per_tick = item_entity.get_velocity().0.y / SERVER_TICKS_PER_SECOND;
    match item_entity.has_no_gravity() {
        true => velocity_y_per_tick,
        false => velocity_y_per_tick - VANILLA_ITEM_GRAVITY,
    }
}

fn vanilla_item_horizontal_resistance(
    world: &WorldSnapshot,
    position: EntityPosition,
    is_on_ground: bool,
) -> f64 {
    if !is_on_ground {
        return VANILLA_ITEM_HORIZONTAL_RESISTANCE;
    }
    f64::from(world.block(vanilla_item_block_below(position)).friction())
        * VANILLA_ITEM_HORIZONTAL_RESISTANCE
}

fn vanilla_item_block_below(position: EntityPosition) -> BlockPosition {
    BlockPosition::new(
        position.get_x().floor() as i32,
        (position.get_y() - 0.999999).floor() as i32,
        position.get_z().floor() as i32,
    )
}
