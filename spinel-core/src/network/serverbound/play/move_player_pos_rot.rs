use spinel_macros::packet;

#[packet(id: "move_player_pos_rot", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct MovePlayerPosRotPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub y_rot: f32,
    pub x_rot: f32,
    pub horizontal_collision: u8,
}
