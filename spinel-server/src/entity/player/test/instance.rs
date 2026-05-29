use super::super::{Player, PlayerHand, PlayerSkin, QueuedPlayerPacket};
use crate::entity::player::{PlayerChunk, PlayerChunkTransition};
use crate::entity::{Entity, EntityPosition, EquipmentSlot};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket;
use spinel_core::network::clientbound::play::game_event::{GameEvent, GameEventPacket};
use spinel_core::network::clientbound::play::login_play::LoginPlayPacket;
use spinel_core::network::clientbound::play::player_abilities::PlayerAbilitiesPacket;
use spinel_core::network::clientbound::play::player_info_update::{
    PlayerInfoActions, PlayerInfoUpdatePacket,
};
use spinel_core::network::clientbound::play::recipe_book_add::RecipeBookAddPacket;
use spinel_core::network::clientbound::play::set_time::SetTimePacket;
use spinel_core::network::clientbound::play::start_configuration::StartConfigurationPacket;
use spinel_core::network::clientbound::play::sync_player_pos::SyncPlayerPositionPacket;
use spinel_core::network::clientbound::play::update_recipes::UpdateRecipesPacket;
use spinel_core::network::clientbound::play::{
    chunk_batch_finished::ChunkBatchFinishedPacket, chunk_batch_start::ChunkBatchStartPacket,
    chunk_data::ChunkDataAndUpdateLightPacket, set_chunk_cache_center::SetChunkCacheCenterPacket,
};
use spinel_network::types::chunk::ChunkData;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::light::LightData;
use spinel_network::types::{Identifier, TeleportFlags, Vector3d};
use spinel_network::{ConnectionState, DataType, PacketSender, VarIntWrapper};
use spinel_registry::{ItemStack, Material};
use spinel_utils::component::Component;
use spinel_utils::component::text::TextComponent;
use std::io::{Cursor, Read};
use std::net::TcpListener;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;
use uuid::Uuid;

#[test]
fn chunk_batch_acknowledgement_matches_minestom_target_and_lead_rules() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );

    assert_eq!(player.chunk_batch_lead(), 0);
    assert_eq!(player.max_chunk_batch_lead(), 1);
    assert_eq!(player.target_chunks_per_tick(), 9.0);

    player.on_chunk_batch_received(1000.0);

    assert_eq!(player.chunk_batch_lead(), -1);
    assert_eq!(player.max_chunk_batch_lead(), 10);
    assert_eq!(player.target_chunks_per_tick(), 64.0);

    player.on_chunk_batch_received(f32::NAN);

    assert_eq!(player.target_chunks_per_tick(), 0.01);
}

#[test]
fn queued_chunks_send_minestom_batch_packets_and_first_position_sync() {
    let (mut client, mut peer_stream) = test_client_pair();
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);
    player.send_chunk(empty_chunk_packet(0, 0));

    assert_eq!(player.queued_chunk_count(), 1);

    player.send_pending_chunks().unwrap();

    let (chunk_batch_start_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (chunk_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (chunk_batch_finished_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (sync_position_packet_id, _) = read_packet_frame(&mut peer_stream);

    assert_eq!(chunk_batch_start_packet_id, ChunkBatchStartPacket::get_id());
    assert_eq!(chunk_packet_id, ChunkDataAndUpdateLightPacket::get_id());
    assert_eq!(
        chunk_batch_finished_packet_id,
        ChunkBatchFinishedPacket::get_id()
    );
    assert_eq!(sync_position_packet_id, SyncPlayerPositionPacket::get_id());
    assert_eq!(player.queued_chunk_count(), 0);
    assert_eq!(player.chunk_batch_lead(), 1);
    assert_eq!(player.pending_chunk_count(), 8.0);
}

#[test]
fn enter_world_sends_minestom_chunk_batch_then_position_sync_sequence() {
    let (mut client, mut peer_stream) = test_client_pair();
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);

    player
        .enter_world(
            &mut client,
            20,
            Identifier::minecraft("overworld"),
            vec![empty_chunk_packet(0, 0)],
            SetTimePacket::new(42, 18000, true),
        )
        .unwrap();

    let packet_frames = read_available_packet_frames(&mut peer_stream);
    let packet_ids = packet_frames
        .iter()
        .map(|(packet_id, _)| *packet_id)
        .collect::<Vec<_>>();
    let chunk_batch_start_index = packet_ids
        .iter()
        .position(|packet_id| *packet_id == ChunkBatchStartPacket::get_id())
        .unwrap();
    let set_time_index = packet_ids
        .iter()
        .position(|packet_id| *packet_id == SetTimePacket::get_id())
        .unwrap();
    let game_event_index = packet_ids
        .iter()
        .position(|packet_id| *packet_id == GameEventPacket::get_id())
        .unwrap();
    let mut set_time_payload = Cursor::new(packet_frames[set_time_index].1.clone());

    assert_eq!(packet_ids[0], LoginPlayPacket::get_id());
    assert!(set_time_index < chunk_batch_start_index);
    assert_eq!(i64::decode(&mut set_time_payload).unwrap(), 42);
    assert_eq!(i64::decode(&mut set_time_payload).unwrap(), 18000);
    assert!(bool::decode(&mut set_time_payload).unwrap());
    assert_eq!(
        packet_ids[chunk_batch_start_index + 1],
        ChunkDataAndUpdateLightPacket::get_id()
    );
    assert_eq!(
        packet_ids[chunk_batch_start_index + 2],
        ChunkBatchFinishedPacket::get_id()
    );
    assert_eq!(
        packet_ids[chunk_batch_start_index + 3],
        SyncPlayerPositionPacket::get_id()
    );
    assert_eq!(
        packet_ids[chunk_batch_start_index + 4],
        SyncPlayerPositionPacket::get_id()
    );
    assert!(chunk_batch_start_index < game_event_index);
    assert!(player.has_entered_world());
    assert_eq!(player.queued_chunk_count(), 0);
    assert!(!player.needs_chunk_position_sync);
}

#[test]
fn queued_chunk_batch_finished_counts_only_sent_chunks() {
    let (mut client, mut peer_stream) = test_client_pair();
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);
    player.send_chunk(empty_chunk_packet(0, 0));
    player.send_chunk(empty_chunk_packet(1, 0));

    player
        .send_pending_chunks_with(&mut client, |queued_chunk| {
            Ok(
                (queued_chunk.chunk == PlayerChunk::new(1, 0)).then_some(empty_chunk_packet(
                    queued_chunk.chunk.x,
                    queued_chunk.chunk.z,
                )),
            )
        })
        .unwrap();

    let (chunk_batch_start_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (chunk_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (chunk_batch_finished_packet_id, chunk_batch_finished_payload) =
        read_packet_frame(&mut peer_stream);
    let mut chunk_batch_finished_payload = Cursor::new(chunk_batch_finished_payload);

    assert_eq!(chunk_batch_start_packet_id, ChunkBatchStartPacket::get_id());
    assert_eq!(chunk_packet_id, ChunkDataAndUpdateLightPacket::get_id());
    assert_eq!(
        chunk_batch_finished_packet_id,
        ChunkBatchFinishedPacket::get_id()
    );
    assert_eq!(
        VarIntWrapper::decode(&mut chunk_batch_finished_payload)
            .unwrap()
            .0,
        1
    );
    assert_eq!(player.pending_chunk_count(), 8.0);
    assert_eq!(player.queued_chunk_count(), 0);
}

#[test]
fn ordinary_chunk_border_crossing_sends_no_position_sync() {
    let (mut client, mut peer_stream) = test_client_pair();
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);
    player.needs_chunk_position_sync = false;

    player
        .move_to(
            &mut client,
            16.0,
            64.0,
            0.0,
            true,
            Some(PlayerChunkTransition {
                next: PlayerChunk::new(1, 0),
                arriving: vec![PlayerChunk::new(2, 0)],
                departing: Vec::new(),
            }),
            vec![empty_chunk_packet(2, 0)],
        )
        .unwrap();

    let (cache_center_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (chunk_batch_start_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (chunk_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (chunk_batch_finished_packet_id, _) = read_packet_frame(&mut peer_stream);
    peer_stream
        .set_read_timeout(Some(Duration::from_millis(25)))
        .unwrap();
    let mut trailing_packet_byte = [0u8; 1];

    assert_eq!(cache_center_packet_id, SetChunkCacheCenterPacket::get_id());
    assert_eq!(chunk_batch_start_packet_id, ChunkBatchStartPacket::get_id());
    assert_eq!(chunk_packet_id, ChunkDataAndUpdateLightPacket::get_id());
    assert_eq!(
        chunk_batch_finished_packet_id,
        ChunkBatchFinishedPacket::get_id()
    );
    assert!(peer_stream.read(&mut trailing_packet_byte).is_err());
    assert!(!player.needs_chunk_position_sync);
    assert_eq!(player.loaded_chunk, PlayerChunk::new(1, 0));
    assert_eq!(player.chunks_loaded_by_client, PlayerChunk::new(1, 0));
}

#[test]
fn ordinary_chunk_border_crossing_keeps_throttled_queue_state() {
    let (mut client, _peer_stream) = test_client_pair();
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);
    player.max_chunk_batch_lead = 0;
    player.needs_chunk_position_sync = false;
    player.send_chunk(empty_chunk_packet(8, 8));

    player
        .move_to(
            &mut client,
            16.0,
            64.0,
            0.0,
            true,
            Some(PlayerChunkTransition {
                next: PlayerChunk::new(1, 0),
                arriving: vec![PlayerChunk::new(2, 0)],
                departing: Vec::new(),
            }),
            vec![empty_chunk_packet(2, 0)],
        )
        .unwrap();

    assert_eq!(player.queued_chunk_count(), 2);
    assert!(
        player
            .chunk_queue
            .iter()
            .any(|queued_chunk| queued_chunk.chunk == PlayerChunk::new(8, 8))
    );
    assert_eq!(player.chunk_batch_lead(), 0);
    assert_eq!(player.pending_chunk_count(), 0.0);
    assert_eq!(player.target_chunks_per_tick(), 9.0);
    assert!(!player.needs_chunk_position_sync);
}

#[test]
fn ordinary_chunk_border_crossing_does_not_forget_unsent_queued_chunks() {
    let (mut client, mut peer_stream) = test_client_pair();
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);
    player.max_chunk_batch_lead = 0;
    player.needs_chunk_position_sync = false;
    player.send_chunk(empty_chunk_packet(0, 0));

    player
        .move_to(
            &mut client,
            16.0,
            64.0,
            0.0,
            true,
            Some(PlayerChunkTransition {
                next: PlayerChunk::new(1, 0),
                arriving: vec![PlayerChunk::new(2, 0)],
                departing: vec![PlayerChunk::new(0, 0)],
            }),
            vec![empty_chunk_packet(2, 0)],
        )
        .unwrap();

    let (cache_center_packet_id, _) = read_packet_frame(&mut peer_stream);
    let mut trailing_packet_byte = [0u8; 1];

    assert_eq!(cache_center_packet_id, SetChunkCacheCenterPacket::get_id());
    assert!(peer_stream.read(&mut trailing_packet_byte).is_err());
    assert_eq!(player.queued_chunk_count(), 1);
    assert!(
        player
            .chunk_queue
            .iter()
            .any(|queued_chunk| queued_chunk.chunk == PlayerChunk::new(2, 0))
    );
}

#[test]
fn sharp_turn_prunes_queued_chunks_outside_latest_view() {
    let (mut client, _peer_stream) = test_client_pair();
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);
    player.max_chunk_batch_lead = 0;
    player.needs_chunk_position_sync = false;

    player
        .move_to_loaded_chunks(
            &mut client,
            160.0,
            64.0,
            0.0,
            true,
            Some(PlayerChunkTransition {
                next: PlayerChunk::new(10, 0),
                arriving: vec![PlayerChunk::new(10, 0), PlayerChunk::new(11, 0)],
                departing: Vec::new(),
            }),
            vec![PlayerChunk::new(10, 0), PlayerChunk::new(11, 0)],
            1,
        )
        .unwrap();

    player
        .move_to_loaded_chunks(
            &mut client,
            0.0,
            64.0,
            0.0,
            true,
            Some(PlayerChunkTransition {
                next: PlayerChunk::new(0, 0),
                arriving: vec![PlayerChunk::new(0, 0), PlayerChunk::new(1, 0)],
                departing: vec![PlayerChunk::new(10, 0), PlayerChunk::new(11, 0)],
            }),
            vec![PlayerChunk::new(0, 0), PlayerChunk::new(1, 0)],
            1,
        )
        .unwrap();

    assert_eq!(player.chunks_loaded_by_client, PlayerChunk::new(0, 0));
    assert_eq!(player.queued_chunk_count(), 2);
    assert!(player.chunk_queue.iter().all(|queued_chunk| {
        queued_chunk
            .chunk
            .is_within_view_distance(PlayerChunk::new(0, 0), 2)
    }));
    assert!(
        player
            .chunk_queue
            .iter()
            .all(|queued_chunk| queued_chunk.chunk.x < 10)
    );
}

#[test]
fn stale_departing_chunk_inside_latest_view_is_not_forgotten() {
    let (mut client, mut peer_stream) = test_client_pair();
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);
    player.needs_chunk_position_sync = false;
    player.send_chunk(empty_chunk_packet(0, 0));
    player.send_pending_chunks().unwrap();
    let _ = read_packet_frame(&mut peer_stream);
    let _ = read_packet_frame(&mut peer_stream);
    let _ = read_packet_frame(&mut peer_stream);

    player
        .move_to_loaded_chunks(
            &mut client,
            16.0,
            64.0,
            0.0,
            true,
            Some(PlayerChunkTransition {
                next: PlayerChunk::new(1, 0),
                arriving: vec![PlayerChunk::new(2, 0)],
                departing: vec![PlayerChunk::new(0, 0)],
            }),
            vec![PlayerChunk::new(2, 0)],
            1,
        )
        .unwrap();

    let (cache_center_packet_id, _) = read_packet_frame(&mut peer_stream);
    let mut trailing_packet_byte = [0u8; 1];

    assert_eq!(cache_center_packet_id, SetChunkCacheCenterPacket::get_id());
    assert!(peer_stream.read(&mut trailing_packet_byte).is_err());
    assert!(player.client_sent_chunks.contains(&PlayerChunk::new(0, 0)));
}

#[test]
fn enabled_outbound_queue_buffers_play_packets_until_flush() {
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();

    client.send_packet(99, &[1, 2, 3]).unwrap();
    let mut trailing_packet_byte = [0u8; 1];

    assert_eq!(client.queued_outbound_packet_count(), 1);
    assert!(peer_stream.read(&mut trailing_packet_byte).is_err());

    assert_eq!(client.flush_outbound_packets().unwrap(), 1);
    let (packet_id, payload) = read_packet_frame(&mut peer_stream);

    assert_eq!(packet_id, 99);
    assert_eq!(payload, vec![1, 2, 3]);
    assert_eq!(client.queued_outbound_packet_count(), 0);
}

#[test]
fn play_disconnect_packet_bypasses_enabled_outbound_queue() {
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();

    client
        .send_packet(PlayDisconnectPacket::get_id(), &[0])
        .unwrap();
    let (packet_id, payload) = read_packet_frame(&mut peer_stream);

    assert_eq!(packet_id, PlayDisconnectPacket::get_id());
    assert_eq!(payload, vec![0]);
    assert_eq!(client.queued_outbound_packet_count(), 0);
}

#[test]
fn effective_chunk_view_distance_matches_minestom_client_world_cap_plus_one() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );

    assert_eq!(player.effective_chunk_view_distance(10), 9);

    player.set_client_chunk_view_distance(4);

    assert_eq!(player.client_chunk_view_distance(), 4);
    assert_eq!(player.effective_chunk_view_distance(10), 5);
    assert_eq!(player.effective_chunk_view_distance(2), 3);

    player.set_client_chunk_view_distance(-10);

    assert_eq!(player.client_chunk_view_distance(), 0);
    assert_eq!(player.effective_chunk_view_distance(10), 1);
}

#[test]
fn chunk_update_limiter_suppresses_recent_chunks_like_minestom() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let chunk = PlayerChunk::new(4, 5);

    assert!(player.chunk_update_limit_checker.add_to_history(chunk));
    assert!(!player.chunk_update_limit_checker.add_to_history(chunk));

    player.chunk_update_limit_checker.clear_history();

    assert!(player.chunk_update_limit_checker.add_to_history(chunk));
}

#[test]
fn player_game_mode_defaults_to_survival_and_can_be_set_during_configuration() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );

    assert_eq!(player.game_mode(), GameMode::Survival);
    assert!(player.set_game_mode(GameMode::Creative));
    assert_eq!(player.game_mode(), GameMode::Creative);
    assert!(player.can_fly());
    assert!(player.has_instant_break());
    assert!(player.is_invulnerable());
    assert!(!player.is_flying());
}

#[test]
fn active_player_game_mode_change_sends_client_game_mode_and_abilities() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let mut server = MinecraftServer::new();
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    client.server_ptr = Some(&mut server as *mut MinecraftServer as usize);
    player.set_client(&mut client);
    player.mark_entered_world();

    assert!(player.set_game_mode(GameMode::Creative));
    assert_eq!(player.game_mode(), GameMode::Creative);

    let (game_event_packet_id, game_event_payload) = read_packet_frame(&mut peer_stream);
    let game_event_packet = GameEventPacket::decode(&mut game_event_payload.as_slice()).unwrap();
    let (player_info_packet_id, player_info_payload) = read_packet_frame(&mut peer_stream);
    let (abilities_packet_id, abilities_payload) = read_packet_frame(&mut peer_stream);
    let (player_info_actions, player_info_uuid, player_info_game_mode) =
        decode_game_mode_player_info_update(player_info_payload);
    let abilities_packet =
        PlayerAbilitiesPacket::decode(&mut abilities_payload.as_slice()).unwrap();

    assert_eq!(game_event_packet_id, GameEventPacket::get_id());
    assert_eq!(
        game_event_packet.event,
        GameEvent::ChangeGameMode(GameMode::Creative).event_id()
    );
    assert_eq!(
        game_event_packet.value,
        GameEvent::ChangeGameMode(GameMode::Creative).value()
    );
    assert_eq!(player_info_packet_id, PlayerInfoUpdatePacket::get_id());
    assert_eq!(player_info_actions, PlayerInfoActions::update_game_mode());
    assert_eq!(player_info_uuid, player.uuid());
    assert_eq!(player_info_game_mode, GameMode::Creative);
    assert_eq!(abilities_packet_id, PlayerAbilitiesPacket::get_id());
    assert_eq!(
        abilities_packet.flags,
        PlayerAbilitiesPacket::INVULNERABLE
            | PlayerAbilitiesPacket::ALLOW_FLYING
            | PlayerAbilitiesPacket::INSTANT_BREAK
    );
    assert_eq!(abilities_packet.flying_speed, 0.05);
    assert_eq!(abilities_packet.field_view_modifier, 0.1);
}

#[test]
fn spectator_game_mode_enables_flying_like_minestom() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );

    assert!(player.set_game_mode(GameMode::Spectator));

    assert_eq!(player.game_mode(), GameMode::Spectator);
    assert!(player.can_fly());
    assert!(player.is_flying());
    assert!(!player.has_instant_break());
    assert!(player.is_invulnerable());
}

#[test]
fn player_identity_and_connection_getters_match_minestom_profile_surface() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565);
    let player = Player::new(Uuid::nil(), "Player".to_string(), 765, addr);

    assert_eq!(player.uuid(), Uuid::nil());
    assert_eq!(player.username(), "Player");
    assert_eq!(player.protocol_version(), 765);
    assert_eq!(player.address(), addr);
    assert!(player.can_pickup_item());
    assert!(!player.is_dead());
}

#[test]
fn player_pending_options_match_minestom_configuration_handoff_state() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let spawning_world = Uuid::from_u128(7);

    player.set_pending_options(spawning_world, true);

    assert_eq!(player.pending_spawning_world(), Some(spawning_world));
    assert!(player.is_hardcore());
}

#[test]
fn player_online_state_delegates_to_connection_like_minestom() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let (mut client, _peer_stream) = test_client_pair();

    player.set_client(&mut client);
    assert!(player.is_online());

    client.disconnect();

    assert!(!player.is_online());
}

#[test]
fn start_configuration_phase_requires_play_state_and_sends_transition_packet() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    player.set_client(&mut client);
    player.mark_entered_world();

    player.start_configuration_phase().unwrap();

    let (packet_id, payload) = read_packet_frame(&mut peer_stream);
    assert_eq!(packet_id, StartConfigurationPacket::get_id());
    assert!(payload.is_empty());
    assert_eq!(client.state, ConnectionState::Configuration);
    assert!(!player.has_entered_world());
}

#[test]
fn refresh_recipes_sends_empty_declare_and_recipe_book_reset_like_empty_minestom_manager() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    player.set_client(&mut client);

    player.refresh_recipes().unwrap();

    let (update_recipes_packet_id, update_recipes_payload) = read_packet_frame(&mut peer_stream);
    let (recipe_book_packet_id, recipe_book_payload) = read_packet_frame(&mut peer_stream);

    assert_eq!(update_recipes_packet_id, UpdateRecipesPacket::get_id());
    assert_eq!(update_recipes_payload, vec![0, 0]);
    assert_eq!(recipe_book_packet_id, RecipeBookAddPacket::get_id());
    assert_eq!(recipe_book_payload, vec![0, 1]);
}

#[test]
fn queued_player_packets_drain_at_minestom_packet_per_tick_limit() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let mut server = MinecraftServer::new();
    let (mut client, _peer_stream) = test_client_pair();

    (0..60).for_each(|_| {
        assert!(player.add_packet_to_queue(QueuedPlayerPacket::new(
            ConnectionState::Play,
            -1,
            Vec::new(),
        )));
    });

    let interpreted_packets = player.interpret_packet_queue(&mut server, &mut client);

    assert_eq!(interpreted_packets, 50);
    assert_eq!(player.queued_packet_count(), 10);
}

#[test]
fn server_queues_play_packets_for_player_tick_instead_of_dispatching_immediately() {
    let mut server = MinecraftServer::new();
    let (mut client, _peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    player.set_client(&mut client);
    assert!(
        server
            .world_manager
            .add_entity(world_uuid, Entity::Player(player))
    );

    assert!(server.queue_player_packet(-1, &mut client, vec![1, 2, 3]));

    let queued_player = server.world_manager.player_mut_for_client(&client).unwrap();
    assert_eq!(queued_player.queued_packet_count(), 1);
}

#[test]
fn queued_player_packet_overflow_kicks_with_minestom_reason() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    player.set_client(&mut client);

    (0..1000).for_each(|_| {
        assert!(player.add_packet_to_queue(QueuedPlayerPacket::new(
            ConnectionState::Play,
            -1,
            Vec::new(),
        )));
    });

    assert!(!player.add_packet_to_queue(QueuedPlayerPacket::new(
        ConnectionState::Play,
        -1,
        Vec::new(),
    )));

    let (packet_id, payload) = read_packet_frame(&mut peer_stream);
    let packet = spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket::decode(
        &mut payload.as_slice(),
    )
    .unwrap();
    let expected_reason = TextComponent::literal_with_color(
        "Too Many Packets",
        spinel_utils::component::color::TextColor::from_named(
            spinel_utils::component::color::NamedTextColor::Red,
        ),
    );
    assert_eq!(
        packet_id,
        spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket::get_id()
    );
    assert_eq!(packet.reason, expected_reason);
    assert!(!client.is_online());
}

#[test]
fn player_kill_sets_dead_state_and_sends_death_screen_without_dropping_items() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    player.set_client(&mut client);
    player
        .inventory()
        .set_item_stack(9, ItemStack::of(Material::STONE));

    player.kill().unwrap();

    let (packet_id, _payload) = read_packet_frame(&mut peer_stream);
    assert_eq!(
        packet_id,
        spinel_core::network::clientbound::play::player_combat_kill::PlayerCombatKillPacket::get_id(
        )
    );
    assert!(player.is_dead());
    assert_eq!(
        player.inventory_ref().item_stack(9),
        Some(&ItemStack::of(Material::STONE))
    );
}

#[test]
fn player_respawn_is_noop_when_alive_and_refreshes_client_state_when_dead() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    player.set_client(&mut client);
    player.mark_entered_world();

    assert!(!player.respawn().unwrap());

    player.kill().unwrap();
    let _ = read_packet_frame(&mut peer_stream);

    assert!(player.respawn().unwrap());

    let (respawn_packet_id, _respawn_payload) = read_packet_frame(&mut peer_stream);
    let (game_event_packet_id, _game_event_payload) = read_packet_frame(&mut peer_stream);
    let (difficulty_packet_id, _difficulty_payload) = read_packet_frame(&mut peer_stream);
    let (health_packet_id, _health_payload) = read_packet_frame(&mut peer_stream);
    let (experience_packet_id, _experience_payload) = read_packet_frame(&mut peer_stream);
    let (abilities_packet_id, _abilities_payload) = read_packet_frame(&mut peer_stream);

    assert_eq!(
        respawn_packet_id,
        spinel_core::network::clientbound::play::respawn::RespawnPacket::get_id()
    );
    assert_eq!(game_event_packet_id, GameEventPacket::get_id());
    assert_eq!(
        difficulty_packet_id,
        spinel_core::network::clientbound::play::server_difficulty::ServerDifficultyPacket::get_id(
        )
    );
    assert_eq!(
        health_packet_id,
        spinel_core::network::clientbound::play::set_health::SetHealthPacket::get_id()
    );
    assert_eq!(
        experience_packet_id,
        spinel_core::network::clientbound::play::set_experience::SetExperiencePacket::get_id()
    );
    assert_eq!(abilities_packet_id, PlayerAbilitiesPacket::get_id());
    assert!(!player.is_dead());
}

#[test]
fn teleport_chunk_sync_sends_chunk_updates_before_position_sync() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    player.set_client(&mut client);
    player.mark_entered_world();
    player.needs_chunk_position_sync = false;

    player
        .move_to(
            &mut client,
            16.0,
            64.0,
            0.0,
            true,
            Some(PlayerChunkTransition {
                next: PlayerChunk::new(1, 0),
                arriving: vec![PlayerChunk::new(2, 0)],
                departing: Vec::new(),
            }),
            vec![empty_chunk_packet(2, 0)],
        )
        .unwrap();
    player
        .synchronize_position_after_teleport(
            EntityPosition::new(16.0, 64.0, 0.0, 0.0, 0.0),
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            TeleportFlags::absolute(),
            true,
        )
        .unwrap();

    let (cache_center_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (chunk_batch_start_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (chunk_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (chunk_batch_finished_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (sync_position_packet_id, _) = read_packet_frame(&mut peer_stream);

    assert_eq!(cache_center_packet_id, SetChunkCacheCenterPacket::get_id());
    assert_eq!(chunk_batch_start_packet_id, ChunkBatchStartPacket::get_id());
    assert_eq!(chunk_packet_id, ChunkDataAndUpdateLightPacket::get_id());
    assert_eq!(
        chunk_batch_finished_packet_id,
        ChunkBatchFinishedPacket::get_id()
    );
    assert_eq!(sync_position_packet_id, SyncPlayerPositionPacket::get_id());
}

#[test]
fn respawn_chunk_reload_sends_respawn_state_then_chunk_batch_and_position_sync() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    player.set_client(&mut client);
    player.mark_entered_world();
    player.kill().unwrap();
    let _ = read_packet_frame(&mut peer_stream);

    assert!(player.respawn().unwrap());
    player.reset_chunk_queue_for_world_entry_or_teleport();
    player.send_chunk(empty_chunk_packet(0, 0));
    player.send_pending_chunks().unwrap();

    let (respawn_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (game_event_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (difficulty_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (health_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (experience_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (abilities_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (chunk_batch_start_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (chunk_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (chunk_batch_finished_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (sync_position_packet_id, _) = read_packet_frame(&mut peer_stream);

    assert_eq!(
        respawn_packet_id,
        spinel_core::network::clientbound::play::respawn::RespawnPacket::get_id()
    );
    assert_eq!(game_event_packet_id, GameEventPacket::get_id());
    assert_eq!(
        difficulty_packet_id,
        spinel_core::network::clientbound::play::server_difficulty::ServerDifficultyPacket::get_id(
        )
    );
    assert_eq!(
        health_packet_id,
        spinel_core::network::clientbound::play::set_health::SetHealthPacket::get_id()
    );
    assert_eq!(
        experience_packet_id,
        spinel_core::network::clientbound::play::set_experience::SetExperiencePacket::get_id()
    );
    assert_eq!(abilities_packet_id, PlayerAbilitiesPacket::get_id());
    assert_eq!(chunk_batch_start_packet_id, ChunkBatchStartPacket::get_id());
    assert_eq!(chunk_packet_id, ChunkDataAndUpdateLightPacket::get_id());
    assert_eq!(
        chunk_batch_finished_packet_id,
        ChunkBatchFinishedPacket::get_id()
    );
    assert_eq!(sync_position_packet_id, SyncPlayerPositionPacket::get_id());
}

#[test]
fn player_skin_is_state_only_before_world_entry_and_is_used_in_player_info() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let skin = PlayerSkin::new("texture-data", Some("signature-data".to_string()));

    player.set_skin(Some(skin)).unwrap();
    let player_info_packet = player.player_info_packet();

    assert_eq!(player.skin().unwrap().textures(), "texture-data");
    assert_eq!(player.skin().unwrap().signature(), Some("signature-data"));
    assert_eq!(player_info_packet.entries.0[0].properties.len(), 1);
    assert_eq!(
        player_info_packet.entries.0[0].properties[0].name,
        "textures"
    );
    assert_eq!(
        player_info_packet.entries.0[0].properties[0]
            .signature
            .as_deref(),
        Some("signature-data")
    );
}

#[test]
fn player_display_name_is_state_only_before_world_entry() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );

    player
        .set_display_name(Some(Component::text("Display").build()))
        .unwrap();

    assert!(player.display_name().is_some());
}

#[test]
fn active_player_display_name_syncs_client_player_info() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    player.set_client(&mut client);
    player.mark_entered_world();

    player
        .set_display_name(Some(Component::text("Display").build()))
        .unwrap();

    let (packet_id, payload) = read_packet_frame(&mut peer_stream);
    let packet = PlayerInfoUpdatePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(packet_id, PlayerInfoUpdatePacket::get_id());
    assert_eq!(packet.actions, PlayerInfoActions::update_display_name());
    assert_eq!(packet.entries.0[0].uuid, player.uuid());
    assert!(packet.entries.0[0].display_name.is_some());
}

#[test]
fn player_teleport_ids_match_minestom_sent_and_received_tracking() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );

    assert_eq!(player.last_sent_teleport_id(), 0);
    assert_eq!(player.last_received_teleport_id(), 0);
    assert_eq!(player.next_teleport_id(), 1);
    assert!(player.has_pending_teleport_confirmation());
    player.set_last_received_teleport_id(-1);
    assert_eq!(player.last_received_teleport_id(), 0);
    player.set_last_received_teleport_id(1);
    assert_eq!(player.last_received_teleport_id(), 1);
    assert!(!player.has_pending_teleport_confirmation());
}

#[test]
fn active_player_synchronize_position_after_teleport_sends_sync_packet() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    player.set_client(&mut client);

    player
        .synchronize_position_after_teleport(
            EntityPosition::new(1.0, 2.0, 3.0, 90.0, 45.0),
            Vector3d {
                x: 0.1,
                y: 0.2,
                z: 0.3,
            },
            TeleportFlags::from_bitmask(TeleportFlags::X | TeleportFlags::Y),
            true,
        )
        .unwrap();

    let (packet_id, payload) = read_packet_frame(&mut peer_stream);
    let packet = SyncPlayerPositionPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(packet_id, SyncPlayerPositionPacket::get_id());
    assert_eq!(packet.teleport_id, 1);
    assert_eq!(packet.x, 1.0);
    assert_eq!(packet.velocity_z, 0.3);
    assert_eq!(packet.flags.bitmask(), TeleportFlags::X | TeleportFlags::Y);
    assert_eq!(player.last_sent_teleport_id(), 1);
    assert!(player.has_pending_teleport_confirmation());
}

#[test]
fn player_synchronize_position_without_confirmation_uses_negative_id() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    player.set_client(&mut client);

    player
        .synchronize_position_after_teleport(
            EntityPosition::new(1.0, 2.0, 3.0, 90.0, 45.0),
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            TeleportFlags::absolute(),
            false,
        )
        .unwrap();

    let (_, payload) = read_packet_frame(&mut peer_stream);
    let packet = SyncPlayerPositionPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(packet.teleport_id, -1);
    assert_eq!(player.last_sent_teleport_id(), 0);
    assert!(!player.has_pending_teleport_confirmation());
}

#[test]
fn player_listed_latency_and_ability_setters_sync_active_client_state() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    player.set_client(&mut client);
    player.mark_entered_world();

    player.set_listed(false).unwrap();
    player.refresh_latency(42).unwrap();
    player.set_allow_flying(true).unwrap();
    player.set_flying(true).unwrap();
    player.set_instant_break(true).unwrap();
    player.set_invulnerable(true).unwrap();
    player.set_flying_speed(0.2).unwrap();
    player.set_field_view_modifier(0.3).unwrap();

    let (_, listed_payload) = read_packet_frame(&mut peer_stream);
    let (_, _, listed) = decode_listed_player_info_update(listed_payload);
    let (_, latency_payload) = read_packet_frame(&mut peer_stream);
    let (_, _, latency) = decode_latency_player_info_update(latency_payload);
    let (_, allow_flying_payload) = read_packet_frame(&mut peer_stream);
    let allow_flying_packet =
        PlayerAbilitiesPacket::decode(&mut allow_flying_payload.as_slice()).unwrap();
    let (_, flying_payload) = read_packet_frame(&mut peer_stream);
    let flying_packet = PlayerAbilitiesPacket::decode(&mut flying_payload.as_slice()).unwrap();
    let (_, instant_break_payload) = read_packet_frame(&mut peer_stream);
    let instant_break_packet =
        PlayerAbilitiesPacket::decode(&mut instant_break_payload.as_slice()).unwrap();
    let (_, invulnerable_payload) = read_packet_frame(&mut peer_stream);
    let invulnerable_packet =
        PlayerAbilitiesPacket::decode(&mut invulnerable_payload.as_slice()).unwrap();
    let (_, flying_speed_payload) = read_packet_frame(&mut peer_stream);
    let flying_speed_packet =
        PlayerAbilitiesPacket::decode(&mut flying_speed_payload.as_slice()).unwrap();
    let (_, field_view_payload) = read_packet_frame(&mut peer_stream);
    let field_view_packet =
        PlayerAbilitiesPacket::decode(&mut field_view_payload.as_slice()).unwrap();

    assert!(!player.is_listed());
    assert_eq!(player.latency(), 42);
    assert!(player.can_fly());
    assert!(player.is_flying());
    assert!(player.has_instant_break());
    assert!(player.is_invulnerable());
    assert_eq!(player.flying_speed(), 0.2);
    assert_eq!(player.field_view_modifier(), 0.3);
    assert!(!listed);
    assert_eq!(latency, 42);
    assert_eq!(
        allow_flying_packet.flags,
        PlayerAbilitiesPacket::ALLOW_FLYING
    );
    assert_eq!(
        flying_packet.flags,
        PlayerAbilitiesPacket::FLYING | PlayerAbilitiesPacket::ALLOW_FLYING
    );
    assert_eq!(
        instant_break_packet.flags,
        PlayerAbilitiesPacket::FLYING
            | PlayerAbilitiesPacket::ALLOW_FLYING
            | PlayerAbilitiesPacket::INSTANT_BREAK
    );
    assert_eq!(
        invulnerable_packet.flags,
        PlayerAbilitiesPacket::INVULNERABLE
            | PlayerAbilitiesPacket::FLYING
            | PlayerAbilitiesPacket::ALLOW_FLYING
            | PlayerAbilitiesPacket::INSTANT_BREAK
    );
    assert_eq!(flying_speed_packet.flying_speed, 0.2);
    assert_eq!(field_view_packet.field_view_modifier, 0.3);
}

#[test]
fn swap_item_hands_matches_minestom_player_action_swap() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_item_in_hand(PlayerHand::Main, ItemStack::of(Material::DIAMOND));
    player.set_item_in_hand(PlayerHand::Off, ItemStack::of(Material::EMERALD));
    let mut server = MinecraftServer::new();
    let mut client = test_client();

    assert!(player.swap_item_hands(&mut server, &mut client));
    assert_eq!(
        player.item_in_hand(PlayerHand::Main).material(),
        &Material::EMERALD
    );
    assert_eq!(
        player.item_in_hand(PlayerHand::Off).material(),
        &Material::DIAMOND
    );
}

#[test]
fn player_equipment_packet_includes_full_minestom_equipment_set() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_equipment(
        EquipmentSlot::MainHand,
        ItemStack::of(Material::DIAMOND_SWORD),
    );
    player.set_equipment(
        EquipmentSlot::Helmet,
        ItemStack::of(Material::DIAMOND_HELMET),
    );

    let equipment_packet = player.visible_equipment_packet();

    assert_eq!(equipment_packet.entity_id, player.entity_id().value());
    assert_eq!(equipment_packet.equipment.0.len(), 7);
    assert_eq!(
        equipment_packet.equipment.0[0]
            .item
            .to_item_stack()
            .material(),
        &Material::DIAMOND_SWORD
    );
    assert_eq!(
        equipment_packet.equipment.0[5]
            .item
            .to_item_stack()
            .material(),
        &Material::DIAMOND_HELMET
    );
}

#[test]
fn player_exposes_connected_client_like_minestom_player_connection() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let mut client = test_client();

    player.set_client(&mut client);

    assert_eq!(player.client().map(|client| client.addr), Some(client.addr));
}

#[test]
fn player_input_sprint_does_not_set_sprinting_metadata() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );

    assert!(!player.refresh_input(false, false, false, false, false, false, true));
    assert_eq!(
        player.metadata_packet().entries.0[0].value,
        MetadataValue::Byte(0)
    );
    assert!(player.set_sprinting(true));
    assert_eq!(
        player.metadata_packet().entries.0[0].value,
        MetadataValue::Byte(8)
    );
    assert!(player.is_sprinting());
}

#[test]
fn player_sneaking_api_matches_minestom_flying_pose_edge() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );

    player.set_sneaking(true);

    assert!(player.is_sneaking());
    assert!(
        player
            .metadata_packet()
            .entries
            .0
            .iter()
            .any(|entry| entry.value == MetadataValue::Pose(5))
    );
    player.set_sneaking(false);
    player.set_flying_state(true);
    player.set_sneaking(true);

    assert!(player.is_sneaking());
    assert!(
        !player
            .metadata_packet()
            .entries
            .0
            .iter()
            .any(|entry| entry.value == MetadataValue::Pose(5))
    );
}

#[test]
fn player_item_cooldown_uses_extracted_material_default() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_item_in_hand(PlayerHand::Main, ItemStack::of(Material::ENDER_PEARL));
    let mut client = test_client();

    assert!(
        player
            .use_item_with_cooldown(PlayerHand::Main, 10, &mut client)
            .unwrap()
    );
    assert!(player.item_cooldown_is_active("minecraft:ender_pearl", 29));
    assert!(!player.item_cooldown_is_active("minecraft:ender_pearl", 30));
    assert!(
        !player
            .use_item_with_cooldown(PlayerHand::Main, 11, &mut client)
            .unwrap()
    );
}

#[test]
fn player_item_use_state_matches_minestom_timing_and_eating_checks() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );

    assert!(!player.is_using_item());
    assert!(!player.is_eating());
    assert_eq!(player.current_item_use_time(), 0);

    player.set_item_in_hand(PlayerHand::Main, ItemStack::of(Material::POTION));
    player.refresh_item_use(Some(PlayerHand::Main), 2);

    assert!(player.is_using_item());
    assert!(player.is_eating());
    assert_eq!(player.item_use_hand(), Some(PlayerHand::Main));
    assert_eq!(player.current_item_use_time(), 0);
    assert!(player.tick().is_none());
    assert_eq!(player.current_item_use_time(), 1);
    let item_use_completion = player.tick().unwrap();
    assert_eq!(item_use_completion.entity_id, player.entity_id().value());
    assert_eq!(item_use_completion.status, 9);
    assert!(!player.is_using_item());
    assert_eq!(player.current_item_use_time(), 0);
}

fn test_client() -> Client {
    test_client_pair().0
}

fn test_client_pair() -> (Client, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = std::net::TcpStream::connect(addr).unwrap();
    let (peer_stream, _) = listener.accept().unwrap();
    peer_stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    (Client::new(stream, addr), peer_stream)
}

fn read_packet_frame(peer_stream: &mut TcpStream) -> (i32, Vec<u8>) {
    let frame_length = VarIntWrapper::decode(peer_stream).unwrap().0 as usize;
    let mut frame = vec![0; frame_length];
    peer_stream.read_exact(&mut frame).unwrap();
    let mut frame_cursor = Cursor::new(frame);
    let packet_id = VarIntWrapper::decode(&mut frame_cursor).unwrap().0;
    let payload_start = frame_cursor.position() as usize;
    let payload = frame_cursor.into_inner()[payload_start..].to_vec();
    (packet_id, payload)
}

fn read_available_packet_frames(peer_stream: &mut TcpStream) -> Vec<(i32, Vec<u8>)> {
    peer_stream
        .set_read_timeout(Some(Duration::from_millis(25)))
        .unwrap();
    let mut packet_frames = Vec::new();
    loop {
        let frame_length = match VarIntWrapper::decode(peer_stream) {
            Ok(frame_length) => frame_length.0 as usize,
            Err(error)
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut =>
            {
                break;
            }
            Err(error) => panic!("packet frame length decode failed: {error}"),
        };
        let mut frame = vec![0; frame_length];
        peer_stream.read_exact(&mut frame).unwrap();
        let mut frame_cursor = Cursor::new(frame);
        let packet_id = VarIntWrapper::decode(&mut frame_cursor).unwrap().0;
        let payload_start = frame_cursor.position() as usize;
        let payload = frame_cursor.into_inner()[payload_start..].to_vec();
        packet_frames.push((packet_id, payload));
    }
    packet_frames
}

fn decode_game_mode_player_info_update(payload: Vec<u8>) -> (PlayerInfoActions, Uuid, GameMode) {
    let mut payload = Cursor::new(payload);
    let actions = PlayerInfoActions(u8::decode(&mut payload).unwrap());
    assert_eq!(VarIntWrapper::decode(&mut payload).unwrap().0, 1);
    let uuid = Uuid::decode(&mut payload).unwrap();
    let game_mode_id = VarIntWrapper::decode(&mut payload).unwrap().0 as u8;
    let game_mode = GameMode::from_id(game_mode_id).unwrap();
    (actions, uuid, game_mode)
}

fn decode_listed_player_info_update(payload: Vec<u8>) -> (PlayerInfoActions, Uuid, bool) {
    let mut payload = Cursor::new(payload);
    let actions = PlayerInfoActions(u8::decode(&mut payload).unwrap());
    assert_eq!(VarIntWrapper::decode(&mut payload).unwrap().0, 1);
    let uuid = Uuid::decode(&mut payload).unwrap();
    let listed = bool::decode(&mut payload).unwrap();
    (actions, uuid, listed)
}

fn decode_latency_player_info_update(payload: Vec<u8>) -> (PlayerInfoActions, Uuid, i32) {
    let mut payload = Cursor::new(payload);
    let actions = PlayerInfoActions(u8::decode(&mut payload).unwrap());
    assert_eq!(VarIntWrapper::decode(&mut payload).unwrap().0, 1);
    let uuid = Uuid::decode(&mut payload).unwrap();
    let latency = VarIntWrapper::decode(&mut payload).unwrap().0;
    (actions, uuid, latency)
}

fn empty_chunk_packet(chunk_x: i32, chunk_z: i32) -> ChunkDataAndUpdateLightPacket {
    ChunkDataAndUpdateLightPacket::with_light_data(
        chunk_x,
        chunk_z,
        ChunkData {
            heightmaps: Vec::new(),
            sections: Vec::new(),
            block_entities: Vec::new(),
        },
        LightData {
            sky_light_mask: Vec::new(),
            block_light_mask: Vec::new(),
            empty_sky_light_mask: Vec::new(),
            empty_block_light_mask: Vec::new(),
            sky_light_arrays: Vec::new(),
            block_light_arrays: Vec::new(),
        },
    )
}
