use spinel_macros::packet;
use spinel_utils::component::text::TextComponent;

#[packet(id: "system_chat", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SystemChatPacket {
    pub message: TextComponent,
    pub overlay: bool,
}

impl SystemChatPacket {
    pub fn new(message: impl Into<TextComponent>, overlay: bool) -> Self {
        Self {
            message: message.into(),
            overlay,
        }
    }
}
