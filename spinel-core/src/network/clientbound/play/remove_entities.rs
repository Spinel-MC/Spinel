use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "remove_entities", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct RemoveEntitiesPacket {
    pub entity_ids: Vec<VarInt>,
}

impl RemoveEntitiesPacket {
    pub fn new(entity_ids: Vec<i32>) -> Self {
        Self { entity_ids }
    }
}
