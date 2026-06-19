use super::super::section_blocks_update::{SectionBlockStateUpdate, SectionBlocksUpdatePacket};
use spinel_network::data_type::DataType;
use spinel_network::types::SectionPosition;

#[test]
fn section_blocks_update_packet_roundtrips() {
    let packet = SectionBlocksUpdatePacket {
        section_pos: SectionPosition { x: 4, y: 5, z: 6 },
        updates: vec![
            SectionBlockStateUpdate {
                position_in_section: 0x0123,
                block_state_id: 77,
            },
            SectionBlockStateUpdate {
                position_in_section: 0x0abc,
                block_state_id: 1234,
            },
        ],
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = SectionBlocksUpdatePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet, packet);
}
