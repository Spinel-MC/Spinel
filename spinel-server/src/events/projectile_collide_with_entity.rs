use super::projectile_collide::ProjectileCollideEvent;
use crate::entity::{EntityId, EntityPosition};
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct ProjectileCollideWithEntityEvent {
    collision: ProjectileCollideEvent,
    target_id: EntityId,
}

impl ProjectileCollideWithEntityEvent {
    pub fn new(
        projectile_id: EntityId,
        collision_position: EntityPosition,
        target_id: EntityId,
    ) -> Self {
        Self {
            collision: ProjectileCollideEvent::new(projectile_id, collision_position),
            target_id,
        }
    }

    pub const fn projectile_id(&self) -> EntityId {
        self.collision.projectile_id()
    }

    pub const fn collision_position(&self) -> EntityPosition {
        self.collision.collision_position()
    }

    pub const fn target_id(&self) -> EntityId {
        self.target_id
    }

    pub const fn is_cancelled(&self) -> bool {
        self.collision.is_cancelled()
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.collision.set_cancelled(cancelled);
    }

    pub(crate) fn collision_mut(&mut self) -> &mut ProjectileCollideEvent {
        &mut self.collision
    }
}
