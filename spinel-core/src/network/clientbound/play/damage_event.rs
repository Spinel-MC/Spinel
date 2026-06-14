use spinel_macros::packet;
use spinel_network::types::{Optional, VarInt, Vector3d};

#[packet(id: "damage_event", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct DamageEventPacket {
    pub target_entity_id: VarInt,
    pub damage_type_id: VarInt,
    pub source_entity_id: VarInt,
    pub source_direct_id: VarInt,
    pub source_position: Optional<Vector3d>,
}
