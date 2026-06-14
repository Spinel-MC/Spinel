use spinel_macros::packet;

#[packet(id: 0x22, state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct SteerBoatPacket {
    pub left_paddle_turning: bool,
    pub right_paddle_turning: bool,
}
