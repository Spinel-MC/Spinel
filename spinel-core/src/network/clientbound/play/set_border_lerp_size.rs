use spinel_macros::packet;
use spinel_network::types::VarLong;

#[packet(
    id: "set_border_lerp_size",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
pub struct SetBorderLerpSizePacket {
    pub old_size: f64,
    pub new_size: f64,
    pub lerp_time: VarLong,
}
