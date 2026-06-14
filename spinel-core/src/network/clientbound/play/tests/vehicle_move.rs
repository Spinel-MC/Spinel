use super::super::vehicle_move::{VehicleMovePacket, VehiclePosition};
use spinel_network::DataType;

#[test]
fn vehicle_move_packet_matches_minestom_pos_shape() {
    let packet = VehicleMovePacket {
        position: VehiclePosition {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            yaw: 90.0,
            pitch: 45.0,
        },
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = VehicleMovePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(VehicleMovePacket::get_id(), 0x37);
    assert_eq!(decoded_packet.position, packet.position);
}
