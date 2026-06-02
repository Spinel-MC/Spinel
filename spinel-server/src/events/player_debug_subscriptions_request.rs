use crate::entity::Player;
use spinel_macros::event_dispatcher;
use std::collections::BTreeSet;

#[event_dispatcher(with_client: true)]
pub struct PlayerDebugSubscriptionsRequestEvent {
    player: *mut Player,
    subscriptions: BTreeSet<i32>,
}

impl PlayerDebugSubscriptionsRequestEvent {
    pub fn new(player: *mut Player, subscriptions: BTreeSet<i32>) -> Self {
        Self {
            player,
            subscriptions,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn subscriptions(&self) -> &BTreeSet<i32> {
        &self.subscriptions
    }

    pub fn wants_subscriptions(&self) -> bool {
        !self.subscriptions.is_empty()
    }
}
