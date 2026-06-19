use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(
    id: "set_command_minecart",
    state: ConnectionState::Play,
    recipient: Recipient::Server
)]
pub struct SetCommandMinecartPacket {
    pub entity_id: VarInt,
    pub command: String,
    pub track_output: bool,
}
