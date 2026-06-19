use super::super::debug_chunk_value::DebugChunkValuePacket;
use super::super::debug_entity_value::DebugSubscriptionUpdate;
use spinel_network::data_type::DataType;
use spinel_network::types::ChunkPos;

#[test]
fn debug_chunk_value_packet_roundtrips() {
    let packet = DebugChunkValuePacket {
        chunk_pos: ChunkPos { x: 7, z: 9 },
        update: DebugSubscriptionUpdate {
            subscription: 3,
            encoded_value: Some(vec![8, 9]),
        },
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = DebugChunkValuePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet, packet);
}
