use crate::entity::{Entity, EntityId, Player};
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerPickEntityEvent {
    player: *mut Player,
    target: Option<*mut Entity>,
    include_data: bool,
}

impl PlayerPickEntityEvent {
    pub fn new(player: *mut Player, target: Option<*mut Entity>, include_data: bool) -> Self {
        Self {
            player,
            target,
            include_data,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn target(&mut self) -> Option<&mut Entity> {
        self.target.map(|target| unsafe { &mut *target })
    }

    pub fn target_id(&self) -> Option<EntityId> {
        self.target.map(|target| unsafe { (&*target).entity_id() })
    }

    pub const fn include_data(&self) -> bool {
        self.include_data
    }
}
