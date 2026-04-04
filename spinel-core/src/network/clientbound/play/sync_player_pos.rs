use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "player_position", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SyncPlayerPositionPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
    pub teleport_id: VarInt,
}

impl SyncPlayerPositionPacket {
    pub fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32, teleport_id: i32) -> Self {
        Self {
            x,
            y,
            z,
            yaw,
            pitch,
            flags: 0,
            teleport_id,
        }
    }
}
