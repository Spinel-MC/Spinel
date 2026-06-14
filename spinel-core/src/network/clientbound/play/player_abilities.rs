use spinel_macros::packet;

#[packet(
    id: "player_abilities",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerAbilitiesPacket {
    pub flags: u8,
    pub flying_speed: f32,
    pub field_view_modifier: f32,
}

impl PlayerAbilitiesPacket {
    pub const INVULNERABLE: u8 = 0x01;
    pub const FLYING: u8 = 0x02;
    pub const ALLOW_FLYING: u8 = 0x04;
    pub const INSTANT_BREAK: u8 = 0x08;

    pub const fn new(flags: u8, flying_speed: f32, field_view_modifier: f32) -> Self {
        Self {
            flags,
            flying_speed,
            field_view_modifier,
        }
    }
}
