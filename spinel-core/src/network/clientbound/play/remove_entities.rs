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

#[cfg(test)]
mod tests {
    use super::RemoveEntitiesPacket;
    use spinel_network::DataType;

    #[test]
    fn remove_entities_packet_matches_minestom_var_int_list_shape() {
        let packet = RemoveEntitiesPacket::new(vec![7, 8]);
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet = RemoveEntitiesPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(RemoveEntitiesPacket::get_id(), 0x4b);
        assert_eq!(decoded_packet.entity_ids, vec![7, 8]);
    }
}
