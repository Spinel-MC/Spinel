use super::super::update_attributes::{
    EntityAttribute, EntityAttributeModifier, UpdateAttributesPacket,
};
use spinel_network::DataType;

#[test]
fn update_attributes_uses_play_packet_id_and_round_trips_attack_speed() {
    let packet = UpdateAttributesPacket {
        entity_id: 41,
        attributes: vec![EntityAttribute::attack_speed(
            4.0,
            vec![EntityAttributeModifier::base_attack_speed(-2.8)],
        )],
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = UpdateAttributesPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(UpdateAttributesPacket::get_id(), 0x81);
    assert_eq!(decoded_packet.entity_id, packet.entity_id);
    assert_eq!(decoded_packet.attributes, packet.attributes);
}

#[test]
fn update_attributes_nested_var_int_fields_are_not_four_byte_integers() {
    let attribute =
        EntityAttribute::attack_speed(4.0, vec![EntityAttributeModifier::base_attack_speed(-2.8)]);
    let mut payload = Vec::new();

    attribute.encode(&mut payload).unwrap();

    assert_eq!(payload[0], 0x04);
    assert_eq!(payload[payload.len() - 1], 0x00);
}
