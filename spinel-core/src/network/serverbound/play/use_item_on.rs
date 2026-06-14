use spinel_macros::packet;
use spinel_network::types::Position;
use spinel_network::types::var_int::VarInt;

#[packet(id: "use_item_on", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct UseItemOnPacket {
    pub hand: VarInt,
    pub block_position: Position,
    pub block_face: VarInt,
    pub cursor_position_x: f32,
    pub cursor_position_y: f32,
    pub cursor_position_z: f32,
    pub inside_block: bool,
    pub hit_world_border: bool,
    pub sequence: VarInt,
}
