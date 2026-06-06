use spinel_macros::packet;
use spinel_network::types::Position;

#[packet(id: "player_action", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct PlayerActionPacket {
    pub status: i32,
    pub block_position: Position,
    pub block_face: i8,
    pub sequence: i32,
}

impl PlayerActionPacket {
    pub const START_DESTROY_BLOCK: i32 = 0;
    pub const ABORT_DESTROY_BLOCK: i32 = 1;
    pub const STOP_DESTROY_BLOCK: i32 = 2;
    pub const DROP_ITEM_STACK: i32 = 3;
    pub const DROP_ITEM: i32 = 4;
    pub const RELEASE_USE_ITEM: i32 = 5;
    pub const SWAP_ITEM_HAND: i32 = 6;
}

#[cfg(test)]
mod tests {
    use super::PlayerActionPacket;
    use spinel_network::types::var_int::VarIntWrapper;
    use spinel_network::{DataType, Position};
    use std::io::Cursor;

    #[test]
    fn player_action_decodes_minestom_swap_item_hand_shape() {
        let mut payload = Vec::new();
        VarIntWrapper(6).encode(&mut payload).unwrap();
        Position { x: 0, y: 0, z: 0 }.encode(&mut payload).unwrap();
        0i8.encode(&mut payload).unwrap();
        VarIntWrapper(123).encode(&mut payload).unwrap();

        let packet = PlayerActionPacket::decode(&mut Cursor::new(payload)).unwrap();

        assert_eq!(packet.status, 6);
        assert_eq!(packet.block_position, Position { x: 0, y: 0, z: 0 });
        assert_eq!(packet.block_face, 0);
        assert_eq!(packet.sequence, 123);
    }
}
