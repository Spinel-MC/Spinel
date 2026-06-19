use spinel_macros::packet;

#[packet(
    id: "set_border_center",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
pub struct SetBorderCenterPacket {
    pub new_center_x: f64,
    pub new_center_z: f64,
}
