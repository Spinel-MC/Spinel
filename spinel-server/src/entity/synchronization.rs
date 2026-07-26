use crate::entity::{EntityId, EntityPosition};
use spinel_core::network::clientbound::play::entity_position_sync::EntityPositionSyncPacket;
use spinel_network::types::{Vector3d, Velocity};

const DEFAULT_ENTITY_SYNCHRONIZATION_TICKS: u64 = 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntitySynchronizationMode {
    GenericSynchronization,
    VanillaSynchronization,
}

pub(crate) struct EntitySynchronization {
    last_position: EntityPosition,
    last_velocity: Velocity,
    interval_ticks: u64,
    next_tick: u64,
    mode: EntitySynchronizationMode,
}

impl EntitySynchronization {
    pub(crate) const fn new(initial_position: EntityPosition) -> Self {
        Self {
            last_position: initial_position,
            last_velocity: Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            interval_ticks: DEFAULT_ENTITY_SYNCHRONIZATION_TICKS,
            next_tick: DEFAULT_ENTITY_SYNCHRONIZATION_TICKS,
            mode: EntitySynchronizationMode::GenericSynchronization,
        }
    }

    pub(crate) const fn get_interval_ticks(&self) -> u64 {
        self.interval_ticks
    }

    pub(crate) fn set_interval_ticks(&mut self, interval_ticks: u64) {
        self.interval_ticks = interval_ticks;
    }

    pub(crate) const fn get_mode(&self) -> EntitySynchronizationMode {
        self.mode
    }

    pub(crate) fn set_mode(&mut self, mode: EntitySynchronizationMode) {
        self.mode = mode;
    }

    pub(crate) fn synchronize_next_tick(&mut self) {
        self.next_tick = 0;
    }

    pub(crate) const fn is_due(&self, current_tick: u64, has_vehicle: bool) -> bool {
        !has_vehicle && current_tick >= self.next_tick
    }

    pub(crate) const fn is_due_by_next_tick(&self, current_tick: u64) -> bool {
        self.next_tick <= current_tick.saturating_add(1)
    }

    pub(crate) const fn uses_scheduled_position_sync(&self) -> bool {
        matches!(self.mode, EntitySynchronizationMode::GenericSynchronization)
    }

    pub(crate) const fn uses_vanilla_movement_sync(&self) -> bool {
        matches!(self.mode, EntitySynchronizationMode::VanillaSynchronization)
    }

    pub(crate) const fn get_last_position(&self) -> EntityPosition {
        self.last_position
    }

    pub(crate) fn record_position(&mut self, position: EntityPosition) {
        self.last_position = position;
    }

    pub(crate) fn record_vanilla_movement_position(
        &mut self,
        current_tick: u64,
        position: EntityPosition,
    ) {
        self.last_position = position;
        self.next_tick = current_tick.saturating_add(self.interval_ticks);
    }

    pub(crate) fn record_vanilla_velocity_if_changed(&mut self, velocity: Velocity) -> bool {
        let current_velocity = velocity.0;
        let last_velocity = self.last_velocity.0;
        let difference_x = current_velocity.x - last_velocity.x;
        let difference_y = current_velocity.y - last_velocity.y;
        let difference_z = current_velocity.z - last_velocity.z;
        let distance_squared =
            difference_x * difference_x + difference_y * difference_y + difference_z * difference_z;
        let current_velocity_squared = current_velocity.x * current_velocity.x
            + current_velocity.y * current_velocity.y
            + current_velocity.z * current_velocity.z;
        let velocity_changed =
            distance_squared > 1.0e-7 || distance_squared > 0.0 && current_velocity_squared == 0.0;
        if velocity_changed {
            self.last_velocity = velocity;
        }
        velocity_changed
    }

    pub(crate) fn synchronize(
        &mut self,
        entity_id: EntityId,
        current_tick: u64,
        position: EntityPosition,
        velocity: Velocity,
        on_ground: bool,
    ) -> EntityPositionSyncPacket {
        self.last_position = position;
        self.last_velocity = velocity;
        self.next_tick = current_tick.saturating_add(self.interval_ticks);
        EntityPositionSyncPacket {
            entity_id: entity_id.get_value(),
            position: Vector3d {
                x: position.get_x(),
                y: position.get_y(),
                z: position.get_z(),
            },
            delta: Vector3d {
                x: velocity.0.x,
                y: velocity.0.y,
                z: velocity.0.z,
            },
            yaw: position.get_yaw(),
            pitch: position.get_pitch(),
            on_ground,
        }
    }
}
