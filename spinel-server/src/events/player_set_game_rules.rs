use crate::entity::Player;
use crate::world::GameRuleRequestEntry;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerSetGameRulesEvent {
    player: *mut Player,
    requested_rules: Vec<GameRuleRequestEntry>,
}

impl PlayerSetGameRulesEvent {
    pub fn new(player: *mut Player, requested_rules: Vec<GameRuleRequestEntry>) -> Self {
        Self {
            player,
            requested_rules,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn get_requested_rules(&self) -> &[GameRuleRequestEntry] {
        &self.requested_rules
    }
}
