use spinel_macros::packet;
use spinel_utils::component::text::TextComponent;

#[packet(id: "tab_list", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct TabListPacket {
    pub header: TextComponent,
    pub footer: TextComponent,
}

impl TabListPacket {
    pub fn new(header: TextComponent, footer: TextComponent) -> Self {
        Self { header, footer }
    }
}
