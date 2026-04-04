use spinel_macros::packet;
use spinel_network::types::game_profile::GameProfile;
use uuid::Uuid;

#[packet(id: "login_finished", state: ConnectionState::Login, recipient: Recipient::Client)]
pub struct LoginSuccessPacket {
    pub profile: GameProfile,
}

impl LoginSuccessPacket {
    pub fn new(uuid: Uuid, username: String) -> Self {
        Self {
            profile: GameProfile {
                uuid,
                username,
                properties: vec![],
            },
        }
    }
}
