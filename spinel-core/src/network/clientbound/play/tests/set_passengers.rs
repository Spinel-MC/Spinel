use super::super::set_passengers::SetPassengersPacket;
use spinel_network::DataType;
use spinel_network::types::IntList;

#[test]
fn set_passengers_packet_matches_minestom_var_int_list_shape() {
    let packet = SetPassengersPacket {
        vehicle_entity_id: 7,
        passenger_entity_ids: IntList(vec![8, 9]),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();

    assert_eq!(SetPassengersPacket::get_id(), 0x69);
    assert_eq!(payload, vec![7, 2, 8, 9]);
}
