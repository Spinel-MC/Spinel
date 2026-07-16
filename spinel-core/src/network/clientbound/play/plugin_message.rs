use spinel_macros::packet;
use spinel_network::RawBytes;

#[packet(id: "custom_payload", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct PlayCustomPayloadPacket {
    pub channel: String,
    pub data: RawBytes,
}

impl PlayCustomPayloadPacket {
    pub fn new(channel: impl Into<String>, data: Vec<u8>) -> Self {
        Self {
            channel: channel.into(),
            data: RawBytes::from(data),
        }
    }
}