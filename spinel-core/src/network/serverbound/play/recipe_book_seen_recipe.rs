use spinel_macros::packet;
use spinel_network::types::RecipeDisplayId;

#[packet(
    id: "recipe_book_seen_recipe",
    state: ConnectionState::Play,
    recipient: Recipient::Server
)]
pub struct RecipeBookSeenRecipePacket {
    pub recipe: RecipeDisplayId,
}
