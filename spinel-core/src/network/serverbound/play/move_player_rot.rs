use spinel_macros::packet;

#[packet(id: "move_player_rot", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct MovePlayerRotPacket {
    pub y_rot: f32,
    pub x_rot: f32,
    pub flags: u8,
}

impl MovePlayerRotPacket {
    pub const FLAG_ON_GROUND: u8 = 1;
    pub const FLAG_HORIZONTAL_COLLISION: u8 = 1 << 1;

    pub const fn on_ground(&self) -> bool {
        self.flags & Self::FLAG_ON_GROUND != 0
    }

    pub const fn horizontal_collision(&self) -> bool {
        self.flags & Self::FLAG_HORIZONTAL_COLLISION != 0
    }
}
