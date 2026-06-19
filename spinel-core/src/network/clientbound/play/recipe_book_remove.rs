use spinel_macros::packet;
use spinel_network::types::RecipeDisplayId;

#[packet(
    id: "recipe_book_remove",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
pub struct RecipeBookRemovePacket {
    pub recipes: Vec<RecipeDisplayId>,
}
