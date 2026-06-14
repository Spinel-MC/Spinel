use spinel_macros::packet;
use spinel_network::types::Array;
use uuid::Uuid;

#[packet(
    id: "player_info_remove",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
pub struct PlayerInfoRemovePacket {
    pub profile_ids: Array<Uuid>,
}

impl PlayerInfoRemovePacket {
    pub fn new(profile_id: Uuid) -> Self {
        Self {
            profile_ids: Array(vec![profile_id]),
        }
    }
}
