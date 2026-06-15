use crate::showcase::EntityShowcase;
use spinel::registry::Identifier;
use spinel::server::MinecraftServer;
use spinel::server::entity::{Entity, EntityPosition};
use spinel::server::world::{Block, BlockPosition, ChunkPosition};

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
    let controller = EntityShowcase::spawn(
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
        &controller,
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
