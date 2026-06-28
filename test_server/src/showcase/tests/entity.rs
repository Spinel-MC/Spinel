use crate::showcase::EntityShowcase;
use spinel::registry::Identifier;
use spinel::server::MinecraftServer;
use spinel::server::entity::{Entity, EntityPosition, Player};
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
            Entity::Creature(creature) => Some(creature.get_position()),
            _ => None,
        })
        .collect::<Vec<_>>();
    starts.sort_by(|left, right| left.get_z().total_cmp(&right.get_z()));

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
                creature.get_position().get_x() != start.get_x()
                    || creature.get_position().get_z() != start.get_z()
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
            Entity::Creature(creature) => Some(creature.get_position()),
            _ => None,
        })
        .collect::<Vec<_>>();
    starts.sort_by(|left, right| left.get_z().total_cmp(&right.get_z()));
    let minestom_start = starts[0];
    let vanilla_start = starts[1];

    assert!(EntityShowcase::pathfind(
        world,
        controls.minestom_pathfinding_stick(),
        BlockPosition::new(8, 64, 0),
    ));
    world.tick();
    let positions_after_minestom_stick = creature_positions_by_z(world);
    assert!(positions_after_minestom_stick[0].get_x() != minestom_start.get_x());
    assert_eq!(positions_after_minestom_stick[1], vanilla_start);

    assert!(EntityShowcase::pathfind(
        world,
        controls.vanilla_pathfinding_stick(),
        BlockPosition::new(8, 64, 2),
    ));
    world.tick();
    let positions_after_vanilla_stick = creature_positions_by_z(world);
    assert!(positions_after_vanilla_stick[1].get_x() != vanilla_start.get_x());
}

#[test]
fn entity_showcase_pathfinding_sticks_are_added_without_replacing_occupied_slots() {
    let mut server = MinecraftServer::new();
    let world_id = server
        .world_manager
        .create_world(Identifier::minecraft("pathfinding_stick_inventory_test"));
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
    let dual_pathfinding_stick = controls.dual_pathfinding_stick().clone();
    let mut player = Player::new(
        Uuid::new_v4(),
        "InventoryInsertion".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
    );
    let diamond = spinel::registry::ItemStack::of(spinel::registry::Material::DIAMOND);
    let emerald = spinel::registry::ItemStack::of(spinel::registry::Material::EMERALD);
    assert!(player.get_inventory().set_item_stack(0, diamond.clone()));
    assert!(player.get_inventory().set_item_stack(1, emerald.clone()));

    assert_eq!(controls.give_to_player(&mut player), vec![true, true, true]);
    assert_eq!(player.get_inventory_ref().get_item_stack(0), Some(&diamond));
    assert_eq!(player.get_inventory_ref().get_item_stack(1), Some(&emerald));
    let stored_pathfinding_sticks = player
        .get_inventory_ref()
        .item_stacks()
        .iter()
        .filter(|item_stack| item_stack.material() == &spinel::registry::Material::STICK)
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(stored_pathfinding_sticks.len(), 3);
    assert!(stored_pathfinding_sticks.contains(&dual_pathfinding_stick));

    let world = server.world_manager.world_mut(world_id).unwrap();
    let starts = creature_positions_by_z(world);
    assert!(EntityShowcase::pathfind(
        world,
        &dual_pathfinding_stick,
        BlockPosition::new(8, 64, 1),
    ));
    world.tick();
    let positions_after_pathfind = creature_positions_by_z(world);

    assert!(positions_after_pathfind[0].get_x() != starts[0].get_x());
    assert!(positions_after_pathfind[1].get_x() != starts[1].get_x());
}
fn creature_positions_by_z(world: &spinel::server::world::World) -> Vec<EntityPosition> {
    let mut positions = world
        .entities()
        .filter_map(|entity| match entity {
            Entity::Creature(creature) => Some(creature.get_position()),
            _ => None,
        })
        .collect::<Vec<_>>();
    positions.sort_by(|left, right| left.get_z().total_cmp(&right.get_z()));
    positions
}
