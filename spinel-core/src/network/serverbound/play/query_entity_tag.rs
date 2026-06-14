use spinel_macros::packet;
use spinel_network::types::var_int::VarInt;

#[packet(id: "entity_tag_query", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct QueryEntityTagPacket {
    pub transaction_id: VarInt,
    pub entity_id: VarInt,
}
