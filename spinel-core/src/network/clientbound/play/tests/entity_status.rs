use super::super::entity_status::EntityStatusPacket;
use spinel_network::DataType;

#[test]
fn entity_status_packet_matches_minestom_int_then_byte_shape() {
    let packet = EntityStatusPacket {
        entity_id: 7,
        status: 3,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();

    assert_eq!(EntityStatusPacket::get_id(), 0x22);
    assert_eq!(payload, vec![0, 0, 0, 7, 3]);
}
