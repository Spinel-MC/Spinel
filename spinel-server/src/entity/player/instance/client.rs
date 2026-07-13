use crate::network::client::instance::Client;

use super::state::Player;

impl Player {
    pub(crate) fn set_client(&mut self, client: &mut Client) {
        client.set_player_entity_id(self.get_entity_id());
        self.client = Some(client as *mut Client as usize);
    }

    pub fn get_client(&self) -> Option<&Client> {
        self.client
            .map(|client| unsafe { &*(client as *const Client) })
    }

    pub(crate) fn get_client_mut(&mut self) -> Option<&mut Client> {
        self.client
            .map(|client| unsafe { &mut *(client as *mut Client) })
    }

    pub const fn get_last_keep_alive(&self) -> i64 {
        self.last_keep_alive
    }

    pub fn refresh_keep_alive(&mut self, last_keep_alive: i64) {
        self.last_keep_alive = last_keep_alive;
    }

    pub const fn get_did_answer_keep_alive(&self) -> bool {
        self.answer_keep_alive
    }

    pub fn refresh_answer_keep_alive(&mut self, answer_keep_alive: bool) {
        self.answer_keep_alive = answer_keep_alive;
    }
}
