use super::super::recipe_book_add::RecipeBookAddPacket;
use spinel_network::DataType;

#[test]
fn empty_recipe_book_reset_packet_matches_minestom_empty_recipe_manager_shape() {
    let mut payload = Vec::new();

    RecipeBookAddPacket::reset_empty()
        .encode(&mut payload)
        .unwrap();

    assert_eq!(RecipeBookAddPacket::get_id(), 0x48);
    assert_eq!(payload, vec![0, 1]);
}
