use spinel_macros::packet;

#[packet(id: "open_book", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct OpenBookPacket {
    pub hand: i32,
}

impl OpenBookPacket {
    pub const fn new(hand: i32) -> Self {
        Self { hand }
    }
}

#[cfg(test)]
mod tests {
    use super::OpenBookPacket;
    use spinel_network::DataType;
    use spinel_network::types::var_int::VarIntWrapper;

    #[test]
    fn open_book_packet_matches_minestom_hand_enum_shape() {
        let packet = OpenBookPacket::new(1);
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();

        let mut expected_payload = Vec::new();
        VarIntWrapper(1).encode(&mut expected_payload).unwrap();

        assert_eq!(OpenBookPacket::get_id(), 0x38);
        assert_eq!(payload, expected_payload);
    }
}
