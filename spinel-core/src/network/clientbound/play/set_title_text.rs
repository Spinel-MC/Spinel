use spinel_macros::packet;
use spinel_utils::component::text::TextComponent;

#[packet(id: "set_title_text", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SetTitleTextPacket {
    pub title: TextComponent,
}

impl SetTitleTextPacket {
    pub const fn new(title: TextComponent) -> Self {
        Self { title }
    }
}
