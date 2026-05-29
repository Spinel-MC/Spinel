use spinel_macros::packet;

#[packet(id: "move_player_pos_rot", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct MovePlayerPosRotPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub y_rot: f32,
    pub x_rot: f32,
    pub flags: u8,
}

impl MovePlayerPosRotPacket {
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
    use super::MovePlayerPosRotPacket;

    #[test]
    fn move_player_position_and_rotation_decodes_minestom_status_flags() {
        let packet = MovePlayerPosRotPacket {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            y_rot: 4.0,
            x_rot: 5.0,
            flags: MovePlayerPosRotPacket::FLAG_HORIZONTAL_COLLISION,
        };

        assert!(!packet.on_ground());
        assert!(packet.horizontal_collision());
    }
}
