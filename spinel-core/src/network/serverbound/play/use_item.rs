use spinel_macros::packet;

#[packet(id: "use_item", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct UseItemPacket {
    pub hand: i32,
    pub sequence: i32,
    pub y_rot: f32,
    pub x_rot: f32,
}

#[cfg(test)]
mod tests {
    use super::UseItemPacket;
    use spinel_network::DataType;
    use spinel_network::types::var_int::VarIntWrapper;
    use std::io::{Cursor, Read};

    #[test]
    fn use_item_decodes_minestom_shape() {
        let mut payload = Vec::new();
        VarIntWrapper(0).encode(&mut payload).unwrap();
        VarIntWrapper(123).encode(&mut payload).unwrap();
        90.0f32.encode(&mut payload).unwrap();
        45.0f32.encode(&mut payload).unwrap();

        let mut reader = Cursor::new(payload);
        let packet = UseItemPacket::decode(&mut reader).unwrap();
        let mut remaining = Vec::new();
        reader.read_to_end(&mut remaining).unwrap();

        assert_eq!(packet.hand, 0);
        assert_eq!(packet.sequence, 123);
        assert_eq!(packet.y_rot, 90.0);
        assert_eq!(packet.x_rot, 45.0);
        assert!(remaining.is_empty());
    }
}
