use super::super::steer_boat::SteerBoatPacket;
use spinel_network::DataType;

#[test]
fn steer_boat_packet_matches_minestom_shape() {
    let packet = SteerBoatPacket {
        left_paddle_turning: true,
        right_paddle_turning: false,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = SteerBoatPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(SteerBoatPacket::get_id(), 0x22);
    assert!(decoded_packet.left_paddle_turning);
    assert!(!decoded_packet.right_paddle_turning);
}
