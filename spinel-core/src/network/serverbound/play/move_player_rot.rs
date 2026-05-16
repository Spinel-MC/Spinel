use spinel_macros::packet;

#[packet(id: "move_player_rot", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct MovePlayerRotPacket {
    pub y_rot: f32,
    pub x_rot: f32,
    pub horizontal_collision: u8,
}
