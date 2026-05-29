use spinel_macros::packet;
use spinel_network::types::{Position, VarInt};

#[packet(id: "block_update", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct BlockUpdatePacket {
    pub block_position: Position,
    pub block_state: VarInt,
}

impl BlockUpdatePacket {
    pub fn new(block_position: Position, block_state: i32) -> Self {
        Self {
            block_position,
            block_state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BlockUpdatePacket;
    use spinel_network::DataType;
    use spinel_network::types::Position;
    use spinel_network::types::var_int::VarIntWrapper;

    #[test]
    fn block_update_packet_matches_minestom_block_change_shape() {
        let packet = BlockUpdatePacket::new(Position { x: 1, y: 2, z: 3 }, 42);
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();

        assert_eq!(BlockUpdatePacket::get_id(), 0x08);
        assert_eq!(
            Position::decode(&mut payload.as_slice()).unwrap(),
            packet.block_position
        );
        assert_eq!(payload.len(), 9);
        assert_eq!(payload[8..], encoded_var_int(42));
    }

    fn encoded_var_int(value: i32) -> Vec<u8> {
        let mut payload = Vec::new();
        VarIntWrapper(value).encode(&mut payload).unwrap();
        payload
    }
}
