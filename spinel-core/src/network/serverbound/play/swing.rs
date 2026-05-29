use spinel_macros::packet;

#[packet(id: "swing", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct SwingPacket {
    pub hand: i32,
}

#[cfg(test)]
mod tests {
    use super::SwingPacket;
    use spinel_network::DataType;
    use spinel_network::types::var_int::VarIntWrapper;
    use std::io::{Cursor, Read};

    #[test]
    fn swing_packet_decodes_minestom_client_animation_shape() {
        let mut payload = Vec::new();
        VarIntWrapper(1).encode(&mut payload).unwrap();

        let mut reader = Cursor::new(payload);
        let packet = SwingPacket::decode(&mut reader).unwrap();
        let mut remaining = Vec::new();
        reader.read_to_end(&mut remaining).unwrap();

        assert_eq!(SwingPacket::get_id(), 0x3c);
        assert_eq!(packet.hand, 1);
        assert!(remaining.is_empty());
    }
}
