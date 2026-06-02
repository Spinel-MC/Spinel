use spinel_macros::packet;
use spinel_utils::component::text::TextComponent;

#[packet(id: "set_subtitle_text", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SetSubtitleTextPacket {
    pub subtitle: TextComponent,
}

impl SetSubtitleTextPacket {
    pub const fn new(subtitle: TextComponent) -> Self {
        Self { subtitle }
    }
}
