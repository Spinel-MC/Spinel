use spinel_macros::packet;

#[packet(id: "player_input", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct PlayerInputPacket {
    pub flags: i8,
}

impl PlayerInputPacket {
    const FORWARD: i8 = 0x01;
    const BACKWARD: i8 = 0x02;
    const LEFT: i8 = 0x04;
    const RIGHT: i8 = 0x08;
    const JUMP: i8 = 0x10;
    const SHIFT: i8 = 0x20;
    const SPRINT: i8 = 0x40;

    pub fn forward(&self) -> bool {
        self.flags & Self::FORWARD != 0
    }

    pub fn backward(&self) -> bool {
        self.flags & Self::BACKWARD != 0
    }

    pub fn left(&self) -> bool {
        self.flags & Self::LEFT != 0
    }

    pub fn right(&self) -> bool {
        self.flags & Self::RIGHT != 0
    }

    pub fn jump(&self) -> bool {
        self.flags & Self::JUMP != 0
    }

    pub fn shift(&self) -> bool {
        self.flags & Self::SHIFT != 0
    }

    pub fn sprint(&self) -> bool {
        self.flags & Self::SPRINT != 0
    }
}
