use spinel_macros::packet;

#[packet(id: "open_book", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct OpenBookPacket {
    pub hand: i32,
}

impl OpenBookPacket {
    pub const fn new(hand: i32) -> Self {
        Self { hand }
    }
}
