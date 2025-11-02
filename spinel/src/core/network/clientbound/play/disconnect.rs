use spinel_macros::packet_dispatcher;
use spinel_utils::component::text::TextComponent;
use crate as spinel;

#[packet_dispatcher(id: 0x1C, state: ConnectionState::Play)]
pub struct PlayDisconnectPacket {
        reason: TextComponent
}

impl PlayDisconnectPacket {
        pub fn new(reason: impl Into<TextComponent>) -> Self { 
                Self {
                        reason: reason.into()
                }
        }
}
