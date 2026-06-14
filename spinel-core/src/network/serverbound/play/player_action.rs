use spinel_macros::packet;
use spinel_network::types::Position;
use spinel_network::types::var_int::VarInt;

#[packet(id: "player_action", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct PlayerActionPacket {
    pub status: VarInt,
    pub block_position: Position,
    pub block_face: i8,
    pub sequence: VarInt,
}

impl PlayerActionPacket {
    pub const START_DESTROY_BLOCK: i32 = 0;
    pub const ABORT_DESTROY_BLOCK: i32 = 1;
    pub const STOP_DESTROY_BLOCK: i32 = 2;
    pub const DROP_ITEM_STACK: i32 = 3;
    pub const DROP_ITEM: i32 = 4;
    pub const RELEASE_USE_ITEM: i32 = 5;
    pub const SWAP_ITEM_HAND: i32 = 6;
    pub const STAB: i32 = 7;
}
