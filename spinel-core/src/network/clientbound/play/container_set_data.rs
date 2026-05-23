use spinel_macros::packet;
use spinel_network::VarInt;

#[packet(id: "container_set_data", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ContainerSetDataPacket {
    pub container_id: VarInt,
    pub id: i16,
    pub value: i16,
}
