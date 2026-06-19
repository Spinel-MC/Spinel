use spinel_macros::packet;
use spinel_network::types::Position;

#[packet(id: "game_test_highlight_pos", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct GameTestHighlightPosPacket {
    pub absolute_pos: Position,
    pub relative_pos: Position,
}
