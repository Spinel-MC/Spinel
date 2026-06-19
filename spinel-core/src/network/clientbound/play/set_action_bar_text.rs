use spinel_macros::packet;
use spinel_utils::component::text::TextComponent;

#[packet(
    id: "set_action_bar_text",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
pub struct SetActionBarTextPacket {
    pub text: TextComponent,
}
