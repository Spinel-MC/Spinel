use spinel_macros::packet;
use spinel_network::types::{RecipeDisplayId, VarInt};

#[packet(id: "place_recipe", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct PlaceRecipePacket {
    pub container_id: VarInt,
    pub recipe: RecipeDisplayId,
    pub use_max_items: bool,
}
