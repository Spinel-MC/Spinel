use spinel_macros::packet;
use spinel_utils::component::text::TextComponent;

#[packet(id: "disconnect", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct PlayDisconnectPacket {
    pub reason: TextComponent,
}

impl PlayDisconnectPacket {
    pub fn new(reason: impl Into<TextComponent>) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}
