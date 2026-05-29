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

#[cfg(test)]
mod tests {
    use super::MovePlayerRotPacket;

    #[test]
    fn move_player_rotation_decodes_minestom_status_flags() {
        let packet = MovePlayerRotPacket {
            y_rot: 1.0,
            x_rot: 2.0,
            flags: 0,
        };

        assert!(!packet.on_ground());
        assert!(!packet.horizontal_collision());
    }
}
