use crate::{
    BlockPredicate, BlockPredicates, DataComponentPredicates, PropertiesPredicate, Registries,
    vanilla_world_blocks::Block,
};
use spinel_nbt::{Nbt, NbtCompound};

#[test]
fn block_predicates_match_concrete_block_state_properties() {
    let registries = Registries::new_vanilla();
    let predicates = BlockPredicates::new(vec![BlockPredicate::new(
        None,
        Some(PropertiesPredicate::exact(
            "axis".to_string(),
            "x".to_string(),
        )),
        None,
        DataComponentPredicates,
    )]);
    let x_axis_log = Block::OAK_LOG
        .default_state()
        .with_property("axis", "x")
        .unwrap();
    let y_axis_log = Block::OAK_LOG
        .default_state()
        .with_property("axis", "y")
        .unwrap();

    assert!(predicates.test_state(x_axis_log, &registries));
    assert!(!predicates.test_state(y_axis_log, &registries));
}

#[test]
fn block_predicates_match_exact_client_visible_nbt() {
    let registries = Registries::new_vanilla();
    let mut expected_nbt = NbtCompound::new();
    expected_nbt.insert("CustomName".to_string(), Nbt::String("Chest".to_string()));
    let predicates = BlockPredicates::new(vec![BlockPredicate::new(
        None,
        None,
        Some(expected_nbt.clone()),
        DataComponentPredicates,
    )]);
    let mut different_nbt = expected_nbt.clone();
    different_nbt.insert("Lock".to_string(), Nbt::String("Key".to_string()));

    assert!(predicates.test_state_with_nbt(
        Block::CHEST.default_state(),
        Some(&expected_nbt),
        &registries,
    ));
    assert!(!predicates.test_state(Block::CHEST.default_state(), &registries));
    assert!(!predicates.test_state_with_nbt(
        Block::CHEST.default_state(),
        Some(&different_nbt),
        &registries,
    ));
}
