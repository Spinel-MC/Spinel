use crate::entity::{EntityId, EntityPosition};
use spinel_core::network::clientbound::play::entity_position_sync::EntityPositionSyncPacket;
use spinel_network::types::Vector3d;

const DEFAULT_ENTITY_SYNCHRONIZATION_TICKS: u64 = 20;

pub(crate) struct EntitySynchronization {
    last_position: EntityPosition,
    interval_ticks: u64,
    next_tick: u64,
}

impl EntitySynchronization {
    pub(crate) const fn new(initial_position: EntityPosition) -> Self {
        Self {
            last_position: initial_position,
            interval_ticks: DEFAULT_ENTITY_SYNCHRONIZATION_TICKS,
            next_tick: DEFAULT_ENTITY_SYNCHRONIZATION_TICKS,
        }
    }

    pub(crate) const fn get_interval_ticks(&self) -> u64 {
        self.interval_ticks
    }

    pub(crate) fn set_interval_ticks(&mut self, interval_ticks: u64) {
        self.interval_ticks = interval_ticks;
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

    pub(crate) const fn get_last_position(&self) -> EntityPosition {
        self.last_position
    }

    pub(crate) fn record_position(&mut self, position: EntityPosition) {
        self.last_position = position;
    }

    pub(crate) fn synchronize(
        &mut self,
        entity_id: EntityId,
        current_tick: u64,
        position: EntityPosition,
        on_ground: bool,
    ) -> EntityPositionSyncPacket {
        let previous_position = self.last_position;
        self.last_position = position;
        self.next_tick = current_tick.saturating_add(self.interval_ticks);
        EntityPositionSyncPacket {
            entity_id: entity_id.get_value(),
            position: Vector3d {
                x: position.get_x(),
                y: position.get_y(),
                z: position.get_z(),
            },
            delta: Vector3d {
                x: position.get_x() - previous_position.get_x(),
                y: position.get_y() - previous_position.get_y(),
                z: position.get_z() - previous_position.get_z(),
            },
            yaw: position.get_yaw(),
            pitch: position.get_pitch(),
            on_ground,
        }
    }
}
