use crate::entity::{Entity, EntityPosition, Player, PlayerSpawnPoint};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::{
    Block, BlockEntity, BlockHandler, BlockHandlerDestroy, BlockHandlerPlacement, BlockHandlerTick,
    BlockLookupCondition, BlockPlacementRule, BlockPlacementState, BlockPosition, BlockUpdateState,
    BossBar, Chunk, ChunkLoader, ChunkPosition, Explosion, Weather, World, WorldBorder,
    WorldBossBarColor, WorldBossBarOverlay, WorldManager, WorldSoundEmitter,
};
use spinel_core::network::clientbound::play::block_entity_data::BlockEntityDataPacket;
use spinel_core::network::clientbound::play::block_update::BlockUpdatePacket;
use spinel_core::network::clientbound::play::boss_bar::BossBarPacket;
use spinel_core::network::clientbound::play::entity_sound_effect::EntitySoundEffectPacket;
use spinel_core::network::clientbound::play::explosion::ExplosionPacket;
use spinel_core::network::clientbound::play::game_event::GameEventPacket;
use spinel_core::network::clientbound::play::initialize_world_border::InitializeWorldBorderPacket;
use spinel_core::network::clientbound::play::login_play::LoginPlayPacket;
use spinel_core::network::clientbound::play::player_info_remove::PlayerInfoRemovePacket;
use spinel_core::network::clientbound::play::player_info_update::PlayerInfoUpdatePacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::respawn::RespawnPacket;
use spinel_core::network::clientbound::play::set_chunk_cache_center::SetChunkCacheCenterPacket;
use spinel_core::network::clientbound::play::set_default_spawn_position::SetDefaultSpawnPositionPacket;
use spinel_core::network::clientbound::play::set_time::SetTimePacket;
use spinel_core::network::clientbound::play::sound_effect::SoundEffectPacket;
use spinel_core::network::clientbound::play::sync_player_pos::SyncPlayerPositionPacket;
use spinel_core::network::clientbound::play::world_event::WorldEventPacket;
use spinel_nbt::{Nbt, NbtCompound, Tag, TagReadable, TagWritable};
use spinel_network::types::Identifier;
use spinel_network::types::sound::SoundEvent;
use spinel_network::{ConnectionState, DataType};
use spinel_registry::block_entity_type::BlockEntityType;
use spinel_registry::{Biome, EntityType, Registries};
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread::ThreadId;
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
fn invalidated_light_relights_before_reading_levels() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let lit_position = BlockPosition::new(0, 64, 0);
    let solid_position = BlockPosition::new(1, 64, 0);
    let chunk_position = ChunkPosition::new(0, 0);

    world.load_chunk(chunk_position).unwrap();
    world.set_block(lit_position, Block::GLOWSTONE).unwrap();
    world.set_block(solid_position, Block::STONE).unwrap();

    assert_eq!(world.block_light(lit_position), 15);
    assert_eq!(world.sky_light(solid_position), 0);

    let chunk = world.chunk(chunk_position).unwrap();
    let section = chunk.section_at_block_y(64).unwrap();

    assert!(!section.block_light_is_invalidated());
    assert!(!section.sky_light_is_invalidated());
    assert!(chunk.is_invalidated());
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
fn world_weather_transition_ticks_follow_minestom_overload() {
    let mut world = World::new(Identifier::minecraft("overworld"));

    assert!(
        world
            .set_weather_with_transition(Weather::new(1.0, 0.0), 0)
            .is_err()
    );
    world
        .set_weather_with_transition(Weather::new(1.0, 0.5), 4)
        .unwrap();
    world.tick();

    assert_eq!(world.weather().rain_level(), 1.0);
    assert_eq!(world.remaining_rain_transition_ticks(), 3);
    assert_eq!(world.remaining_thunder_transition_ticks(), 3);
    assert_eq!(world.transitioning_weather().rain_level(), 0.25);
    assert_eq!(world.transitioning_weather().thunder_level(), 0.125);
}

#[test]
fn world_set_biome_loads_chunk_updates_palette_and_timestamp() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let position = BlockPosition::new(-1, 64, -1);
    let chunk_position = ChunkPosition::new(-1, -1);
    let previous_change_time = world.last_block_change_time();

    assert!(world.chunk(chunk_position).is_none());
    assert!(world.set_biome(position, Biome::BADLANDS).unwrap());

    assert_eq!(world.biome_at(position).unwrap(), Biome::BADLANDS);
    assert!(world.chunk(chunk_position).unwrap().is_invalidated());
    assert!(world.last_block_change_time() >= previous_change_time);
}

#[test]
fn world_block_lookup_conditions_match_minestom_cached_and_none_edges() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let position = BlockPosition::new(1, 64, 1);
    let chunk_position = ChunkPosition::new(0, 0);

    assert_eq!(
        world
            .block_at_with_condition(position, BlockLookupCondition::Cached)
            .unwrap(),
        None
    );
    assert!(world.chunk(chunk_position).is_none());
    assert_eq!(
        world
            .block_at_with_condition(position, BlockLookupCondition::None)
            .unwrap(),
        Some(Block::AIR)
    );
    assert!(world.chunk(chunk_position).is_some());
    world.set_block(position, Block::STONE).unwrap();
    assert_eq!(
        world
            .block_at_with_condition(position, BlockLookupCondition::Type)
            .unwrap(),
        Some(Block::STONE)
    );
}

#[test]
fn world_chunk_loader_getter_exposes_assigned_loader_without_mutable_ownership() {
    let mut world = World::new(Identifier::minecraft("overworld"));

    world.set_chunk_loader(CountingUnloadLoader {
        unload_count: Arc::new(AtomicUsize::new(0)),
    });

    assert!(!world.chunk_loader().supports_parallel_loading());
    assert!(!world.chunk_loader().supports_parallel_saving());
}

#[test]
fn future_chunk_loads_share_in_flight_ticket_and_remove_it_after_completion() {
    let load_count = Arc::new(AtomicUsize::new(0));
    let mut world = World::new(Identifier::minecraft("overworld"));
    let chunk_position = ChunkPosition::new(2, 3);
    world.set_chunk_loader(ParallelTrackingLoader {
        load_count: load_count.clone(),
        save_count: Arc::new(AtomicUsize::new(0)),
        load_thread: Arc::new(std::sync::Mutex::new(None)),
        save_thread: Arc::new(std::sync::Mutex::new(None)),
        should_fail_load: false,
    });

    let first_ticket = world.load_chunk_future(chunk_position).unwrap();
    let second_ticket = world.load_chunk_future(chunk_position).unwrap();

    assert_eq!(first_ticket, second_ticket);
    assert!(world.chunk_load_in_progress(chunk_position));
    while !world.complete_chunk_load(first_ticket).unwrap() {}
    assert!(!world.chunk_load_in_progress(chunk_position));
    assert!(world.chunk(chunk_position).is_some());
    assert_eq!(load_count.load(Ordering::SeqCst), 1);
}

#[test]
fn future_chunk_load_failure_removes_in_flight_entry_and_propagates_error() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let chunk_position = ChunkPosition::new(2, 3);
    world.set_chunk_loader(ParallelTrackingLoader {
        load_count: Arc::new(AtomicUsize::new(0)),
        save_count: Arc::new(AtomicUsize::new(0)),
        load_thread: Arc::new(std::sync::Mutex::new(None)),
        save_thread: Arc::new(std::sync::Mutex::new(None)),
        should_fail_load: true,
    });

    let ticket = world.load_chunk_future(chunk_position).unwrap();
    let load_error = loop {
        match world.complete_chunk_load(ticket) {
            Ok(false) => continue,
            Ok(true) => panic!("failed load should not complete successfully"),
            Err(error) => break error,
        }
    };

    assert_eq!(load_error.kind(), io::ErrorKind::Other);
    assert!(!world.chunk_load_in_progress(chunk_position));
}

#[test]
fn optional_future_chunk_load_respects_auto_chunk_load_disabled() {
    let mut world = World::new(Identifier::minecraft("overworld"));

    world.enable_auto_chunk_load(false);

    assert!(
        world
            .load_optional_chunk_future(ChunkPosition::new(2, 3))
            .unwrap()
            .is_none()
    );
}

#[test]
fn future_chunk_load_and_save_follow_loader_parallel_flags() {
    let load_thread = Arc::new(std::sync::Mutex::new(None));
    let save_thread = Arc::new(std::sync::Mutex::new(None));
    let save_count = Arc::new(AtomicUsize::new(0));
    let mut world = World::new(Identifier::minecraft("overworld"));
    let caller_thread = std::thread::current().id();
    world.set_chunk_loader(ParallelTrackingLoader {
        load_count: Arc::new(AtomicUsize::new(0)),
        save_count: save_count.clone(),
        load_thread: load_thread.clone(),
        save_thread: save_thread.clone(),
        should_fail_load: false,
    });

    let ticket = world.load_chunk_future(ChunkPosition::new(0, 0)).unwrap();
    while !world.complete_chunk_load(ticket).unwrap() {}
    world.save_chunks_future().join().unwrap();

    assert_ne!(load_thread.lock().unwrap().unwrap(), caller_thread);
    assert_ne!(save_thread.lock().unwrap().unwrap(), caller_thread);
    assert_eq!(save_count.load(Ordering::SeqCst), 1);
}

#[test]
fn future_chunk_load_and_save_stay_synchronous_when_loader_parallel_flags_are_false() {
    let load_thread = Arc::new(std::sync::Mutex::new(None));
    let save_thread = Arc::new(std::sync::Mutex::new(None));
    let mut world = World::new(Identifier::minecraft("overworld"));
    let caller_thread = std::thread::current().id();
    world.set_chunk_loader(SynchronousTrackingLoader {
        load_thread: load_thread.clone(),
        save_thread: save_thread.clone(),
    });

    let ticket = world.load_chunk_future(ChunkPosition::new(0, 0)).unwrap();
    assert!(world.complete_chunk_load(ticket).unwrap());
    world
        .save_chunk_future(ChunkPosition::new(0, 0))
        .join()
        .unwrap();

    assert_eq!(load_thread.lock().unwrap().unwrap(), caller_thread);
    assert_eq!(save_thread.lock().unwrap().unwrap(), caller_thread);
}

#[test]
fn world_border_api_matches_minestom_initialize_packet_surface() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let border = WorldBorder::new(128.0, 4.0, -8.0, 6, 20, 256).unwrap();
    let mut client = test_client();
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    let mut player = Player::new(
        Uuid::nil(),
        "BorderViewer".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);
    player.mark_entered_world();
    world.add_entity(Entity::Player(player));

    world.set_world_border(border).unwrap();

    let packet = world.create_initialize_world_border_packet();
    assert_eq!(world.world_border(), border);
    assert_eq!(packet.center_x, 4.0);
    assert_eq!(packet.center_z, -8.0);
    assert_eq!(packet.old_diameter, 128.0);
    assert_eq!(packet.new_diameter, 128.0);
    assert_eq!(packet.absolute_max_size, 256);
    assert_eq!(packet.warning_blocks, 6);
    assert_eq!(packet.warning_time, 20);
    assert_eq!(
        client.queued_outbound_packet_ids(),
        vec![InitializeWorldBorderPacket::get_id()]
    );
}

#[test]
fn world_play_sound_except_sends_positioned_sound_to_all_but_excluded_player() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let excluded_uuid = Uuid::new_v4();
    let included_uuid = Uuid::new_v4();
    let mut excluded_client = test_client();
    let mut included_client = test_client();
    excluded_client.state = ConnectionState::Play;
    included_client.state = ConnectionState::Play;
    excluded_client.enable_outbound_packet_queue();
    included_client.enable_outbound_packet_queue();
    let mut excluded_player = Player::new(
        excluded_uuid,
        "Excluded".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let mut included_player = Player::new(
        included_uuid,
        "Included".to_string(),
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25566),
    );
    excluded_player.set_client(&mut excluded_client);
    included_player.set_client(&mut included_client);
    excluded_player.mark_entered_world();
    included_player.mark_entered_world();
    world.add_entity(Entity::Player(excluded_player));
    world.add_entity(Entity::Player(included_player));

    world
        .play_sound_except(
            Some(excluded_uuid),
            SoundEvent::Id(2),
            1,
            EntityPosition::new(1.25, 65.0, -3.5, 0.0, 0.0),
            0.75,
            1.25,
            42,
        )
        .unwrap();

    assert!(excluded_client.queued_outbound_packet_ids().is_empty());
    assert_eq!(
        included_client.queued_outbound_packet_ids(),
        vec![SoundEffectPacket::get_id()]
    );
}

#[test]
fn world_play_sound_except_emitter_delegates_self_emitter_per_player() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let excluded_uuid = Uuid::new_v4();
    let included_uuid = Uuid::new_v4();
    let mut excluded_client = test_client();
    let mut included_client = test_client();
    excluded_client.state = ConnectionState::Play;
    included_client.state = ConnectionState::Play;
    excluded_client.enable_outbound_packet_queue();
    included_client.enable_outbound_packet_queue();
    let mut excluded_player = Player::new(
        excluded_uuid,
        "Excluded".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let mut included_player = Player::new(
        included_uuid,
        "Included".to_string(),
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25566),
    );
    excluded_player.set_client(&mut excluded_client);
    included_player.set_client(&mut included_client);
    excluded_player.mark_entered_world();
    included_player.mark_entered_world();
    world.add_entity(Entity::Player(excluded_player));
    world.add_entity(Entity::Player(included_player));

    world
        .play_sound_except_emitter(
            Some(excluded_uuid),
            SoundEvent::Id(2),
            1,
            WorldSoundEmitter::SelfPlayer,
            0.75,
            1.25,
            42,
        )
        .unwrap();

    assert!(excluded_client.queued_outbound_packet_ids().is_empty());
    assert_eq!(
        included_client.queued_outbound_packet_ids(),
        vec![EntitySoundEffectPacket::get_id()]
    );
}

#[test]
fn world_boss_bar_api_tracks_unique_bars_and_dispatches_show_hide_packets() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut client = test_client();
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    let mut player = Player::new(
        Uuid::nil(),
        "BossViewer".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);
    player.mark_entered_world();
    world.add_entity(Entity::Player(player));
    let boss_bar = BossBar::new(
        spinel_utils::component::text::TextComponent::literal("Boss"),
        0.5,
        WorldBossBarColor::Purple,
        WorldBossBarOverlay::Notched10,
        0,
    );
    let boss_bar_id = boss_bar.id();

    assert!(world.show_boss_bar(boss_bar.clone()).unwrap());
    assert!(!world.show_boss_bar(boss_bar).unwrap());
    assert_eq!(world.boss_bars().len(), 1);
    assert!(world.hide_boss_bar(boss_bar_id).unwrap());
    assert!(!world.hide_boss_bar(boss_bar_id).unwrap());
    assert!(world.boss_bars().is_empty());
    assert_eq!(
        client.queued_outbound_packet_ids(),
        vec![BossBarPacket::get_id(), BossBarPacket::get_id()]
    );
}

#[test]
fn world_explosion_supplier_get_set_and_clear_match_minestom_surface() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let center = EntityPosition::new(1.0, 2.0, 3.0, 0.0, 0.0);

    assert!(world.explosion_supplier().is_none());
    world.set_explosion_supplier(|center, strength| {
        Explosion::new(center, strength, vec![BlockPosition::new(1, 2, 3)])
    });

    let explosion = world
        .explosion_supplier()
        .unwrap()
        .create_explosion(center, 4.0);

    assert_eq!(explosion.center(), center);
    assert_eq!(explosion.strength(), 4.0);
    assert_eq!(explosion.affected_blocks(), &[BlockPosition::new(1, 2, 3)]);
    world.clear_explosion_supplier();
    assert!(world.explosion_supplier().is_none());
}

#[test]
fn world_explode_requires_supplier_like_minestom_state_branch() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let error = world
        .explode(EntityPosition::new(1.0, 2.0, 3.0, 0.0, 0.0), 4.0)
        .unwrap_err();

    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
}

#[test]
fn world_explosion_supplier_receives_additional_data() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let observed_data = Arc::new(AtomicBool::new(false));
    let supplier = DataAwareExplosionSupplier {
        observed_data: observed_data.clone(),
    };
    let mut additional_data = NbtCompound::new();
    additional_data.insert("custom".to_string(), Nbt::Int(7));

    world.set_explosion_supplier(supplier);
    world
        .explode_with_data(
            EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0),
            2.0,
            Some(&additional_data),
        )
        .unwrap();

    assert!(observed_data.load(Ordering::SeqCst));
}

#[test]
fn world_explode_prepares_removes_blocks_sends_packet_and_runs_hooks() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let chunk_position = ChunkPosition::new(0, 0);
    let block_position = BlockPosition::new(1, 64, 1);
    let post_explosion_count = Arc::new(AtomicUsize::new(0));
    let post_send_count = Arc::new(AtomicUsize::new(0));
    let mut client = test_client();
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    let mut player = Player::new(
        Uuid::nil(),
        "ExplosionViewer".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);
    player.mark_entered_world();
    world.add_entity(Entity::Player(player));
    world.load_chunk(chunk_position).unwrap();
    world.set_block(block_position, Block::STONE).unwrap();

    let post_explosion_count_for_hook = post_explosion_count.clone();
    let post_send_count_for_hook = post_send_count.clone();
    world.set_explosion_supplier(move |center, strength| {
        Explosion::new(center, strength, Vec::new())
            .with_prepare(move |_world, _affected_blocks| vec![block_position])
            .with_post_explosion({
                let post_explosion_count = post_explosion_count_for_hook.clone();
                move |_world, affected_blocks, packet| {
                    assert_eq!(affected_blocks, &[block_position]);
                    packet.radius = 3.0;
                    post_explosion_count.fetch_add(1, Ordering::SeqCst);
                }
            })
            .with_post_send({
                let post_send_count = post_send_count_for_hook.clone();
                move |world, affected_blocks| {
                    assert_eq!(affected_blocks, &[block_position]);
                    world.set_block(block_position, Block::DIRT).unwrap();
                    post_send_count.fetch_add(1, Ordering::SeqCst);
                }
            })
    });

    let affected_blocks = world
        .explode(EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0), 4.0)
        .unwrap();

    assert_eq!(affected_blocks, vec![block_position]);
    assert_eq!(world.loaded_block_at(block_position), Some(Block::DIRT));
    assert_eq!(post_explosion_count.load(Ordering::SeqCst), 1);
    assert_eq!(post_send_count.load(Ordering::SeqCst), 1);
    assert!(
        client
            .queued_outbound_packet_ids()
            .contains(&ExplosionPacket::get_id())
    );
}

#[test]
fn block_handler_place_destroy_and_chunk_tick_follow_minestom_owner_flow() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let handler_counts = Arc::new(BlockHandlerCounts::default());
    let block_position = BlockPosition::new(1, 64, 1);
    let player = Player::new(
        Uuid::nil(),
        "Breaker".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let player_id = player.entity_id();
    world.add_entity(Entity::Player(player));
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();

    world.register_block_handler(
        Block::CHEST,
        CountingBlockHandler {
            key: Identifier::minecraft("chest"),
            counts: handler_counts.clone(),
            tickable: true,
        },
    );
    world
        .place_block(BlockHandlerPlacement::new(
            Block::CHEST,
            Block::AIR,
            world.uuid(),
            block_position,
        ))
        .then_some(())
        .unwrap();

    assert_eq!(handler_counts.place_count.load(Ordering::SeqCst), 1);
    assert_eq!(
        world
            .chunk(ChunkPosition::new(0, 0))
            .unwrap()
            .tickable_block_count(),
        1
    );
    assert_eq!(world.tick_chunks(1), 1);
    assert_eq!(handler_counts.tick_count.load(Ordering::SeqCst), 1);
    world.tick();
    assert_eq!(handler_counts.tick_count.load(Ordering::SeqCst), 2);

    assert!(world.break_block(
        player_id,
        block_position,
        crate::events::player_block_interact::BlockFace::Top,
    ));

    assert_eq!(handler_counts.destroy_count.load(Ordering::SeqCst), 1);
    assert_eq!(
        world
            .chunk(ChunkPosition::new(0, 0))
            .unwrap()
            .tickable_block_count(),
        0
    );
}

#[test]
fn block_handler_interaction_routes_through_world_owner_before_item_use() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let handler_counts = Arc::new(BlockHandlerCounts::default());
    let block_position = BlockPosition::new(1, 64, 1);
    let player = Player::new(
        Uuid::nil(),
        "Interactor".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let player_id = player.entity_id();
    world.add_entity(Entity::Player(player));
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world.register_block_handler(
        Block::CHEST,
        CountingBlockHandler {
            key: Identifier::minecraft("chest"),
            counts: handler_counts.clone(),
            tickable: false,
        },
    );
    world.set_block(block_position, Block::CHEST).unwrap();

    let handler_allows_item_use = world.interact_block_handler(
        player_id,
        crate::entity::PlayerHand::Main,
        crate::events::player_block_interact::BlockFace::Top,
        block_position,
        (0.5, 1.0, 0.5),
    );

    assert!(!handler_allows_item_use);
    assert_eq!(handler_counts.interact_count.load(Ordering::SeqCst), 1);
}

#[test]
fn block_handler_touch_routes_from_world_entity_tick() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let handler_counts = Arc::new(BlockHandlerCounts::default());
    let block_position = BlockPosition::new(1, 64, 1);
    let mut entity = Entity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(1.5, 64.0, 1.5, 0.0, 0.0));
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world.register_block_handler(
        Block::CHEST,
        CountingBlockHandler {
            key: Identifier::minecraft("chest"),
            counts: handler_counts.clone(),
            tickable: false,
        },
    );
    world.set_block(block_position, Block::CHEST).unwrap();
    world.add_entity(entity);

    world.tick();

    assert_eq!(handler_counts.touch_count.load(Ordering::SeqCst), 1);
}

#[test]
fn block_handler_place_block_requires_loaded_chunk_like_minestom() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let block_position = BlockPosition::new(1, 64, 1);

    assert!(!world.place_block(BlockHandlerPlacement::new(
        Block::CHEST,
        Block::AIR,
        world.uuid(),
        block_position,
    )));
}

#[test]
fn block_placement_rule_changes_placed_block_only_when_updates_are_enabled() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let block_position = BlockPosition::new(1, 64, 1);
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world.register_block_placement_rule(ReplaceOnPlaceRule {
        source_block: Block::STONE,
        placed_block: Block::DIRT,
    });

    assert!(world.place_block_with_updates(
        BlockHandlerPlacement::new(Block::STONE, Block::AIR, world.uuid(), block_position),
        true,
    ));
    assert_eq!(world.loaded_block_at(block_position), Some(Block::DIRT));

    assert!(world.place_block_with_updates(
        BlockHandlerPlacement::new(
            Block::STONE,
            Block::DIRT,
            world.uuid(),
            BlockPosition::new(2, 64, 1),
        ),
        false,
    ));
    assert_eq!(
        world.loaded_block_at(BlockPosition::new(2, 64, 1)),
        Some(Block::STONE)
    );
}

#[test]
fn neighbor_block_placement_rules_recompute_six_neighbors_until_max_update_distance() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let changed_position = BlockPosition::new(1, 64, 1);
    let first_neighbor = BlockPosition::new(2, 64, 1);
    let second_neighbor = BlockPosition::new(3, 64, 1);
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world.set_block(first_neighbor, Block::CHEST).unwrap();
    world.set_block(second_neighbor, Block::CHEST).unwrap();
    world.register_block_placement_rule(NeighborUpdateRule {
        source_block: Block::CHEST,
        updated_block: Block::TRAPPED_CHEST,
        max_distance: 1,
    });

    assert!(world.place_block_with_updates(
        BlockHandlerPlacement::new(Block::STONE, Block::AIR, world.uuid(), changed_position),
        true,
    ));

    assert_eq!(
        world.loaded_block_at(first_neighbor),
        Some(Block::TRAPPED_CHEST)
    );
    assert_eq!(world.loaded_block_at(second_neighbor), Some(Block::CHEST));
}

#[test]
fn block_mutation_dispatches_instance_block_update_event_and_refreshes_timestamp() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let event_count = Arc::new(AtomicUsize::new(0));
    let listener_count = event_count.clone();
    let block_position = BlockPosition::new(1, 64, 1);
    let previous_change_time = world.last_block_change_time();

    world
        .event_node()
        .listen("InstanceBlockUpdateEvent", move |_| {
            listener_count.fetch_add(1, Ordering::SeqCst);
        });
    world.set_block(block_position, Block::STONE).unwrap();

    assert_eq!(event_count.load(Ordering::SeqCst), 1);
    assert!(world.last_block_change_time() >= previous_change_time);
    assert!(world.block_change_guard_contains(block_position, Block::STONE));
}

#[test]
fn world_tick_clears_changed_block_recursion_guard() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let block_position = BlockPosition::new(1, 64, 1);

    world.set_block(block_position, Block::STONE).unwrap();
    assert!(world.block_change_guard_contains(block_position, Block::STONE));
    world.tick();

    assert!(!world.block_change_guard_contains(block_position, Block::STONE));
}

#[test]
fn block_entity_data_packet_is_sent_to_chunk_viewers_after_block_entity_capable_write() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let chunk_position = ChunkPosition::new(0, 0);
    let block_position = BlockPosition::new(1, 64, 1);
    let mut client = test_client();
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    let mut player = Player::new(
        Uuid::nil(),
        "Viewer".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let player_id = player.entity_id();
    player.set_client(&mut client);
    player.mark_entered_world();
    world.load_chunk(chunk_position).unwrap();
    world
        .load_chunk(chunk_position)
        .unwrap()
        .add_viewer(player_id);
    world
        .load_chunk(chunk_position)
        .unwrap()
        .set_block_entity(BlockEntity::new(
            block_position,
            BlockEntityType::Chest,
            spinel_nbt::NbtCompound::new(),
        ));
    world.add_entity(Entity::Player(player));

    world.set_block(block_position, Block::CHEST).unwrap();

    assert_eq!(
        client.queued_outbound_packet_ids(),
        vec![BlockUpdatePacket::get_id(), BlockEntityDataPacket::get_id()]
    );
}

#[test]
fn block_entity_refresh_sends_special_block_data_for_failed_digging_resync() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut client = test_client();
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    let chunk_position = ChunkPosition::new(0, 0);
    let block_position = BlockPosition::new(1, 64, 1);
    let mut nbt = NbtCompound::new();
    nbt.insert("custom_name".to_string(), Nbt::String("Chest".to_string()));

    world.load_chunk(chunk_position).unwrap();
    world.set_block(block_position, Block::CHEST).unwrap();
    world
        .load_chunk(chunk_position)
        .unwrap()
        .set_block_entity(BlockEntity::new(
            block_position,
            BlockEntityType::Chest,
            nbt,
        ));

    world
        .refresh_block_entity_for_player(&mut client, block_position)
        .unwrap();

    assert_eq!(
        client.queued_outbound_packet_ids(),
        vec![BlockEntityDataPacket::get_id()]
    );
}

#[test]
fn block_entity_data_survives_direct_write_placement_and_is_removed_on_break() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let block_position = BlockPosition::new(1, 64, 1);
    let chunk_position = ChunkPosition::new(0, 0);
    let player = Player::new(
        Uuid::nil(),
        "Breaker".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let player_id = player.entity_id();
    let mut nbt = NbtCompound::new();
    nbt.insert("custom".to_string(), Nbt::Int(4));

    world.load_chunk(chunk_position).unwrap();
    world.add_entity(Entity::Player(player));
    world
        .load_chunk(chunk_position)
        .unwrap()
        .set_block_entity(BlockEntity::new(
            block_position,
            BlockEntityType::Chest,
            nbt,
        ));
    world.set_block(block_position, Block::CHEST).unwrap();

    assert!(
        world
            .chunk(chunk_position)
            .unwrap()
            .block_entity(block_position)
            .is_some()
    );

    assert!(world.place_block(BlockHandlerPlacement::new(
        Block::TRAPPED_CHEST,
        Block::CHEST,
        world.uuid(),
        block_position,
    )));

    assert!(
        world
            .chunk(chunk_position)
            .unwrap()
            .block_entity(block_position)
            .is_some()
    );

    assert!(world.break_block(
        player_id,
        block_position,
        crate::events::player_block_interact::BlockFace::Top,
    ));
    assert!(
        world
            .chunk(chunk_position)
            .unwrap()
            .block_entity(block_position)
            .is_none()
    );
}

#[test]
fn block_entity_packet_uses_handler_type_and_client_tag_filter() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let chunk_position = ChunkPosition::new(0, 0);
    let block_position = BlockPosition::new(1, 64, 1);
    let mut client = test_client();
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    let mut player = Player::new(
        Uuid::nil(),
        "Viewer".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let player_id = player.entity_id();
    let mut block_entity_nbt = NbtCompound::new();
    block_entity_nbt.insert("allowed".to_string(), Nbt::String("yes".to_string()));
    block_entity_nbt.insert("server_only".to_string(), Nbt::String("no".to_string()));
    player.set_client(&mut client);
    player.mark_entered_world();
    world.register_block_handler(Block::CHEST, FilteringBlockEntityHandler);
    world.load_chunk(chunk_position).unwrap();
    world
        .load_chunk(chunk_position)
        .unwrap()
        .add_viewer(player_id);
    world
        .load_chunk(chunk_position)
        .unwrap()
        .set_block_entity(BlockEntity::new(
            block_position,
            BlockEntityType::Chest,
            block_entity_nbt,
        ));
    world.add_entity(Entity::Player(player));

    world.set_block(block_position, Block::CHEST).unwrap();

    let packet_ids = client.queued_outbound_packet_ids();
    let block_entity_payload = client
        .queued_outbound_packet_payloads()
        .into_iter()
        .find_map(|(packet_id, payload)| {
            (packet_id == BlockEntityDataPacket::get_id()).then_some(payload)
        })
        .unwrap();
    let decoded_packet =
        BlockEntityDataPacket::decode(&mut block_entity_payload.as_slice()).unwrap();

    assert_eq!(
        packet_ids,
        vec![BlockUpdatePacket::get_id(), BlockEntityDataPacket::get_id()]
    );
    assert_eq!(
        decoded_packet.block_entity_type.0,
        BlockEntityType::TrappedChest
    );
    assert!(
        decoded_packet
            .data
            .0
            .as_ref()
            .is_some_and(|nbt| nbt.contains_key("allowed") && !nbt.contains_key("server_only"))
    );
}

#[test]
fn break_block_sends_destroy_effect_to_chunk_viewers_except_breaker() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let chunk_position = ChunkPosition::new(0, 0);
    let block_position = BlockPosition::new(1, 64, 1);
    let mut breaker_client = test_client();
    let mut viewer_client = test_client();
    breaker_client.state = ConnectionState::Play;
    viewer_client.state = ConnectionState::Play;
    breaker_client.enable_outbound_packet_queue();
    viewer_client.enable_outbound_packet_queue();
    let mut breaker = Player::new(
        Uuid::new_v4(),
        "Breaker".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let mut viewer = Player::new(
        Uuid::new_v4(),
        "Viewer".to_string(),
        1,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25566),
    );
    let breaker_id = breaker.entity_id();
    let viewer_id = viewer.entity_id();
    breaker.set_client(&mut breaker_client);
    viewer.set_client(&mut viewer_client);
    breaker.mark_entered_world();
    viewer.mark_entered_world();
    world.load_chunk(chunk_position).unwrap();
    world.set_block(block_position, Block::STONE).unwrap();
    world
        .load_chunk(chunk_position)
        .unwrap()
        .add_viewer(breaker_id);
    world
        .load_chunk(chunk_position)
        .unwrap()
        .add_viewer(viewer_id);
    world.add_entity(Entity::Player(breaker));
    world.add_entity(Entity::Player(viewer));

    assert!(world.break_block(
        breaker_id,
        block_position,
        crate::events::player_block_interact::BlockFace::Top,
    ));

    assert_eq!(
        breaker_client.queued_outbound_packet_ids(),
        vec![BlockUpdatePacket::get_id()]
    );
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![BlockUpdatePacket::get_id(), WorldEventPacket::get_id()]
    );
}

#[test]
fn world_identity_and_pointers_resolve_instance_uuid_like_minestom() {
    let world = World::new(Identifier::minecraft("overworld"));

    assert_eq!(world.identity().uuid(), world.uuid());
    assert_eq!(world.pointers().uuid(), world.uuid());
    assert_eq!(world.pointers().identity(), world.identity());
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
fn chunk_unload_removes_players_and_generic_entities_in_unloaded_chunk() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let chunk_position = ChunkPosition::new(0, 0);
    let generic_entity = Entity::new(EntityType::ZOMBIE);
    let generic_entity_id = generic_entity.entity_id();
    let player = Player::new(
        Uuid::nil(),
        "ChunkPlayer".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let player_uuid = player.uuid();

    world.load_chunk(chunk_position).unwrap();
    world.add_entity(generic_entity);
    world.add_entity(Entity::Player(player));

    assert!(world.entity_by_id(generic_entity_id).is_some());
    assert!(world.player_by_uuid(player_uuid).is_some());
    assert_eq!(world.chunk_entities(chunk_position).len(), 2);

    world.unload_chunk(chunk_position).unwrap();

    assert!(world.entity_by_id(generic_entity_id).is_none());
    assert!(world.player_by_uuid(player_uuid).is_none());
    assert!(world.chunk_entities(chunk_position).is_empty());
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
fn world_manager_register_and_unregister_dispatch_instance_events_in_order() {
    let event_order = Arc::new(std::sync::Mutex::new(Vec::new()));
    let mut worlds = WorldManager::new();
    let mut world = World::new(Identifier::minecraft("overworld"));
    let world_uuid = world.uuid();
    let register_order = event_order.clone();
    let unregister_order = event_order.clone();

    world
        .event_node()
        .listen("InstanceRegisterEvent", move |_| {
            register_order.lock().unwrap().push("register");
        });
    world
        .event_node()
        .listen("InstanceUnregisterEvent", move |_| {
            unregister_order.lock().unwrap().push("unregister");
        });
    worlds.register_world(world);
    worlds.unregister_world(world_uuid).unwrap().unwrap();

    assert_eq!(
        event_order.lock().unwrap().as_slice(),
        ["register", "unregister"]
    );
}

#[test]
fn world_manager_unregister_world_rejects_online_players() {
    let mut worlds = WorldManager::new();
    let world_uuid = worlds.create_world(Identifier::minecraft("overworld"));
    let mut client = test_client();
    let mut player = Player::new(
        Uuid::nil(),
        "Online".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);
    worlds.add_entity(world_uuid, Entity::Player(player));

    let error = match worlds.unregister_world(world_uuid) {
        Ok(_) => panic!("online player unregister should fail"),
        Err(error) => error,
    };

    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    assert!(worlds.world(world_uuid).is_some());
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

struct ParallelTrackingLoader {
    load_count: Arc<AtomicUsize>,
    save_count: Arc<AtomicUsize>,
    load_thread: Arc<std::sync::Mutex<Option<ThreadId>>>,
    save_thread: Arc<std::sync::Mutex<Option<ThreadId>>>,
    should_fail_load: bool,
}

struct SynchronousTrackingLoader {
    load_thread: Arc<std::sync::Mutex<Option<ThreadId>>>,
    save_thread: Arc<std::sync::Mutex<Option<ThreadId>>>,
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

impl ChunkLoader for ParallelTrackingLoader {
    fn load_chunk(&self, _position: ChunkPosition) -> io::Result<Option<Chunk>> {
        self.load_count.fetch_add(1, Ordering::SeqCst);
        *self.load_thread.lock().unwrap() = Some(std::thread::current().id());
        if self.should_fail_load {
            return Err(io::Error::new(io::ErrorKind::Other, "load failed"));
        }
        Ok(None)
    }

    fn save_chunk(&self, _chunk: &Chunk) -> io::Result<()> {
        self.save_count.fetch_add(1, Ordering::SeqCst);
        *self.save_thread.lock().unwrap() = Some(std::thread::current().id());
        Ok(())
    }

    fn unload_chunk(&self, chunk: &mut Chunk) -> io::Result<()> {
        chunk.unload();
        Ok(())
    }

    fn supports_parallel_loading(&self) -> bool {
        true
    }

    fn supports_parallel_saving(&self) -> bool {
        true
    }
}

impl ChunkLoader for SynchronousTrackingLoader {
    fn load_chunk(&self, _position: ChunkPosition) -> io::Result<Option<Chunk>> {
        *self.load_thread.lock().unwrap() = Some(std::thread::current().id());
        Ok(None)
    }

    fn save_chunk(&self, _chunk: &Chunk) -> io::Result<()> {
        *self.save_thread.lock().unwrap() = Some(std::thread::current().id());
        Ok(())
    }

    fn unload_chunk(&self, chunk: &mut Chunk) -> io::Result<()> {
        chunk.unload();
        Ok(())
    }
}

#[test]
fn generic_entity_set_instance_moves_between_worlds_after_target_chunk_load() {
    let mut worlds = WorldManager::new();
    let first_world = worlds.create_world(Identifier::minecraft("first"));
    let second_world = worlds.create_world(Identifier::minecraft("second"));
    let entity = Entity::new(EntityType::ZOMBIE);
    let entity_id = entity.entity_id();
    let target_position = EntityPosition::new(32.0, 70.0, -16.0, 90.0, 45.0);

    worlds.add_entity(first_world, entity);
    worlds
        .set_entity_world_at_position(entity_id, second_world, target_position)
        .unwrap();

    assert!(
        worlds
            .world(first_world)
            .unwrap()
            .entity_by_id(entity_id)
            .is_none()
    );
    assert_eq!(
        worlds
            .world(second_world)
            .unwrap()
            .entity_by_id(entity_id)
            .map(Entity::position),
        Some(target_position)
    );
    assert!(
        worlds
            .chunk_for_world(second_world, ChunkPosition::new(2, -1))
            .is_some()
    );
}

#[test]
fn generic_entity_set_instance_point_and_default_position_match_minestom_overloads() {
    let mut worlds = WorldManager::new();
    let first_world = worlds.create_world(Identifier::minecraft("first"));
    let second_world = worlds.create_world(Identifier::minecraft("second"));
    let third_world = worlds.create_world(Identifier::minecraft("third"));
    let mut entity = Entity::new(EntityType::ZOMBIE);
    let entity_id = entity.entity_id();
    entity.set_position(EntityPosition::new(1.0, 64.0, 1.0, 15.0, 30.0));

    worlds.add_entity(first_world, entity);
    worlds
        .set_entity_world_at_point(entity_id, second_world, BlockPosition::new(5, 66, 7))
        .unwrap();
    worlds.set_entity_world(entity_id, third_world).unwrap();

    let entity = worlds
        .world(third_world)
        .unwrap()
        .entity_by_id(entity_id)
        .unwrap();

    assert_eq!(entity.position().x(), 5.0);
    assert_eq!(entity.position().y(), 66.0);
    assert_eq!(entity.position().z(), 7.0);
    assert_eq!(entity.position().yaw(), 15.0);
    assert_eq!(entity.position().pitch(), 30.0);
}

#[test]
fn player_set_instance_rejects_same_world_and_completes_on_next_manager_tick() {
    let mut server = MinecraftServer::new();
    let first_world = server
        .world_manager
        .create_world(Identifier::minecraft("first"));
    let second_world = server
        .world_manager
        .create_world(Identifier::minecraft("second"));
    server
        .world_manager
        .world_mut(second_world)
        .unwrap()
        .set_view_distance(0);
    let player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let player_uuid = player.uuid();
    server
        .world_manager
        .add_entity(first_world, Entity::Player(player));

    assert!(
        server
            .world_manager
            .set_player_world(player_uuid, first_world)
            .is_err()
    );
    server
        .world_manager
        .set_player_world_at_position(
            player_uuid,
            second_world,
            EntityPosition::new(16.0, 65.0, 0.0, 0.0, 0.0),
        )
        .unwrap();

    assert_eq!(
        server.world_manager.pending_player_world_transition_count(),
        1
    );
    assert!(
        server
            .world_manager
            .world(second_world)
            .unwrap()
            .player_by_uuid(player_uuid)
            .is_none()
    );

    let registries = Registries::new_vanilla();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    server.world_manager.tick(&registries, server_ptr);

    let player = server
        .world_manager
        .world(second_world)
        .unwrap()
        .player_by_uuid(player_uuid)
        .unwrap();

    assert_eq!(
        server.world_manager.pending_player_world_transition_count(),
        0
    );
    assert_eq!(player.position().x(), 16.0);
    assert_eq!(player.queued_chunk_count(), 9);
}

#[test]
fn inactive_player_set_instance_default_uses_respawn_point() {
    let mut server = MinecraftServer::new();
    let target_world = server
        .world_manager
        .create_world(Identifier::minecraft("target"));
    server
        .world_manager
        .world_mut(target_world)
        .unwrap()
        .set_view_distance(0);
    let mut client = test_client();
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    let mut player = Player::new(Uuid::new_v4(), "Inactive".to_string(), 0, client.addr);
    let player_uuid = player.uuid();
    player.set_client(&mut client);
    player.set_respawn_point(PlayerSpawnPoint::new(32.0, 70.0, -16.0, 45.0, 20.0));

    assert!(server.world_manager.add_inactive_player(player));

    let ticket = server
        .world_manager
        .set_player_world_future(player_uuid, target_world)
        .unwrap();
    let registries = Registries::new_vanilla();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    server.world_manager.tick(&registries, server_ptr);

    let player = server
        .world_manager
        .world(target_world)
        .unwrap()
        .player_by_uuid(player_uuid)
        .unwrap();

    assert!(
        server
            .world_manager
            .player_world_transition_is_complete(ticket)
    );
    assert_eq!(player.position().x(), 32.0);
    assert_eq!(player.position().y(), 70.0);
    assert_eq!(player.position().z(), -16.0);
    assert_eq!(player.position().yaw(), 45.0);
    assert_eq!(player.position().pitch(), 20.0);
    assert!(server.world_manager.inactive_player(player_uuid).is_none());

    let packet_ids = client.queued_outbound_packet_ids();
    assert!(!packet_ids.contains(&RespawnPacket::get_id()));
    assert_packet_order(
        &packet_ids,
        &[
            SetChunkCacheCenterPacket::get_id(),
            SyncPlayerPositionPacket::get_id(),
            GameEventPacket::get_id(),
        ],
    );
}

#[test]
fn player_set_instance_future_completes_after_spawn_packets_and_viewer_refresh() {
    let mut server = MinecraftServer::new();
    let first_world = server
        .world_manager
        .create_world(Identifier::minecraft("first"));
    let second_world = server
        .world_manager
        .create_world(Identifier::minecraft("second"));
    server
        .world_manager
        .world_mut(second_world)
        .unwrap()
        .set_view_distance(0);
    let mut moving_client = test_client();
    let mut old_viewer_client = test_client();
    let mut new_viewer_client = test_client();
    moving_client.state = ConnectionState::Play;
    old_viewer_client.state = ConnectionState::Play;
    new_viewer_client.state = ConnectionState::Play;
    moving_client.enable_outbound_packet_queue();
    old_viewer_client.enable_outbound_packet_queue();
    new_viewer_client.enable_outbound_packet_queue();
    let mut moving_player =
        Player::new(Uuid::new_v4(), "Moving".to_string(), 0, moving_client.addr);
    let mut old_viewer = Player::new(
        Uuid::new_v4(),
        "OldViewer".to_string(),
        1,
        old_viewer_client.addr,
    );
    let mut new_viewer = Player::new(
        Uuid::new_v4(),
        "NewViewer".to_string(),
        2,
        new_viewer_client.addr,
    );
    let moving_player_uuid = moving_player.uuid();
    moving_player.set_client(&mut moving_client);
    old_viewer.set_client(&mut old_viewer_client);
    new_viewer.set_client(&mut new_viewer_client);
    moving_player.mark_entered_world();
    old_viewer.mark_entered_world();
    new_viewer.mark_entered_world();
    server
        .world_manager
        .add_entity(first_world, Entity::Player(moving_player));
    server
        .world_manager
        .add_entity(first_world, Entity::Player(old_viewer));
    server
        .world_manager
        .add_entity(second_world, Entity::Player(new_viewer));

    let ticket = server
        .world_manager
        .set_player_world_at_position_future(
            moving_player_uuid,
            second_world,
            EntityPosition::new(1.0, 65.0, 1.0, 0.0, 0.0),
        )
        .unwrap();

    assert!(
        !server
            .world_manager
            .player_world_transition_is_complete(ticket)
    );

    let registries = Registries::new_vanilla();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    server.world_manager.tick(&registries, server_ptr);

    assert!(
        server
            .world_manager
            .player_world_transition_is_complete(ticket)
    );
    assert_eq!(
        old_viewer_client.queued_outbound_packet_ids()[..2],
        [
            PlayerInfoRemovePacket::get_id(),
            RemoveEntitiesPacket::get_id()
        ]
    );
    assert!(
        new_viewer_client
            .queued_outbound_packet_ids()
            .contains(&PlayerInfoUpdatePacket::get_id())
    );
    let moving_packets = moving_client.queued_outbound_packet_ids();
    assert!(!moving_packets.contains(&LoginPlayPacket::get_id()));
    assert_packet_order(
        &moving_packets,
        &[
            RespawnPacket::get_id(),
            SetChunkCacheCenterPacket::get_id(),
            SyncPlayerPositionPacket::get_id(),
            SetDefaultSpawnPositionPacket::get_id(),
            InitializeWorldBorderPacket::get_id(),
            SetTimePacket::get_id(),
        ],
    );
}

#[test]
fn same_dimension_player_set_instance_does_not_send_respawn_packet() {
    let mut server = MinecraftServer::new();
    let first_world = server
        .world_manager
        .create_world(Identifier::minecraft("same_dimension"));
    let second_world = server
        .world_manager
        .create_world(Identifier::minecraft("same_dimension"));
    server
        .world_manager
        .world_mut(second_world)
        .unwrap()
        .set_view_distance(0);
    let mut client = test_client();
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    let mut player = Player::new(Uuid::new_v4(), "Moving".to_string(), 0, client.addr);
    let player_uuid = player.uuid();
    player.set_client(&mut client);
    player.mark_entered_world();
    server
        .world_manager
        .add_entity(first_world, Entity::Player(player));

    server
        .world_manager
        .set_player_world_at_position(
            player_uuid,
            second_world,
            EntityPosition::new(1.0, 65.0, 1.0, 0.0, 0.0),
        )
        .unwrap();
    let registries = Registries::new_vanilla();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    server.world_manager.tick(&registries, server_ptr);

    let packet_ids = client.queued_outbound_packet_ids();
    assert!(!packet_ids.contains(&RespawnPacket::get_id()));
    assert_packet_order(
        &packet_ids,
        &[
            SetChunkCacheCenterPacket::get_id(),
            SyncPlayerPositionPacket::get_id(),
        ],
    );
}

#[test]
fn shared_world_registration_linking_and_chunk_delegation_match_minestom_shape() {
    let mut worlds = WorldManager::new();
    let source_world = worlds.create_world(Identifier::minecraft("source"));
    let shared_world = worlds.create_shared_world(source_world).unwrap();
    let chunk_position = ChunkPosition::new(3, -2);

    assert!(worlds.shared_world(shared_world).unwrap().is_registered());
    assert!(worlds.worlds_are_linked(source_world, shared_world));
    worlds
        .load_chunk_for_world(shared_world, chunk_position)
        .unwrap();
    assert!(
        worlds
            .chunk_for_world(source_world, chunk_position)
            .is_some()
    );
    assert!(
        worlds
            .chunk_for_world(shared_world, chunk_position)
            .is_some()
    );
    assert!(
        worlds
            .save_chunk_for_world(shared_world, chunk_position)
            .unwrap()
    );
    worlds.save_chunks_for_world(shared_world).unwrap();
    assert!(
        worlds
            .generate_chunk_for_world(shared_world, chunk_position)
            .unwrap()
    );
    assert!(
        worlds
            .unload_chunk_for_world(shared_world, chunk_position)
            .unwrap()
    );
    assert!(
        worlds
            .chunk_for_world(source_world, chunk_position)
            .is_none()
    );
    assert!(worlds.world(source_world).unwrap().has_shared_worlds());
    assert_eq!(
        worlds.world(source_world).unwrap().shared_worlds(),
        &[shared_world]
    );
    assert_eq!(
        worlds
            .shared_world(shared_world)
            .unwrap()
            .world()
            .source_world(),
        Some(source_world)
    );
}

#[test]
fn world_copy_preserves_loaded_chunks_tags_dimension_and_source_world() {
    let mut worlds = WorldManager::new();
    let source_world = worlds.create_world(Identifier::minecraft("source"));
    let chunk_position = ChunkPosition::new(1, -1);
    let block_position = BlockPosition::new(17, 64, -1);
    let copied_marker = Tag::<i32>::integer("copied_marker");
    {
        let world = worlds.world_mut(source_world).unwrap();
        world.set_tag(&copied_marker, Some(12));
        world.load_chunk(chunk_position).unwrap();
        world.set_block(block_position, Block::STONE).unwrap();
    }

    let copied_world = worlds.copy_world(source_world).unwrap();
    let copied = worlds.world(copied_world).unwrap();

    assert_ne!(copied_world, source_world);
    assert_eq!(copied.source_world(), Some(source_world));
    assert_eq!(copied.get_tag(&copied_marker), Some(12));
    assert_eq!(
        copied.chunk(chunk_position).unwrap().block(block_position),
        Block::STONE
    );
    assert_eq!(
        copied.chunk(chunk_position).unwrap().world(),
        Some(copied_world)
    );
}

#[test]
fn world_manager_instances_include_normal_and_shared_worlds_as_owned_snapshot() {
    let mut worlds = WorldManager::new();
    let source_world = worlds.create_world(Identifier::minecraft("source"));
    let shared_world = worlds.create_shared_world(source_world).unwrap();
    let instance_uuids = worlds.instance_uuids();

    assert!(instance_uuids.contains(&source_world));
    assert!(instance_uuids.contains(&shared_world));
    assert_eq!(instance_uuids.len(), 2);
}

#[test]
fn linked_shared_world_same_chunk_player_transition_skips_chunk_refresh() {
    let mut server = MinecraftServer::new();
    let source_world = server
        .world_manager
        .create_world(Identifier::minecraft("source"));
    let shared_world = server
        .world_manager
        .create_shared_world(source_world)
        .unwrap();
    let player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let player_uuid = player.uuid();

    server
        .world_manager
        .add_entity(source_world, Entity::Player(player));
    server
        .world_manager
        .set_player_world_at_position(
            player_uuid,
            shared_world,
            EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0),
        )
        .unwrap();

    let registries = Registries::new_vanilla();
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    server.world_manager.tick(&registries, server_ptr);

    let player = server
        .world_manager
        .world(shared_world)
        .unwrap()
        .player_by_uuid(player_uuid)
        .unwrap();

    assert_eq!(player.queued_chunk_count(), 0);
    assert!(
        server
            .world_manager
            .worlds_are_linked(source_world, shared_world)
    );
}

#[test]
fn world_snapshot_preserves_instance_chunk_entity_and_tag_state_after_mutation() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let world_tag = Tag::<i32>::integer("snapshot_world");
    let chunk_tag = Tag::<i32>::integer("snapshot_chunk");
    let chunk_position = ChunkPosition::new(0, 0);
    let block_position = BlockPosition::new(1, 64, 1);
    let entity = Entity::new(EntityType::ZOMBIE);
    let entity_id = entity.entity_id();

    world.set_tag(&world_tag, Some(5));
    world.load_chunk(chunk_position).unwrap();
    world
        .load_chunk(chunk_position)
        .unwrap()
        .set_tag(&chunk_tag, Some(8));
    world.set_block(block_position, Block::STONE).unwrap();
    world.add_entity(entity);

    let snapshot = world.update_snapshot();

    world.set_tag(&world_tag, Some(10));
    world
        .load_chunk(chunk_position)
        .unwrap()
        .set_tag(&chunk_tag, Some(16));
    world.set_block(block_position, Block::DIRT).unwrap();

    let chunk_snapshot = snapshot.chunk(chunk_position).unwrap();
    assert_eq!(snapshot.world(), world.uuid());
    assert_eq!(snapshot.name(), world.name());
    assert_eq!(snapshot.dimension_type(), world.dimension_type());
    assert_eq!(snapshot.get_tag(&world_tag), Some(5));
    assert_eq!(chunk_snapshot.get_tag(&chunk_tag), Some(8));
    assert_eq!(chunk_snapshot.block(block_position), Block::STONE);
    assert_eq!(chunk_snapshot.entity_ids(), &[entity_id]);
    assert_eq!(snapshot.entity_ids(), &[entity_id]);
    assert_eq!(snapshot.chunks().count(), 1);
}

fn test_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = TcpStream::connect(addr).unwrap();
    let (_peer_stream, _) = listener.accept().unwrap();
    Client::new(stream, addr)
}

fn assert_packet_order(packet_ids: &[i32], expected_order: &[i32]) {
    let mut packet_index = 0;
    for expected_packet_id in expected_order {
        let Some(relative_index) = packet_ids[packet_index..]
            .iter()
            .position(|packet_id| packet_id == expected_packet_id)
        else {
            panic!("packet {expected_packet_id} missing from {packet_ids:?}");
        };
        packet_index += relative_index + 1;
    }
}

#[derive(Default)]
struct BlockHandlerCounts {
    place_count: AtomicUsize,
    destroy_count: AtomicUsize,
    interact_count: AtomicUsize,
    touch_count: AtomicUsize,
    tick_count: AtomicUsize,
}

struct DataAwareExplosionSupplier {
    observed_data: Arc<AtomicBool>,
}

impl crate::world::ExplosionSupplier for DataAwareExplosionSupplier {
    fn create_explosion(&self, center: EntityPosition, strength: f32) -> Explosion {
        Explosion::new(center, strength, Vec::new())
    }

    fn create_explosion_with_data(
        &self,
        center: EntityPosition,
        strength: f32,
        additional_data: Option<&NbtCompound>,
    ) -> Explosion {
        if additional_data.is_some_and(|data| data.contains_key("custom")) {
            self.observed_data.store(true, Ordering::SeqCst);
        }
        self.create_explosion(center, strength)
    }
}

struct FilteringBlockEntityHandler;

impl BlockHandler for FilteringBlockEntityHandler {
    fn key(&self) -> &Identifier {
        static KEY: std::sync::LazyLock<Identifier> =
            std::sync::LazyLock::new(|| Identifier::minecraft("filtered_chest"));
        &KEY
    }

    fn block_entity_tags(&self) -> Vec<Tag<Nbt>> {
        vec![Tag::nbt("allowed")]
    }

    fn block_entity_action(&self) -> i8 {
        BlockEntityType::TrappedChest.id() as i8
    }
}

struct CountingBlockHandler {
    key: Identifier,
    counts: Arc<BlockHandlerCounts>,
    tickable: bool,
}

struct ReplaceOnPlaceRule {
    source_block: Block,
    placed_block: Block,
}

struct NeighborUpdateRule {
    source_block: Block,
    updated_block: Block,
    max_distance: i32,
}

impl BlockHandler for CountingBlockHandler {
    fn key(&self) -> &Identifier {
        &self.key
    }

    fn on_place(&self, placement: BlockHandlerPlacement) {
        assert_eq!(placement.block(), Block::CHEST);
        self.counts.place_count.fetch_add(1, Ordering::SeqCst);
    }

    fn on_destroy(&self, destroy: BlockHandlerDestroy) {
        assert_eq!(destroy.block(), Block::CHEST);
        assert_eq!(destroy.new_block(), Block::AIR);
        self.counts.destroy_count.fetch_add(1, Ordering::SeqCst);
    }

    fn on_interact(&self, interaction: crate::world::BlockHandlerInteraction) -> bool {
        assert_eq!(interaction.block(), Block::CHEST);
        self.counts.interact_count.fetch_add(1, Ordering::SeqCst);
        false
    }

    fn on_touch(&self, touch: crate::world::BlockHandlerTouch) {
        assert_eq!(touch.block(), Block::CHEST);
        self.counts.touch_count.fetch_add(1, Ordering::SeqCst);
    }

    fn tick(&self, tick: BlockHandlerTick) {
        assert_eq!(tick.block(), Block::CHEST);
        self.counts.tick_count.fetch_add(1, Ordering::SeqCst);
    }

    fn is_tickable(&self) -> bool {
        self.tickable
    }
}

impl BlockPlacementRule for ReplaceOnPlaceRule {
    fn block(&self) -> Block {
        self.source_block
    }

    fn block_place(&self, placement: BlockPlacementState) -> Option<Block> {
        assert_eq!(placement.block(), self.source_block);
        Some(self.placed_block)
    }
}

impl BlockPlacementRule for NeighborUpdateRule {
    fn block(&self) -> Block {
        self.source_block
    }

    fn block_place(&self, placement: BlockPlacementState) -> Option<Block> {
        Some(placement.block())
    }

    fn block_update(&self, update: BlockUpdateState) -> Block {
        assert_eq!(update.current_block(), self.source_block);
        self.updated_block
    }

    fn max_update_distance(&self) -> i32 {
        self.max_distance
    }
}
