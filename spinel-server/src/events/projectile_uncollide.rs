use crate::entity::EntityId;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct ProjectileUncollideEvent {
    projectile_id: EntityId,
}

impl ProjectileUncollideEvent {
    pub fn new(projectile_id: EntityId) -> Self {
        Self { projectile_id }
    }

    pub const fn get_projectile_id(&self) -> EntityId {
        self.projectile_id
    }
}
