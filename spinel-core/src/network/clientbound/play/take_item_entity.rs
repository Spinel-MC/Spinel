use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "take_item_entity", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct TakeItemEntityPacket {
    pub collected_entity_id: VarInt,
    pub collector_entity_id: VarInt,
    pub pickup_item_count: VarInt,
}
