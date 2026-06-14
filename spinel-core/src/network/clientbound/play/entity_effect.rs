use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "update_mob_effect", state: ConnectionState::Play, recipient: Recipient::Client)]
#[derive(Clone)]
pub struct EntityEffectPacket {
    pub entity_id: VarInt,
    pub effect_id: VarInt,
    pub amplifier: VarInt,
    pub duration_ticks: VarInt,
    pub flags: i8,
}
