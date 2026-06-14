use super::super::remove_entities::RemoveEntitiesPacket;
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
