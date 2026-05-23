use spinel_macros::packet;
use spinel_network::VarInt;
use spinel_network::types::{Array, Slot};

#[packet(id: "container_set_content", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ContainerSetContentPacket {
    pub container_id: VarInt,
    pub state_id: VarInt,
    pub items: Array<Slot>,
    pub carried_item: Slot,
}
