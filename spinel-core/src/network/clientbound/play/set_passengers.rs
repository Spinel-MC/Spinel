use spinel_macros::packet;
use spinel_network::types::{IntList, VarInt};

#[packet(id: "set_passengers", state: ConnectionState::Play, recipient: Recipient::Client)]
#[derive(Clone)]
pub struct SetPassengersPacket {
    pub vehicle_entity_id: VarInt,
    pub passenger_entity_ids: IntList,
}
