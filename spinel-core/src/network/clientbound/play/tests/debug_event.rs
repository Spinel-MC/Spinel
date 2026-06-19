use super::super::debug_event::{DebugEventPacket, DebugSubscriptionEvent};
use spinel_network::data_type::DataType;

#[test]
fn debug_event_packet_roundtrips() {
    let packet = DebugEventPacket {
        event: DebugSubscriptionEvent {
            subscription: 4,
            encoded_value: vec![1, 2, 3, 4],
        },
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = DebugEventPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet, packet);
}
