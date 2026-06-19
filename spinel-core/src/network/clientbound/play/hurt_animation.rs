use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "hurt_animation", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct HurtAnimationPacket {
    pub id: VarInt,
    pub yaw: f32,
}
