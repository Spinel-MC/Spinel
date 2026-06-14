use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "remove_mob_effect", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct RemoveEntityEffectPacket {
    pub entity_id: VarInt,
    pub effect_id: VarInt,
}
