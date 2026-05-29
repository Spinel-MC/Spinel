use spinel_macros::packet;

#[packet(id: "move_player_status_only", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct MovePlayerStatusOnlyPacket {
    pub flags: u8,
}

impl MovePlayerStatusOnlyPacket {
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
    use super::MovePlayerStatusOnlyPacket;

    #[test]
    fn move_player_status_only_decodes_minestom_status_flags() {
        let packet = MovePlayerStatusOnlyPacket {
            flags: MovePlayerStatusOnlyPacket::FLAG_ON_GROUND
                | MovePlayerStatusOnlyPacket::FLAG_HORIZONTAL_COLLISION,
        };

        assert!(packet.on_ground());
        assert!(packet.horizontal_collision());
    }
}
