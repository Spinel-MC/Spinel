use crate::entity::game_mode::GameMode;
use spinel_macros::packet;

#[packet(id: "change_game_mode", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct ChangeGameModePacket {
    pub game_mode: GameMode,
}
