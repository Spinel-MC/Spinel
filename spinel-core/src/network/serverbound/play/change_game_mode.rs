use crate::entity::game_mode::GameMode;
use spinel_macros::packet;

#[packet(id: "change_game_mode", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct ChangeGameModePacket {
    pub game_mode: GameMode,
}

#[cfg(test)]
mod tests {
    use super::ChangeGameModePacket;
    use crate::entity::game_mode::GameMode;
    use spinel_network::DataType;
    use std::io::{Cursor, Read};

    #[test]
    fn change_game_mode_decodes_minestom_byte_backed_game_mode() {
        let mut payload = Vec::new();
        GameMode::Spectator.encode(&mut payload).unwrap();

        let mut reader = Cursor::new(payload);
        let packet = ChangeGameModePacket::decode(&mut reader).unwrap();
        let mut remaining = Vec::new();
        reader.read_to_end(&mut remaining).unwrap();

        assert_eq!(ChangeGameModePacket::get_id(), 0x04);
        assert_eq!(packet.game_mode, GameMode::Spectator);
        assert!(remaining.is_empty());
    }

    #[test]
    fn change_game_mode_rejects_unknown_minestom_game_mode_id() {
        let mut reader = Cursor::new(vec![4]);

        assert!(ChangeGameModePacket::decode(&mut reader).is_err());
    }
}
