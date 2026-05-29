use spinel_macros::packet;
use spinel_network::types::{IntList, VarInt};

#[packet(id: "set_passengers", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SetPassengersPacket {
    pub vehicle_entity_id: VarInt,
    pub passenger_entity_ids: IntList,
}

#[cfg(test)]
mod tests {
    use super::SetPassengersPacket;
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
}
