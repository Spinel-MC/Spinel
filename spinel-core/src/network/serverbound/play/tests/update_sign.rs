use super::super::update_sign::UpdateSignPacket;
use spinel_network::data_type::DataType;
use spinel_network::types::Position;

#[test]
fn update_sign_packet_matches_minestom_fixed_four_line_shape() {
    let packet = UpdateSignPacket {
        block_position: Position { x: 1, y: 2, z: 3 },
        is_front_text: true,
        lines: ["a".into(), "b".into(), "c".into(), "d".into()],
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = UpdateSignPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(UpdateSignPacket::get_id_const(), 0x3B);
    assert_eq!(decoded_packet, packet);
}
