use spinel_macros::packet;
use spinel_network::types::var_int::VarInt;

#[packet(id: "use_item", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct UseItemPacket {
    pub hand: VarInt,
    pub sequence: VarInt,
    pub y_rot: f32,
    pub x_rot: f32,
}
