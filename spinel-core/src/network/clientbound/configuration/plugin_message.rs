use spinel_macros::packet;
use spinel_network::RawBytes;

#[packet(id: "custom_payload", state: ConnectionState::Configuration, recipient: Recipient::Client)]
pub struct CustomPayloadPacket {
    pub channel: String,
    pub data: RawBytes,
}
