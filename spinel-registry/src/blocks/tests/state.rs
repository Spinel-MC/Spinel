use crate::{
    BlockFaceDirection, BlockState, block_entity_type::BlockEntityType, vanilla_world_blocks::Block,
};
use std::collections::HashSet;

#[test]
fn state_ids_cover_the_extracted_protocol_range() {
    assert!(BlockState::COUNT > Block::ALL.len());
    assert_eq!(BlockState::from_state_id(-1), None);
    assert_eq!(BlockState::from_state_id(BlockState::COUNT as i32), None);
    assert_eq!(
        BlockState::from_state_id(Block::STONE.state_id()),
        Some(Block::STONE.default_state())
    );
}

#[test]
fn state_metadata_comes_from_the_concrete_state() {
    assert_eq!(Block::TORCH.default_state().light_emission(), 14);
    assert_eq!(Block::STONE.default_state().light_block(), 15);
    assert!(Block::AIR.default_state().propagates_skylight_down());
    assert!(Block::AIR.default_state().occlusion_shape().is_empty());
    assert!(
        !Block::STONE
            .default_state()
            .face_occlusion_shape(BlockFaceDirection::Up)
            .is_empty()
    );
    assert!(!Block::STONE.default_state().collision_shape().is_empty());
    assert!(Block::AIR.default_state().collision_shape().is_empty());
    assert_eq!(Block::STONE.friction(), 0.6);
    assert_eq!(Block::ICE.friction(), 0.98);
}

#[test]
fn block_properties_select_another_state_of_the_same_block() {
    let north_stairs = Block::OAK_STAIRS
        .default_state()
        .with_property("facing", "north")
        .expect("oak stairs must expose a north-facing state");
    let east_stairs = north_stairs
        .with_property("facing", "east")
        .expect("oak stairs must expose an east-facing state");

    assert_eq!(north_stairs.block(), Block::OAK_STAIRS);
    assert_eq!(east_stairs.block(), Block::OAK_STAIRS);
    assert_eq!(north_stairs.property("facing"), Some("north"));
    assert_eq!(east_stairs.property("facing"), Some("east"));
    assert_ne!(north_stairs.state_id(), east_stairs.state_id());
    assert_eq!(east_stairs.with_property("missing", "value"), None);
}

#[test]
fn block_entity_types_come_from_extracted_block_metadata() {
    let represented_types = Block::ALL
        .iter()
        .filter_map(|block| block.block_entity_type())
        .collect::<HashSet<_>>();

    assert_eq!(represented_types.len(), BlockEntityType::ALL.len());
    assert!(
        BlockEntityType::ALL
            .into_iter()
            .all(|block_entity_type| represented_types.contains(&block_entity_type))
    );
    assert_eq!(
        Block::WHITE_BED.block_entity_type(),
        Some(BlockEntityType::Bed)
    );
    assert_eq!(
        Block::OAK_WALL_HANGING_SIGN.block_entity_type(),
        Some(BlockEntityType::HangingSign)
    );
    assert_eq!(
        Block::PLAYER_WALL_HEAD.block_entity_type(),
        Some(BlockEntityType::Skull)
    );
    assert_eq!(
        Block::WAXED_WEATHERED_COPPER_GOLEM_STATUE.block_entity_type(),
        Some(BlockEntityType::CopperGolemStatue)
    );
    assert_eq!(Block::STONE.block_entity_type(), None);
}
