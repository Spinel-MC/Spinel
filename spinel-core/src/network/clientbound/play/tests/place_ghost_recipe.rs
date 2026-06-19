use super::super::place_ghost_recipe::{PlaceGhostRecipePacket, RecipeDisplayPayload};
use spinel_network::data_type::DataType;

#[test]
fn place_ghost_recipe_packet_roundtrips() {
    let packet = PlaceGhostRecipePacket {
        container_id: 4,
        recipe_display: RecipeDisplayPayload {
            recipe_display_type_id: 2,
            encoded_recipe_display_data: vec![9, 8, 7],
        },
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = PlaceGhostRecipePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet, packet);
}
