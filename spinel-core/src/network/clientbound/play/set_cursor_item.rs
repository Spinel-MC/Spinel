use spinel_macros::packet;
use spinel_network::types::Slot;

#[packet(id: "set_cursor_item", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SetCursorItemPacket {
    pub item: Slot,
}
