use spinel_macros::packet;

#[packet(id: "move_player_status_only", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct MovePlayerStatusOnlyPacket {
    pub horizontal_collision: u8,
}
