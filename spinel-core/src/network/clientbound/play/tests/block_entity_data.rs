use super::super::block_entity_data::BlockEntityDataPacket;
use spinel_network::DataType;
use spinel_network::types::Position;
use spinel_registry::block_entity_type::BlockEntityType;

#[test]
fn block_entity_data_packet_matches_minestom_position_type_and_nbt_shape() {
    let packet =
        BlockEntityDataPacket::new(Position { x: 1, y: 2, z: 3 }, BlockEntityType::Chest, None);
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = BlockEntityDataPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(BlockEntityDataPacket::get_id(), 0x06);
    assert_eq!(decoded_packet.block_position, packet.block_position);
    assert_eq!(decoded_packet.block_entity_type, packet.block_entity_type);
    assert_eq!(decoded_packet.data, packet.data);
    assert_eq!(payload.last().copied(), Some(0));
}
