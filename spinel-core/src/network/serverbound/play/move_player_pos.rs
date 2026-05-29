use spinel_macros::packet;

#[packet(id: "move_player_pos", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct MovePlayerPosPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub flags: u8,
}

impl MovePlayerPosPacket {
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
    use super::MovePlayerPosPacket;

    #[test]
    fn move_player_position_decodes_minestom_status_flags() {
        let packet = MovePlayerPosPacket {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            flags: MovePlayerPosPacket::FLAG_ON_GROUND,
        };

        assert!(packet.on_ground());
        assert!(!packet.horizontal_collision());
    }
}
