use spinel_macros::packet;

#[packet(id: "move_player_pos", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct MovePlayerPosPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub horizontal_collision: u8,
}
