use super::super::pick_item_from_entity::PickItemFromEntityPacket;
use spinel_network::DataType;

#[test]
fn pick_item_from_entity_packet_matches_minestom_shape() {
    let mut payload = [0x2a, 0x01].as_slice();

    let packet = PickItemFromEntityPacket::decode(&mut payload).unwrap();

    assert_eq!(PickItemFromEntityPacket::get_id(), 0x24);
    assert_eq!(packet.entity_id, 42);
    assert!(packet.include_data);
    assert!(payload.is_empty());
}
