use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "accept_teleportation", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct AcceptTeleportationPacket {
    pub id: VarInt,
}
