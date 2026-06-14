use spinel_macros::packet;
use spinel_network::types::var_int::VarInt;

#[packet(id: "pick_item_from_entity", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct PickItemFromEntityPacket {
    pub entity_id: VarInt,
    pub include_data: bool,
}
