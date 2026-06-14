use super::super::player_look_at::{FacePoint, PlayerLookAtPacket};
use spinel_network::DataType;
use spinel_network::types::Vector3d;

#[test]
fn player_look_at_position_packet_matches_minestom_shape() {
    let packet = PlayerLookAtPacket::at_position(
        FacePoint::Eyes,
        Vector3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
    );
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = PlayerLookAtPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(PlayerLookAtPacket::get_id(), 0x45);
    assert_eq!(decoded_packet.look_at, packet.look_at);
    assert!(!payload[payload.len() - 1].eq(&1));
}

#[test]
fn player_look_at_entity_packet_matches_minestom_shape() {
    let packet = PlayerLookAtPacket::at_entity(
        FacePoint::Eyes,
        Vector3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        42,
        FacePoint::Eyes,
    );
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = PlayerLookAtPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet.look_at, packet.look_at);
}
