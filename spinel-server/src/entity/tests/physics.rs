use crate::entity::generic_entity::EntityAerodynamics;
use crate::entity::physics::simulate_movement;
use crate::entity::{EntityPosition, GenericEntity, Player};
use crate::world::{Block, BlockPosition, ChunkPosition, World};
use spinel_network::types::{Identifier, Vector3d, Velocity};
use spinel_registry::EntityType;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

#[test]
fn generic_entity_movement_tick_applies_minestom_gravity_in_blocks_per_second() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let snapshot = world.update_snapshot();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));

    assert!(entity.movement_tick(&snapshot).is_none());

    assert_eq!(entity.get_position().get_y(), 64.0);
    assert!((entity.get_velocity().0.y - -1.568).abs() < 0.000001);
    assert_eq!(entity.get_gravity_tick_count(), 1);
}

#[test]
fn generic_entity_movement_tick_collides_with_extracted_block_collision_shape() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    world
        .set_block(BlockPosition::new(1, 63, 1), Block::STONE)
        .unwrap();
    let snapshot = world.update_snapshot();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(1.5, 64.0, 1.5, 0.0, 0.0));
    entity.set_velocity(Velocity(Vector3d {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    }));

    entity.movement_tick(&snapshot);

    assert_eq!(entity.get_position().get_y(), 64.0);
    assert!(entity.is_on_ground());
}

#[test]
fn physics_result_preserves_minestom_collision_evidence_and_cache_reuse() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    world
        .set_block(BlockPosition::new(1, 63, 1), Block::STONE)
        .unwrap();
    let snapshot = world.update_snapshot();
    let entity_type = EntityType::ZOMBIE;
    let position = EntityPosition::new(1.5, 64.0, 1.5, 0.0, 0.0);
    let velocity = Velocity(Vector3d {
        x: 0.0,
        y: -0.0784,
        z: 0.0,
    });
    let aerodynamics = EntityAerodynamics::new(
        entity_type.horizontal_air_resistance(),
        entity_type.vertical_air_resistance(),
        entity_type.acceleration(),
    );

    let first = simulate_movement(
        position,
        velocity,
        entity_type.get_bounding_box(),
        &snapshot,
        aerodynamics,
        false,
        true,
        false,
        false,
        None,
    );
    let second = simulate_movement(
        first.get_new_position(),
        velocity,
        entity_type.get_bounding_box(),
        &snapshot,
        aerodynamics,
        false,
        true,
        first.is_on_ground(),
        false,
        Some(first),
    );

    assert!(first.has_collision());
    assert!(first.get_collision_points()[1].is_some());
    assert_eq!(
        first.get_collision_shape_positions()[1],
        Some(BlockPosition::new(1, 63, 1))
    );
    assert!(!first.is_cached());
    assert!(second.is_cached());
    assert_eq!(second, first.as_cached());
}

#[test]
fn physics_matches_minestom_high_speed_wall_and_slab_fixtures() {
    let mut wall_world = World::new(Identifier::minecraft("overworld"));
    wall_world
        .set_block(BlockPosition::new(0, 43, 1), Block::STONE)
        .unwrap();
    let wall_result = simulate_zombie_movement(
        &wall_world.update_snapshot(),
        EntityPosition::new(0.0, 42.0, 0.0, 0.0, 0.0),
        Vector3d {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        },
        None,
    );

    let mut slab_world = World::new(Identifier::minecraft("overworld"));
    slab_world
        .set_block(BlockPosition::new(0, 42, 0), Block::STONE_SLAB)
        .unwrap();
    let slab_result = simulate_zombie_movement(
        &slab_world.update_snapshot(),
        EntityPosition::new(0.0, 44.0, 0.0, 0.0, 0.0),
        Vector3d {
            x: 0.0,
            y: -10.0,
            z: 0.0,
        },
        None,
    );

    assert_position_close(
        wall_result.get_new_position(),
        EntityPosition::new(0.0, 42.0, 0.7, 0.0, 0.0),
    );
    assert_position_close(
        slab_result.get_new_position(),
        EntityPosition::new(0.0, 42.5, 0.0, 0.0, 0.0),
    );
}

#[test]
fn physics_matches_minestom_tall_fence_and_diagonal_slide_fixtures() {
    let mut fence_world = World::new(Identifier::minecraft("overworld"));
    fence_world
        .set_block(BlockPosition::new(1, 42, 0), Block::OAK_FENCE)
        .unwrap();
    let fence_result = simulate_zombie_movement(
        &fence_world.update_snapshot(),
        EntityPosition::new(0.5, 43.25, 0.5, 0.0, 0.0),
        Vector3d {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        },
        None,
    );

    let mut diagonal_world = World::new(Identifier::minecraft("overworld"));
    diagonal_world
        .set_block(BlockPosition::new(1, 43, 1), Block::STONE)
        .unwrap();
    diagonal_world
        .set_block(BlockPosition::new(1, 43, 2), Block::STONE)
        .unwrap();
    let diagonal_result = simulate_zombie_movement(
        &diagonal_world.update_snapshot(),
        EntityPosition::new(0.69, 42.0, 0.69, 0.0, 0.0),
        Vector3d {
            x: 10.0,
            y: 0.0,
            z: 11.0,
        },
        None,
    );

    assert_position_close(
        fence_result.get_new_position(),
        EntityPosition::new(1.075, 43.25, 0.5, 0.0, 0.0),
    );
    assert_position_close(
        diagonal_result.get_new_position(),
        EntityPosition::new(0.7, 42.0, 11.69, 0.0, 0.0),
    );
}

#[test]
fn player_movement_tick_updates_velocity_without_overriding_client_position() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let snapshot = world.update_snapshot();
    let mut player = Player::new(
        Uuid::new_v4(),
        "PhysicsPlayer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));

    player.movement_tick(&snapshot);

    assert_eq!(
        player.get_position(),
        EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0)
    );
    assert!((player.get_velocity().0.y - -1.568).abs() < 0.000001);
    assert_eq!(player.get_gravity_tick_count(), 1);
}

fn simulate_zombie_movement(
    snapshot: &crate::world::WorldSnapshot,
    position: EntityPosition,
    velocity: Vector3d,
    previous: Option<crate::entity::physics::EntityPhysicsResult>,
) -> crate::entity::physics::EntityPhysicsResult {
    let entity_type = EntityType::ZOMBIE;
    simulate_movement(
        position,
        Velocity(velocity),
        entity_type.get_bounding_box(),
        snapshot,
        EntityAerodynamics::new(
            entity_type.horizontal_air_resistance(),
            entity_type.vertical_air_resistance(),
            entity_type.acceleration(),
        ),
        false,
        true,
        false,
        false,
        previous,
    )
}

fn assert_position_close(actual: EntityPosition, expected: EntityPosition) {
    assert!((actual.get_x() - expected.get_x()).abs() < 0.01);
    assert!((actual.get_y() - expected.get_y()).abs() < 0.01);
    assert!((actual.get_z() - expected.get_z()).abs() < 0.01);
}
