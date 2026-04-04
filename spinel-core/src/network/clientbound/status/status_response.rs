use spinel_macros::packet;

#[packet(id: "status_response", state: ConnectionState::Status, recipient: Recipient::Client, generate_constructor: true)]
pub struct StatusResponsePacket {
    pub json_response: String,
}
