use crate as spinel;
use spinel_macros::packet_dispatcher;
use spinel_network::types::game_profile::GameProfile;
use uuid::Uuid;

#[packet_dispatcher(id: 0x02)]
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
