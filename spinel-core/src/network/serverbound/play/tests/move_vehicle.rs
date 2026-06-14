use super::super::move_vehicle::ServerboundVehicleMovePacket;
use spinel_network::DataType;

#[test]
fn move_vehicle_packet_matches_minestom_shape() {
    let packet = ServerboundVehicleMovePacket {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        yaw: 4.0,
        pitch: 5.0,
        on_ground: true,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = ServerboundVehicleMovePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(ServerboundVehicleMovePacket::get_id(), 0x21);
    assert_eq!(decoded_packet.x, 1.0);
    assert_eq!(decoded_packet.y, 2.0);
    assert_eq!(decoded_packet.z, 3.0);
    assert_eq!(decoded_packet.yaw, 4.0);
    assert_eq!(decoded_packet.pitch, 5.0);
    assert!(decoded_packet.on_ground);
}
