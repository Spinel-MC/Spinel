use spinel_macros::packet;
use spinel_network::types::RecipeBookSettings;

#[packet(
    id: "recipe_book_settings",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
pub struct RecipeBookSettingsPacket {
    pub book_settings: RecipeBookSettings,
}
