use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(
    id: "set_border_warning_delay",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
pub struct SetBorderWarningDelayPacket {
    pub warning_delay: VarInt,
}
