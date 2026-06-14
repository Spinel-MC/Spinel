use super::super::move_minecart::{MinecartLerpStep, MoveMinecartPacket};
use spinel_network::DataType;
use spinel_network::types::Vector3d;

#[test]
fn move_minecart_packet_matches_minestom_lerp_step_shape() {
    let packet = MoveMinecartPacket {
        entity_id: 7,
        lerp_steps: vec![MinecartLerpStep {
            position: Vector3d {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            velocity: Vector3d {
                x: 0.1,
                y: 0.2,
                z: 0.3,
            },
            yaw: 90.0,
            pitch: 45.0,
            weight: 1.0,
        }],
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = MoveMinecartPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(MoveMinecartPacket::get_id(), 0x35);
    assert_eq!(decoded_packet.entity_id, packet.entity_id);
    assert_eq!(decoded_packet.lerp_steps, packet.lerp_steps);
}
