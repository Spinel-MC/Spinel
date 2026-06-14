use spinel_macros::packet;

#[packet(id: "player_abilities", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct ServerboundPlayerAbilitiesPacket {
    pub flags: i8,
}

impl ServerboundPlayerAbilitiesPacket {
    pub const FLYING: i8 = 0x02;

    pub fn is_flying(&self) -> bool {
        self.flags & Self::FLYING != 0
    }
}
