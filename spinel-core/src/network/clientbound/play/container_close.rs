use spinel_macros::packet;
use spinel_network::VarInt;

#[packet(id: "container_close", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ContainerClosePacket {
    pub container_id: VarInt,
}
