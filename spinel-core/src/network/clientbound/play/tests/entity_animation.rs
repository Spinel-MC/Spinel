use super::super::entity_animation::{EntityAnimation, EntityAnimationPacket};
use spinel_network::DataType;

#[test]
fn entity_animation_packet_matches_minestom_var_int_then_byte_shape() {
    let packet = EntityAnimationPacket {
        entity_id: 7,
        animation: EntityAnimation::SwingOffHand,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();

    assert_eq!(EntityAnimationPacket::get_id(), 0x02);
    assert_eq!(payload, vec![7, 3]);
}
