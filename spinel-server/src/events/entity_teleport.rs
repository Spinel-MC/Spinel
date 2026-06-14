use crate::entity::{Entity, EntityId, EntityPosition};
use spinel_macros::event_dispatcher;
use spinel_network::types::TeleportFlags;

#[event_dispatcher]
pub struct EntityTeleportEvent {
    entity: *mut Entity,
    teleport_position: EntityPosition,
    new_position: EntityPosition,
    relative_flags: TeleportFlags,
}

impl EntityTeleportEvent {
    pub fn new(
        entity: *mut Entity,
        teleport_position: EntityPosition,
        new_position: EntityPosition,
        relative_flags: TeleportFlags,
    ) -> Self {
        Self {
            entity,
            teleport_position,
            new_position,
            relative_flags,
        }
    }

    pub fn entity(&mut self) -> &mut Entity {
        unsafe { &mut *self.entity }
    }

    pub fn entity_id(&self) -> EntityId {
        unsafe { (&*self.entity).entity_id() }
    }

    pub fn teleport_position(&self) -> EntityPosition {
        self.teleport_position
    }

    pub fn new_position(&self) -> EntityPosition {
        self.new_position
    }

    pub fn relative_flags(&self) -> TeleportFlags {
        self.relative_flags
    }
}
