use spinel_macros::event_dispatcher;
use spinel_network::types::game_profile::GameProfile;
use std::io;
use uuid::Uuid;

#[event_dispatcher(with_client: true)]
pub struct AsyncPlayerPreLoginEvent {
    game_profile: GameProfile,
    pub cancelled: bool,
    pub should_authenticate: bool,
}

impl AsyncPlayerPreLoginEvent {
    pub fn new(name: String, uuid: Uuid, should_authenticate: bool) -> Self {
        Self {
            game_profile: GameProfile {
                uuid,
                username: name,
                properties: Vec::new(),
            },
            cancelled: false,
            should_authenticate,
            connection_ptr: None,
        }
    }

    pub fn game_profile(&self) -> &GameProfile {
        &self.game_profile
    }

    pub fn set_game_profile(&mut self, game_profile: GameProfile) {
        self.game_profile = game_profile;
    }

    pub fn username(&self) -> &str {
        &self.game_profile.username
    }

    pub fn set_username(&mut self, username: impl Into<String>) {
        self.game_profile.username = username.into();
    }

    pub const fn player_uuid(&self) -> Uuid {
        self.game_profile.uuid
    }

    pub fn into_game_profile(self) -> GameProfile {
        self.game_profile
    }

    pub fn send_plugin_request(
        &mut self,
        channel: impl Into<String>,
        request_payload: Option<Vec<u8>>,
    ) -> io::Result<crate::network::login::plugin_message::LoginPluginResponseFuture> {
        self.client()
            .send_login_plugin_request(channel, request_payload)
    }
}

pub type PreLoginEvent = AsyncPlayerPreLoginEvent;
