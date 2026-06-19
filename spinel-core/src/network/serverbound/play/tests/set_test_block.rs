use super::super::set_test_block::SetTestBlockPacket;
use spinel_network::DataType;
use spinel_network::types::{Position, TestBlockMode};

#[test]
fn set_test_block_packet_roundtrips_position_mode_and_message() {
    let packet = SetTestBlockPacket {
        position: Position {
            x: 12,
            y: 64,
            z: -3,
        },
        mode: TestBlockMode::Accept,
        message: "accepted".to_string(),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = SetTestBlockPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(
        decoded_packet.position,
        Position {
            x: 12,
            y: 64,
            z: -3
        }
    );
    assert_eq!(decoded_packet.mode, TestBlockMode::Accept);
    assert_eq!(decoded_packet.message, "accepted");
}
