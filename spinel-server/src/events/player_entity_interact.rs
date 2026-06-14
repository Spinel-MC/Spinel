use crate::entity::{Entity, EntityId, Player, PlayerHand};
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerEntityInteractEvent {
    player: *mut Player,
    target: *mut Entity,
    hand: PlayerHand,
    interact_position: (f32, f32, f32),
}

impl PlayerEntityInteractEvent {
    pub fn new(
        player: *mut Player,
        target: *mut Entity,
        hand: PlayerHand,
        interact_position: (f32, f32, f32),
    ) -> Self {
        Self {
            player,
            target,
            hand,
            interact_position,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn target_id(&self) -> EntityId {
        unsafe { (&*self.target).entity_id() }
    }

    pub fn target(&mut self) -> &mut Entity {
        unsafe { &mut *self.target }
    }

    pub fn hand(&self) -> PlayerHand {
        self.hand
    }

    pub fn interact_position(&self) -> (f32, f32, f32) {
        self.interact_position
    }
}
