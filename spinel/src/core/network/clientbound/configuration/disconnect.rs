use crate as spinel;
use spinel_macros::packet_dispatcher;
use spinel_utils::component::text::TextComponent;

#[packet_dispatcher(id: 0x02, state: ConnectionState::Configuration)]
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
