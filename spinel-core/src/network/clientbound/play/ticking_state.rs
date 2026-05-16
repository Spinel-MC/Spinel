use spinel_macros::packet;

#[packet(id: "ticking_state", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct TickingStatePacket {
    pub tick_rate: f32,
    pub is_frozen: bool,
}
