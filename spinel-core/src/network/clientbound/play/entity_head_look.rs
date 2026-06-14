use crate::network::clientbound::play::spawn_entity::EntityAngle;
use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "rotate_head", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct EntityHeadLookPacket {
    pub entity_id: VarInt,
    pub head_yaw: EntityAngle,
}
