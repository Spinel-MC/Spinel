use crate::showcase::EntityShowcase;
use spinel::registry::Identifier;
use spinel::server::MinecraftServer;
use spinel::server::entity::{Entity, EntityPosition, Player, PlayerHand};
use spinel::server::world::{Block, BlockPosition, ChunkPosition};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

#[test]
fn entity_showcase_controls_minestom_and_vanilla_physics_zombies_together() {
    let mut server = MinecraftServer::new();
    let world_id = server
        .world_manager
        .create_world(Identifier::minecraft("pathfinding_showcase_test"));
    let world = server.world_manager.world_mut(world_id).unwrap();
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    for x in 0..=12 {
        for z in 0..=5 {
            world
                .set_block(BlockPosition::new(x, 64, z), Block::STONE)
                .unwrap();
        }
    }
    let controls = EntityShowcase::spawn(
        &mut server,
        world_id,
        EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0),
    )
    .unwrap();
    let world = server.world_manager.world_mut(world_id).unwrap();
    let mut starts = world
        .entities()
        .filter_map(|entity| match entity {
            Entity::Creature(creature) => Some(creature.position()),
            _ => None,
        })
        .collect::<Vec<_>>();
    starts.sort_by(|left, right| left.z().total_cmp(&right.z()));

    assert_eq!(starts.len(), 2);
    assert!(EntityShowcase::pathfind(
        world,
        controls.dual_pathfinding_stick(),
        BlockPosition::new(8, 64, 1),
    ));

    world.tick();

    let moved_creatures = world
        .entities()
        .filter_map(|entity| match entity {
            Entity::Creature(creature) => Some(creature),
            _ => None,
        })
        .filter(|creature| {
            starts.iter().all(|start| {
                creature.position().x() != start.x() || creature.position().z() != start.z()
            })
        })
        .count();
    assert_eq!(moved_creatures, 2);
}

#[test]
fn entity_showcase_controls_minestom_and_vanilla_physics_zombies_individually() {
    let mut server = MinecraftServer::new();
    let world_id = server.world_manager.create_world(Identifier::minecraft(
        "individual_pathfinding_showcase_test",
    ));
    let world = server.world_manager.world_mut(world_id).unwrap();
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    for x in 0..=12 {
        for z in 0..=5 {
            world
                .set_block(BlockPosition::new(x, 64, z), Block::STONE)
                .unwrap();
        }
    }
    let controls = EntityShowcase::spawn(
        &mut server,
        world_id,
        EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0),
    )
    .unwrap();
    let world = server.world_manager.world_mut(world_id).unwrap();
    let mut starts = world
        .entities()
        .filter_map(|entity| match entity {
            Entity::Creature(creature) => Some(creature.position()),
            _ => None,
        })
        .collect::<Vec<_>>();
    starts.sort_by(|left, right| left.z().total_cmp(&right.z()));
    let minestom_start = starts[0];
    let vanilla_start = starts[1];

    assert!(EntityShowcase::pathfind(
        world,
        controls.minestom_pathfinding_stick(),
        BlockPosition::new(8, 64, 0),
    ));
    world.tick();
    let positions_after_minestom_stick = creature_positions_by_z(world);
    assert!(positions_after_minestom_stick[0].x() != minestom_start.x());
    assert_eq!(positions_after_minestom_stick[1], vanilla_start);

    assert!(EntityShowcase::pathfind(
        world,
        controls.vanilla_pathfinding_stick(),
        BlockPosition::new(8, 64, 2),
    ));
    world.tick();
    let positions_after_vanilla_stick = creature_positions_by_z(world);
    assert!(positions_after_vanilla_stick[1].x() != vanilla_start.x());
}

#[test]
fn entity_showcase_dual_stick_survives_nonzero_held_slot() {
    let mut server = MinecraftServer::new();
    let world_id = server
        .world_manager
        .create_world(Identifier::minecraft("held_slot_pathfinding_showcase_test"));
    let world = server.world_manager.world_mut(world_id).unwrap();
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    for x in 0..=12 {
        for z in 0..=5 {
            world
                .set_block(BlockPosition::new(x, 64, z), Block::STONE)
                .unwrap();
        }
    }
    let controls = EntityShowcase::spawn(
        &mut server,
        world_id,
        EntityPosition::new(0.5, 65.0, 0.5, 0.0, 0.0),
    )
    .unwrap();
    let mut player = Player::new(
        Uuid::new_v4(),
        "HeldSlot".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
    );
    assert!(player.set_held_slot(1));
    controls.give_to_player(&mut player);
    let held_stick = player.item_in_hand(PlayerHand::Main);
    let world = server.world_manager.world_mut(world_id).unwrap();
    let starts = creature_positions_by_z(world);

    assert!(EntityShowcase::pathfind(
        world,
        &held_stick,
        BlockPosition::new(8, 64, 1),
    ));
    world.tick();
    let positions_after_pathfind = creature_positions_by_z(world);

    assert!(positions_after_pathfind[0].x() != starts[0].x());
    assert!(positions_after_pathfind[1].x() != starts[1].x());
}

fn creature_positions_by_z(world: &spinel::server::world::World) -> Vec<EntityPosition> {
    let mut positions = world
        .entities()
        .filter_map(|entity| match entity {
            Entity::Creature(creature) => Some(creature.position()),
            _ => None,
        })
        .collect::<Vec<_>>();
    positions.sort_by(|left, right| left.z().total_cmp(&right.z()));
    positions
}
