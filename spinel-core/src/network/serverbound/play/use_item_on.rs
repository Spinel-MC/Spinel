use spinel_macros::packet;
use spinel_network::types::Position;

#[packet(id: "use_item_on", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct UseItemOnPacket {
    pub hand: i32,
    pub block_position: Position,
    pub block_face: i32,
    pub cursor_position_x: f32,
    pub cursor_position_y: f32,
    pub cursor_position_z: f32,
    pub inside_block: bool,
    pub hit_world_border: bool,
    pub sequence: i32,
}

#[cfg(test)]
mod tests {
    use super::UseItemOnPacket;
    use spinel_network::types::var_int::VarIntWrapper;
    use spinel_network::{DataType, Position};
    use std::io::{Cursor, Read};

    #[test]
    fn use_item_on_decodes_minestom_block_placement_shape() {
        let mut payload = Vec::new();
        VarIntWrapper(0).encode(&mut payload).unwrap();
        Position { x: 3, y: 39, z: 0 }.encode(&mut payload).unwrap();
        VarIntWrapper(1).encode(&mut payload).unwrap();
        0.5f32.encode(&mut payload).unwrap();
        0.25f32.encode(&mut payload).unwrap();
        0.75f32.encode(&mut payload).unwrap();
        false.encode(&mut payload).unwrap();
        true.encode(&mut payload).unwrap();
        VarIntWrapper(123).encode(&mut payload).unwrap();

        let mut reader = Cursor::new(payload);
        let packet = UseItemOnPacket::decode(&mut reader).unwrap();
        let mut remaining = Vec::new();
        reader.read_to_end(&mut remaining).unwrap();

        assert_eq!(packet.hand, 0);
        assert_eq!(packet.block_position, Position { x: 3, y: 39, z: 0 });
        assert_eq!(packet.block_face, 1);
        assert_eq!(packet.cursor_position_x, 0.5);
        assert_eq!(packet.cursor_position_y, 0.25);
        assert_eq!(packet.cursor_position_z, 0.75);
        assert!(!packet.inside_block);
        assert!(packet.hit_world_border);
        assert_eq!(packet.sequence, 123);
        assert!(remaining.is_empty());
    }
}
