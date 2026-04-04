use spinel_macros::packet;
use spinel_utils::component::text::TextComponent;

#[packet(id: "login_disconnect", state: ConnectionState::Login, recipient: Recipient::Client)]
pub struct LoginDisconnectPacket {
    pub reason: TextComponent,
}

impl LoginDisconnectPacket {
    pub fn new(reason: impl Into<TextComponent>) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}
