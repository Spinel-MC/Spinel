use spinel_macros::packet;
use spinel_utils::component::text::TextComponent;

#[packet(id: "disconnect", state: ConnectionState::Configuration, recipient: Recipient::Client)]
pub struct ConfigurationDisconnectPacket {
    pub reason: TextComponent,
}

impl ConfigurationDisconnectPacket {
    pub fn new(reason: impl Into<TextComponent>) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}
