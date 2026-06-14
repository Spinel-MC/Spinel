use super::super::debug_entity_value::{DebugEntityValuePacket, DebugSubscriptionUpdate};
use spinel_network::{DataType, PacketStruct};

#[test]
fn debug_entity_value_packet_matches_minestom_update_envelope() {
    let packet = DebugEntityValuePacket {
        entity_id: 300,
        update: DebugSubscriptionUpdate {
            subscription: 5,
            encoded_value: Some(vec![0x01, 0x02]),
        },
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = DebugEntityValuePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(DebugEntityValuePacket::get_id(), 0x1C);
    assert_eq!(payload, vec![0xAC, 0x02, 0x05, 0x01, 0x01, 0x02]);
    assert_eq!(decoded_packet, packet);
}

#[test]
fn debug_entity_value_packet_encodes_removed_value_without_payload() {
    let packet = DebugEntityValuePacket {
        entity_id: 7,
        update: DebugSubscriptionUpdate {
            subscription: 3,
            encoded_value: None,
        },
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();

    assert_eq!(payload, vec![0x07, 0x03, 0x00]);
}
