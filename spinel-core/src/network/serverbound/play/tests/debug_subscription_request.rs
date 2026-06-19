use super::super::debug_subscription_request::DebugSubscriptionRequestPacket;
use spinel_network::data_type::DataType;
use std::collections::BTreeSet;

#[test]
fn debug_subscription_request_uses_vanilla_varint_set_shape() {
    let packet = DebugSubscriptionRequestPacket {
        subscriptions: BTreeSet::from([2, 5, 8]),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = DebugSubscriptionRequestPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(DebugSubscriptionRequestPacket::get_id_const(), 0x16);
    assert_eq!(decoded_packet, packet);
    assert_eq!(payload, vec![3, 2, 5, 8]);
}
