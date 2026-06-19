use spinel_macros::packet;
use spinel_network::types::RemoteChatSessionData;

#[packet(id: "chat_session_update", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct ChatSessionUpdatePacket {
    pub chat_session: RemoteChatSessionData,
}
