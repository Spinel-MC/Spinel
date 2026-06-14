use super::super::update_recipes::UpdateRecipesPacket;
use spinel_network::DataType;

#[test]
fn empty_update_recipes_packet_matches_minestom_empty_recipe_manager_shape() {
    let mut payload = Vec::new();

    UpdateRecipesPacket.encode(&mut payload).unwrap();

    assert_eq!(UpdateRecipesPacket::get_id(), 0x83);
    assert_eq!(payload, vec![0, 0]);
}
