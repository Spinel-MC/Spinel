use spinel_macros::packet;

#[packet(
    id: "set_border_size",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
pub struct SetBorderSizePacket {
    pub size: f64,
}
