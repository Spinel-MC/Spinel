use spinel_macros::packet;

#[packet(id: "entity_event", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct EntityStatusPacket {
    pub entity_id: i32,
    pub status: i8,
}

#[cfg(test)]
mod tests {
    use super::EntityStatusPacket;
    use spinel_network::DataType;

    #[test]
    fn entity_status_packet_matches_minestom_int_then_byte_shape() {
        let packet = EntityStatusPacket {
            entity_id: 7,
            status: 3,
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();

        assert_eq!(EntityStatusPacket::get_id(), 0x22);
        assert_eq!(payload, vec![0, 0, 0, 7, 3]);
    }
}
