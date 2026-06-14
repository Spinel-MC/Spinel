use crate::network::clientbound::play::spawn_entity::EntityAngle;
use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "move_entity_pos_rot", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct EntityPositionAndRotationPacket {
    pub entity_id: VarInt,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub yaw: EntityAngle,
    pub pitch: EntityAngle,
    pub on_ground: bool,
}
