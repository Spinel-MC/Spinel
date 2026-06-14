use spinel_macros::packet;

#[packet(id: "swing", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct SwingPacket {
    pub hand: i32,
}
