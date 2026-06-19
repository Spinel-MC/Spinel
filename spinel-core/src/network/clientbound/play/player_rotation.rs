use spinel_macros::packet;

#[packet(id: "player_rotation", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct PlayerRotationPacket {
    pub y_rot: f32,
    pub relative_y: bool,
    pub x_rot: f32,
    pub relative_x: bool,
}
