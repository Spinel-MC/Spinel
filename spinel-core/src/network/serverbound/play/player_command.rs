use spinel_macros::packet;

#[packet(id: "player_command", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct PlayerCommandPacket {
    pub entity_id: i32,
    pub action: i32,
    pub data: i32,
}

impl PlayerCommandPacket {
    pub const LEAVE_BED: i32 = 0;
    pub const START_SPRINTING: i32 = 1;
    pub const STOP_SPRINTING: i32 = 2;
    pub const START_JUMP_HORSE: i32 = 3;
    pub const STOP_JUMP_HORSE: i32 = 4;
    pub const OPEN_HORSE_INVENTORY: i32 = 5;
    pub const START_FLYING_ELYTRA: i32 = 6;
}

#[cfg(test)]
mod tests {
    use super::PlayerCommandPacket;
    use spinel_network::DataType;
    use spinel_network::types::var_int::VarIntWrapper;
    use std::io::{Cursor, Read};

    #[test]
    fn player_command_packet_decodes_minestom_client_entity_action_shape() {
        let mut payload = Vec::new();
        VarIntWrapper(7).encode(&mut payload).unwrap();
        VarIntWrapper(PlayerCommandPacket::START_SPRINTING)
            .encode(&mut payload)
            .unwrap();
        VarIntWrapper(0).encode(&mut payload).unwrap();

        let mut reader = Cursor::new(payload);
        let packet = PlayerCommandPacket::decode(&mut reader).unwrap();
        let mut remaining = Vec::new();
        reader.read_to_end(&mut remaining).unwrap();

        assert_eq!(PlayerCommandPacket::get_id(), 0x29);
        assert_eq!(packet.entity_id, 7);
        assert_eq!(packet.action, PlayerCommandPacket::START_SPRINTING);
        assert_eq!(packet.data, 0);
        assert!(remaining.is_empty());
    }
}
