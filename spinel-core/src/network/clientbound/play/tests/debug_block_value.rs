use super::super::debug_block_value::DebugBlockValuePacket;
use super::super::debug_entity_value::DebugSubscriptionUpdate;
use spinel_network::data_type::DataType;
use spinel_network::types::Position;

#[test]
fn debug_block_value_packet_roundtrips() {
    let packet = DebugBlockValuePacket {
        block_pos: Position { x: 1, y: 2, z: 3 },
        update: DebugSubscriptionUpdate {
            subscription: 2,
            encoded_value: Some(vec![4, 5, 6]),
        },
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = DebugBlockValuePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet, packet);
}
