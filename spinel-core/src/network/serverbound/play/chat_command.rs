use spinel_macros::packet;

#[packet(id: "chat_command", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct ChatCommandPacket {
    pub command: String,
}
