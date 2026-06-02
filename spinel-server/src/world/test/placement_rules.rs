use crate::entity::{Entity, EntityPosition, GenericEntity};
use crate::world::{BlockPosition, World, WorldBorder};
use spinel_network::types::Identifier;
use spinel_registry::EntityType;

#[test]
fn world_border_and_entity_collision_reject_block_placement_like_minestom() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    world
        .set_world_border(WorldBorder::new(4.0, 0.0, 0.0, 0, 0, 4).unwrap())
        .unwrap();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(0.5, 64.0, 0.5, 0.0, 0.0));

    assert!(world.block_position_is_inside_world_border(BlockPosition::new(1, 64, 1)));
    assert!(!world.block_position_is_inside_world_border(BlockPosition::new(3, 64, 0)));
    assert!(!world.block_position_has_placement_collision(BlockPosition::new(0, 64, 0)));

    world.add_entity(Entity::Generic(entity));

    assert!(world.block_position_has_placement_collision(BlockPosition::new(0, 64, 0)));
}
