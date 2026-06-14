use spinel_macros::packet;
use spinel_network::types::{VarInt, Vector3d};

#[packet(id: "entity_position_sync", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct EntityPositionSyncPacket {
    pub entity_id: VarInt,
    pub position: Vector3d,
    pub delta: Vector3d,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
