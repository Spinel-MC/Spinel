use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(
    id: "set_border_warning_distance",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
pub struct SetBorderWarningDistancePacket {
    pub warning_blocks: VarInt,
}
