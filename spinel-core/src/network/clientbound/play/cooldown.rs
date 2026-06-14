use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "cooldown", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct CooldownPacket {
    pub cooldown_group: String,
    pub cooldown_ticks: VarInt,
}

impl CooldownPacket {
    pub fn new(cooldown_group: impl Into<String>, cooldown_ticks: i32) -> Self {
        Self {
            cooldown_group: cooldown_group.into(),
            cooldown_ticks,
        }
    }
}
