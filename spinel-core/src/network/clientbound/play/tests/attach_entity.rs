use super::super::attach_entity::AttachEntityPacket;
use spinel_network::DataType;

#[test]
fn attach_entity_packet_matches_minestom_two_int_shape() {
    let packet = AttachEntityPacket {
        attached_entity_id: 7,
        holding_entity_id: -1,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();

    assert_eq!(AttachEntityPacket::get_id(), 0x62);
    assert_eq!(payload, vec![0, 0, 0, 7, 255, 255, 255, 255]);
}
