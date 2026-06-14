use super::super::set_entity_data::SetEntityDataPacket;
use spinel_network::DataType;
use spinel_network::types::entity_metadata::{MetadataEntry, MetadataValue};

#[test]
fn set_entity_data_packet_uses_minestom_terminator_shape() {
    let packet = SetEntityDataPacket::new(
        7,
        vec![MetadataEntry {
            index: 1,
            value: MetadataValue::VarInt(300),
        }],
    );
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = SetEntityDataPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(SetEntityDataPacket::get_id(), 0x61);
    assert_eq!(decoded_packet.entity_id, 7);
    assert_eq!(decoded_packet.entries.0, packet.entries.0);
    assert_eq!(payload.last().copied(), Some(0xFF));
}
