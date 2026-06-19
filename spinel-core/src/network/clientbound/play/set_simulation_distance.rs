use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(
    id: "set_simulation_distance",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
pub struct SetSimulationDistancePacket {
    pub simulation_distance: VarInt,
}
