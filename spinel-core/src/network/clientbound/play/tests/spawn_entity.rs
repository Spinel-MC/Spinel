use super::super::spawn_entity::{EntityAngle, SpawnEntityPacket};
use spinel_network::DataType;
use uuid::Uuid;

#[test]
fn spawn_entity_packet_matches_minestom_field_order() {
    let packet = SpawnEntityPacket::new(7, Uuid::nil(), 148, 1.0, 2.0, 3.0);
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = SpawnEntityPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(SpawnEntityPacket::get_id(), 0x01);
    assert_eq!(payload.len(), 48);
    assert_eq!(decoded_packet.entity_id, 7);
    assert_eq!(decoded_packet.entity_type, 148);
    assert_eq!(decoded_packet.x, 1.0);
    assert_eq!(decoded_packet.y, 2.0);
    assert_eq!(decoded_packet.z, 3.0);
}

#[test]
fn entity_angle_uses_minestom_byte_conversion() {
    let mut payload = Vec::new();

    EntityAngle(90.0).encode(&mut payload).unwrap();

    assert_eq!(payload, vec![64]);
}

#[test]
fn entity_angle_wraps_unbounded_yaw_like_minestom_byte_cast() {
    let mut positive_payload = Vec::new();
    let mut negative_payload = Vec::new();

    EntityAngle(450.0).encode(&mut positive_payload).unwrap();
    EntityAngle(-90.0).encode(&mut negative_payload).unwrap();

    assert_eq!(positive_payload, vec![64]);
    assert_eq!(negative_payload, vec![192]);
}
