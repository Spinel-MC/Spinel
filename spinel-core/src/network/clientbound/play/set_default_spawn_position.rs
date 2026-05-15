use spinel_macros::packet;
use spinel_network::types::GlobalPos;

#[packet(id: "set_default_spawn_position", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SetDefaultSpawnPositionPacket {
    pub global_pos: GlobalPos,
    pub yaw: f32,
    pub pitch: f32,
}

impl SetDefaultSpawnPositionPacket {
    pub const fn new(global_pos: GlobalPos, yaw: f32, pitch: f32) -> Self {
        Self {
            global_pos,
            yaw,
            pitch,
        }
    }
}
