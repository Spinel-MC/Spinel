use spinel_macros::packet;
use spinel_network::types::Identifier;

#[packet(
    id: "select_advancements_tab",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
pub struct SelectAdvancementsTabPacket {
    pub tab: Option<Identifier>,
}
