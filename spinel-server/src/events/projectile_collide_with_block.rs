use super::projectile_collide::ProjectileCollideEvent;
use crate::entity::{EntityId, EntityPosition};
use crate::world::Block;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct ProjectileCollideWithBlockEvent {
    collision: ProjectileCollideEvent,
    block: Block,
}

impl ProjectileCollideWithBlockEvent {
    pub fn new(projectile_id: EntityId, collision_position: EntityPosition, block: Block) -> Self {
        Self {
            collision: ProjectileCollideEvent::new(projectile_id, collision_position),
            block,
        }
    }

    pub const fn get_projectile_id(&self) -> EntityId {
        self.collision.get_projectile_id()
    }

    pub const fn get_collision_position(&self) -> EntityPosition {
        self.collision.get_collision_position()
    }

    pub const fn block(&self) -> Block {
        self.block
    }

    pub const fn is_cancelled(&self) -> bool {
        self.collision.is_cancelled()
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.collision.set_cancelled(cancelled);
    }

    pub(crate) fn get_collision_mut(&mut self) -> &mut ProjectileCollideEvent {
        &mut self.collision
    }
}
