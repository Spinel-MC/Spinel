use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "transfer", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct TransferPacket {
    pub host: String,
    pub port: VarInt,
}
