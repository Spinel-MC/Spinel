use spinel_macros::packet;
use spinel_network::types::Position;

#[packet(id: "pick_item_from_block", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct PickItemFromBlockPacket {
    pub position: Position,
    pub include_data: bool,
}
