use spinel_macros::packet;
use spinel_network::types::VarInt;
use spinel_utils::component::text::TextComponent;

#[packet(id: "player_combat_kill", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct PlayerCombatKillPacket {
    pub player_id: VarInt,
    pub message: TextComponent,
}

impl PlayerCombatKillPacket {
    pub fn new(player_id: i32, message: impl Into<TextComponent>) -> Self {
        Self {
            player_id,
            message: message.into(),
        }
    }
}
