use spinel_macros::packet;

#[packet(id: "custom_payload", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct PlayCustomPayloadPacket {
    pub channel: String,
    pub data: Vec<u8>,
}

impl PlayCustomPayloadPacket {
    pub fn new(channel: impl Into<String>, data: Vec<u8>) -> Self {
        Self {
            channel: channel.into(),
            data,
        }
    }
}
