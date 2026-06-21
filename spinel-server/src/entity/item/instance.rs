use crate::entity::GenericEntity;
use crate::entity::item::metadata::ItemEntityMeta;
use crate::entity::metadata::definitions;
use crate::entity::physics::EntityMovement;
use crate::world::WorldSnapshot;
use spinel_core::network::clientbound::play::spawn_entity::SpawnEntityPacket;
use spinel_network::types::Slot;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::{EntityType, ItemStack};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

const NO_MERGE_DELAY: u64 = u64::MAX;
const SERVER_TICK_MILLIS: u64 = 50;
static MERGE_DELAY_TICKS: AtomicU64 = AtomicU64::new(10);

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
}

impl ItemEntity {
    pub fn new(item_stack: ItemStack) -> Self {
        let mut item_entity = Self {
            entity: GenericEntity::new(EntityType::ITEM),
            item_stack: ItemStack::air(),
            is_pickable: true,
            is_mergeable: true,
            merge_range: 1.0,
            spawned_at: Instant::now(),
            pickup_delay: Duration::ZERO,
            last_merge_check_tick: 0,
            previous_on_ground: false,
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
        let movement = self.entity.movement_tick(world);
        let has_landed = !self.previous_on_ground && self.entity.is_on_ground();
        if has_landed {
            self.entity.synchronize_next_tick();
        }
        self.previous_on_ground = self.entity.is_on_ground();
        movement
    }

    pub fn is_pickable(&self) -> bool {
        self.is_pickable && self.spawned_at.elapsed() >= self.pickup_delay
    }

    pub fn set_pickable(&mut self, is_pickable: bool) {
        self.is_pickable = is_pickable;
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
