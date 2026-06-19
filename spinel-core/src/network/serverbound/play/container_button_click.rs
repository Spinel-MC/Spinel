use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(
    id: "container_button_click",
    state: ConnectionState::Play,
    recipient: Recipient::Server
)]
pub struct ContainerButtonClickPacket {
    pub container_id: VarInt,
    pub button_id: VarInt,
}
