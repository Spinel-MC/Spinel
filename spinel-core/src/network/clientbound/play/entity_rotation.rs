use crate::network::clientbound::play::spawn_entity::EntityAngle;
use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "move_entity_rot", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct EntityRotationPacket {
    pub entity_id: VarInt,
    pub yaw: EntityAngle,
    pub pitch: EntityAngle,
    pub on_ground: bool,
}
