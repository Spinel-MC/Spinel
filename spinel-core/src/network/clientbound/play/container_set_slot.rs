use spinel_macros::packet;
use spinel_network::VarInt;
use spinel_network::types::Slot;

#[packet(id: "container_set_slot", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ContainerSetSlotPacket {
    pub container_id: VarInt,
    pub state_id: VarInt,
    pub slot: i16,
    pub item: Slot,
}
