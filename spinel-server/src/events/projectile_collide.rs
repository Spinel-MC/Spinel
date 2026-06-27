use crate::entity::{EntityId, EntityPosition};
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct ProjectileCollideEvent {
    projectile_id: EntityId,
    collision_position: EntityPosition,
    cancelled: bool,
}

impl ProjectileCollideEvent {
    pub fn new(projectile_id: EntityId, collision_position: EntityPosition) -> Self {
        Self {
            projectile_id,
            collision_position,
            cancelled: false,
        }
    }

    pub const fn get_projectile_id(&self) -> EntityId {
        self.projectile_id
    }

    pub const fn get_collision_position(&self) -> EntityPosition {
        self.collision_position
    }

    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}
