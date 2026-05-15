use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "set_health", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SetHealthPacket {
    pub health: f32,
    pub food: VarInt,
    pub saturation: f32,
}

impl SetHealthPacket {
    pub const fn new(health: f32, food: i32, saturation: f32) -> Self {
        Self {
            health,
            food,
            saturation,
        }
    }
}
