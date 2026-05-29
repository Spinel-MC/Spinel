use crate::entity::{Entity, Player};
use crate::server::MinecraftServer;
use crate::world::{
    BlockPosition, Chunk, ChunkLoader, ChunkPosition, Weather, World, WorldManager,
};
use spinel_nbt::{Tag, TagReadable, TagWritable};
use spinel_network::types::Identifier;
use spinel_registry::Registries;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use uuid::Uuid;

#[test]
fn world_and_chunk_tags_match_minestom_tag_handler_surface() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut chunk = Chunk::new(ChunkPosition::new(0, 0));
    let world_score = Tag::<i32>::integer("world_score");
    let chunk_score = Tag::<i32>::integer("chunk_score");

    world.set_tag(&world_score, Some(7));
    chunk.set_tag(&chunk_score, Some(9));

    assert_eq!(world.get_tag(&world_score), Some(7));
    assert_eq!(chunk.get_tag(&chunk_score), Some(9));
    world.remove_tag(&world_score);
    chunk.remove_tag(&chunk_score);
    assert_eq!(world.get_tag(&world_score), None);
    assert_eq!(chunk.get_tag(&chunk_score), None);
}

#[test]
fn world_scheduler_runs_next_tick_before_time_and_tick_end_after_world_work() {
    let mut world = World::new(Identifier::minecraft("overworld"));

    world.schedule_next_tick(|world| {
        world.set_world_age(40).unwrap();
    });
    world.schedule_tick_end(|world| {
        world.set_time(200).unwrap();
    });
    world.tick();

    assert_eq!(world.world_age(), 41);
    assert_eq!(world.time(), 200);
    assert_eq!(world.scheduler().next_tick_callback_count(), 0);
    assert_eq!(world.scheduler().tick_end_callback_count(), 0);
}

#[test]
fn world_event_node_dispatches_instance_scoped_callbacks() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let event_count = Arc::new(AtomicUsize::new(0));
    let listener_count = event_count.clone();

    world.event_node().listen("InstanceTickEvent", move |_| {
        listener_count.fetch_add(1, Ordering::SeqCst);
    });
    world.tick();

    assert_eq!(event_count.load(Ordering::SeqCst), 1);
    assert_eq!(world.event_node().listener_count("InstanceTickEvent"), 1);
}

#[test]
fn section_invalidation_marks_light_and_clears_chunk_packet_cache() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let registries = Registries::new_vanilla();
    let chunk_position = ChunkPosition::new(0, 0);

    world.load_chunk(chunk_position).unwrap();
    let chunk = world.chunk(chunk_position).unwrap();
    let cached_chunk_data = chunk.data(&registries).unwrap();

    assert!(!chunk.is_invalidated());
    assert!(world.invalidate_section(0, 0, 0));

    let chunk = world.chunk(chunk_position).unwrap();
    let section = chunk.section(0).unwrap();

    assert!(chunk.is_invalidated());
    assert!(section.sky_light_is_invalidated());
    assert!(section.block_light_is_invalidated());
    assert_ne!(
        cached_chunk_data.sections.as_ptr(),
        chunk.data(&registries).unwrap().sections.as_ptr()
    );
}

#[test]
fn world_light_reads_match_loaded_chunk_and_missing_chunk_edges() {
    let mut world = World::new(Identifier::minecraft("overworld"));

    assert_eq!(world.block_light(BlockPosition::new(0, 64, 0)), 0);
    assert_eq!(world.sky_light(BlockPosition::new(0, 64, 0)), 0);

    world.load_chunk(ChunkPosition::new(-1, -1)).unwrap();

    assert_eq!(world.block_light(BlockPosition::new(-1, 64, -1)), 0);
    assert_eq!(world.sky_light(BlockPosition::new(-1, 64, -1)), 15);
}

#[test]
fn block_action_requires_loaded_chunk_and_targets_chunk_viewers() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let position = BlockPosition::new(0, 64, 0);

    assert!(world.send_block_action(position, 1, 2).is_err());

    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();

    assert!(world.send_block_action(position, 1, 2).is_ok());
}

#[test]
fn world_weather_api_updates_weather_state() {
    let mut world = World::new(Identifier::minecraft("overworld"));

    assert_eq!(world.weather(), Weather::CLEAR);
    world.set_weather(Weather::new(0.75, 0.25)).unwrap();

    assert_eq!(world.weather().rain_level(), 0.75);
    assert_eq!(world.weather().thunder_level(), 0.25);
}

#[test]
fn chunk_world_membership_and_viewer_chunk_send_match_minestom_chunk_api() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let registries = Registries::new_vanilla();
    let chunk_position = ChunkPosition::new(0, 0);
    let player = Player::new(
        Uuid::nil(),
        "Viewer".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let player_uuid = player.uuid();
    let player_id = player.entity_id();

    world.load_chunk(chunk_position).unwrap();
    world.chunk(chunk_position).unwrap().world().unwrap();
    world
        .load_chunk(chunk_position)
        .unwrap()
        .add_viewer(player_id);
    world.add_entity(Entity::Player(player));
    world
        .send_chunk_to_viewers(chunk_position, &registries)
        .unwrap();

    let player = world.player_by_uuid(player_uuid).unwrap();

    assert_eq!(
        world.chunk(chunk_position).unwrap().world(),
        Some(world.uuid())
    );
    assert_eq!(player.queued_chunk_count(), 1);
}

#[test]
fn world_manager_unregister_world_unloads_chunks_and_removes_registration() {
    let unload_count = Arc::new(AtomicUsize::new(0));
    let mut worlds = WorldManager::new();
    let mut world = World::new(Identifier::minecraft("overworld"));
    world.set_chunk_loader(CountingUnloadLoader {
        unload_count: unload_count.clone(),
    });
    let world_uuid = world.uuid();

    worlds.register_world(world);
    worlds
        .world_mut(world_uuid)
        .unwrap()
        .load_chunk(ChunkPosition::new(0, 0))
        .unwrap();

    let unregistered_world = worlds.unregister_world(world_uuid).unwrap().unwrap();

    assert!(!unregistered_world.is_registered());
    assert!(worlds.world(world_uuid).is_none());
    assert_eq!(unload_count.load(Ordering::SeqCst), 1);
}

#[test]
fn world_manager_resolves_entity_world_handle_through_world_collection() {
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let entity = Entity::new(spinel_registry::EntityType::ZOMBIE);
    let entity_id = entity.entity_id();
    server.world_manager.add_entity(world_uuid, entity);
    let server_ptr = &mut server as *mut MinecraftServer as usize;

    let world_handle = server
        .world_manager
        .entity_world(server_ptr, entity_id)
        .unwrap();

    assert_eq!(world_handle.uuid(), world_uuid);
}

struct CountingUnloadLoader {
    unload_count: Arc<AtomicUsize>,
}

impl ChunkLoader for CountingUnloadLoader {
    fn load_chunk(&self, _position: ChunkPosition) -> io::Result<Option<Chunk>> {
        Ok(None)
    }

    fn save_chunk(&self, _chunk: &Chunk) -> io::Result<()> {
        Ok(())
    }

    fn unload_chunk(&self, chunk: &mut Chunk) -> io::Result<()> {
        chunk.unload();
        self.unload_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}
