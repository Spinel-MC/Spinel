use spinel_macros::packet;
use spinel_network::types::{TeleportFlags, VarInt};

pub struct SyncPlayerPositionSpec {
    pub teleport_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

#[packet(id: "player_position", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SyncPlayerPositionPacket {
    pub teleport_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: TeleportFlags,
}

impl SyncPlayerPositionPacket {
    pub fn new(sync_player_position_spec: SyncPlayerPositionSpec) -> Self {
        Self {
            teleport_id: sync_player_position_spec.teleport_id,
            x: sync_player_position_spec.x,
            y: sync_player_position_spec.y,
            z: sync_player_position_spec.z,
            velocity_x: 0.0,
            velocity_y: 0.0,
            velocity_z: 0.0,
            yaw: sync_player_position_spec.yaw,
            pitch: sync_player_position_spec.pitch,
            flags: TeleportFlags::absolute(),
        }
    }
}
