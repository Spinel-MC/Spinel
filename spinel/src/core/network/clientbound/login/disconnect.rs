use crate as spinel;
use spinel_macros::packet_dispatcher;
use spinel_utils::component::text::TextComponent;

#[packet_dispatcher(id: 0x00, state: ConnectionState::Login)]
pub struct LoginDisconnectPacket {
    reason: TextComponent,
}

impl LoginDisconnectPacket {
    pub fn new(reason: impl Into<TextComponent>) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}
