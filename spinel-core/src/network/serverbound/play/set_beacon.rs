use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "set_beacon", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct SetBeaconPacket {
    pub primary_effect_id: Option<VarInt>,
    pub secondary_effect_id: Option<VarInt>,
}
