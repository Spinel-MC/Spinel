use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "player_combat_end", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct PlayerCombatEndPacket {
    pub duration: VarInt,
}
