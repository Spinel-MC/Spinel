use spinel_macros::packet;

#[packet(id: "change_difficulty", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ServerDifficultyPacket {
    pub difficulty: u8,
    pub locked: bool,
}

impl ServerDifficultyPacket {
    pub const PEACEFUL: u8 = 0;
    pub const EASY: u8 = 1;
    pub const NORMAL: u8 = 2;
    pub const HARD: u8 = 3;

    pub const fn normal(locked: bool) -> Self {
        Self {
            difficulty: Self::NORMAL,
            locked,
        }
    }
}
