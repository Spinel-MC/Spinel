use super::super::query_entity_tag::QueryEntityTagPacket;
use spinel_network::DataType;

#[test]
fn query_entity_tag_packet_matches_minestom_shape() {
    let mut payload = [0x07, 0x2a].as_slice();

    let packet = QueryEntityTagPacket::decode(&mut payload).unwrap();

    assert_eq!(QueryEntityTagPacket::get_id(), 0x18);
    assert_eq!(packet.transaction_id, 7);
    assert_eq!(packet.entity_id, 42);
    assert!(payload.is_empty());
}
