use crate::entity::metadata::definitions;
use crate::entity::{EntityPosition, GenericEntity};
use spinel_network::types::Slot;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::{EntityType, ItemStack};
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};

pub struct ItemEntity {
    entity: GenericEntity,
    item_stack: ItemStack,
    is_pickable: bool,
    is_mergeable: bool,
    merge_range: f32,
    spawned_at: Instant,
    pickup_delay: Duration,
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
        };
        item_entity.set_bounding_box_dimensions(0.25, 0.25, 0.25);
        item_entity.set_item_stack(item_stack);
        item_entity
    }

    pub fn item_stack(&self) -> &ItemStack {
        &self.item_stack
    }

    pub fn set_item_stack(&mut self, item_stack: ItemStack) {
        self.entity.metadata_mut().set(
            &definitions::item_stack(),
            MetadataValue::Slot(Slot::from_item_stack(&item_stack)),
        );
        self.item_stack = item_stack;
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

    pub const fn merge_range(&self) -> f32 {
        self.merge_range
    }

    pub fn set_merge_range(&mut self, merge_range: f32) {
        self.merge_range = merge_range;
    }

    pub const fn pickup_delay(&self) -> Duration {
        self.pickup_delay
    }

    pub fn set_pickup_delay(&mut self, pickup_delay: Duration) {
        self.pickup_delay = pickup_delay;
    }

    pub fn time_since_spawn(&self) -> Duration {
        self.spawned_at.elapsed()
    }

    pub fn spawn(&mut self, position: EntityPosition) {
        self.spawned_at = Instant::now();
        self.set_position(position);
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
