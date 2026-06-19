use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(
    id: "container_slot_state_changed",
    state: ConnectionState::Play,
    recipient: Recipient::Server
)]
pub struct ContainerSlotStateChangedPacket {
    pub slot_id: VarInt,
    pub container_id: VarInt,
    pub new_state: bool,
}
