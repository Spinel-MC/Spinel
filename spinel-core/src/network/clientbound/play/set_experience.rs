use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "set_experience", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SetExperiencePacket {
    pub percentage: f32,
    pub level: VarInt,
    pub total_experience: VarInt,
}

impl SetExperiencePacket {
    pub const fn new(percentage: f32, level: i32, total_experience: i32) -> Self {
        Self {
            percentage,
            level,
            total_experience,
        }
    }
}
