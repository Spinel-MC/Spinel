use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "projectile_power", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ProjectilePowerPacket {
    pub entity_id: VarInt,
    pub acceleration_power: f64,
}
