use spinel_macros::packet;
use spinel_network::types::Position;

#[packet(id: "level_event", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct WorldEventPacket {
    pub effect_id: i32,
    pub position: Position,
    pub data: i32,
    pub global_event: bool,
}

impl WorldEventPacket {
    pub const fn new(effect_id: i32, position: Position, data: i32, global_event: bool) -> Self {
        Self {
            effect_id,
            position,
            data,
            global_event,
        }
    }
}
