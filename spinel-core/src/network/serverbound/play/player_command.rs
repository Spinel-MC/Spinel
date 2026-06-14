use spinel_macros::packet;

#[packet(id: "player_command", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct PlayerCommandPacket {
    pub entity_id: i32,
    pub action: i32,
    pub data: i32,
}

impl PlayerCommandPacket {
    pub const LEAVE_BED: i32 = 0;
    pub const START_SPRINTING: i32 = 1;
    pub const STOP_SPRINTING: i32 = 2;
    pub const START_JUMP_HORSE: i32 = 3;
    pub const STOP_JUMP_HORSE: i32 = 4;
    pub const OPEN_HORSE_INVENTORY: i32 = 5;
    pub const START_FLYING_ELYTRA: i32 = 6;
}
