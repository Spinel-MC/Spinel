use spinel_macros::packet;

#[packet(id: "custom_payload", state: ConnectionState::Configuration, recipient: Recipient::Client)]
pub struct CustomPayloadPacket {
    pub channel: String,
    pub data: Vec<u8>,
}
