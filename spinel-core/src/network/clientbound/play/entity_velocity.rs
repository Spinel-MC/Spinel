use spinel_macros::packet;
use spinel_network::types::{VarInt, Velocity};

#[packet(id: "set_entity_motion", state: ConnectionState::Play, recipient: Recipient::Client)]
#[derive(Clone)]
pub struct EntityVelocityPacket {
    pub entity_id: VarInt,
    pub velocity: Velocity,
}
