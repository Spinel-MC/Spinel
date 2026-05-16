use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "ticking_step", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct TickingStepPacket {
    pub tick_steps: VarInt,
}

impl TickingStepPacket {
    pub const fn new(tick_steps: i32) -> Self {
        Self { tick_steps }
    }
}
