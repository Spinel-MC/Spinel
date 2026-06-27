use super::super::{
    BelowNameTag, Player, PlayerHand, PlayerMessageType, PlayerSkin, QueuedPlayerPacket,
};
use crate::entity::EntityPose;
use crate::entity::player::ResourcePackRequest;
use crate::entity::player::{PlayerChunk, PlayerChunkTransition};
use crate::entity::{
    Entity, EntityId, EntityPosition, EquipmentSlot, GenericEntity, PlayerSpawnPoint,
};
use crate::events::player_game_mode_change::PlayerGameModeChangeEvent;
use crate::events::player_respawn::PlayerRespawnEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::{
    BossBar, Weather, World, WorldBorder, WorldBossBarColor, WorldBossBarOverlay, WorldManager,
};
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::configuration::resource_pack_pop::ResourcePackPopPacket as ConfigurationResourcePackPopPacket;
use spinel_core::network::clientbound::configuration::resource_pack_push::ResourcePackPushPacket as ConfigurationResourcePackPushPacket;
use spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket;
use spinel_core::network::clientbound::play::forget_level_chunk::ForgetLevelChunkPacket;
use spinel_core::network::clientbound::play::game_event::{GameEvent, GameEventPacket};
use spinel_core::network::clientbound::play::initialize_world_border::InitializeWorldBorderPacket;
use spinel_core::network::clientbound::play::login_play::LoginPlayPacket;
use spinel_core::network::clientbound::play::player_abilities::PlayerAbilitiesPacket;
use spinel_core::network::clientbound::play::player_info_remove::PlayerInfoRemovePacket;
use spinel_core::network::clientbound::play::player_info_update::{
    PlayerInfoActions, PlayerInfoUpdatePacket,
};
use spinel_core::network::clientbound::play::recipe_book_add::RecipeBookAddPacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::respawn::RespawnPacket;
use spinel_core::network::clientbound::play::set_equipment::SetEquipmentPacket;
use spinel_core::network::clientbound::play::set_experience::SetExperiencePacket;
use spinel_core::network::clientbound::play::set_health::SetHealthPacket;
use spinel_core::network::clientbound::play::set_player_inventory::SetPlayerInventoryPacket;
use spinel_core::network::clientbound::play::set_time::SetTimePacket;
use spinel_core::network::clientbound::play::spawn_entity::SpawnEntityPacket;
use spinel_core::network::clientbound::play::start_configuration::StartConfigurationPacket;
use spinel_core::network::clientbound::play::sync_player_pos::SyncPlayerPositionPacket;
use spinel_core::network::clientbound::play::update_attributes::UpdateAttributesPacket;
use spinel_core::network::clientbound::play::update_recipes::UpdateRecipesPacket;
use spinel_core::network::clientbound::play::{
    advancements::{AdvancementFrameType, AdvancementsPacket, Notification},
    boss_bar::BossBarPacket,
    chunk_batch_finished::ChunkBatchFinishedPacket,
    chunk_batch_start::ChunkBatchStartPacket,
    chunk_data::ChunkDataAndUpdateLightPacket,
    clear_dialog::ClearDialogPacket,
    clear_titles::ClearTitlesPacket,
    display_scoreboard::DisplayScoreboardPacket,
    entity_sound_effect::EntitySoundEffectPacket,
    entity_status::EntityStatusPacket,
    player_look_at::{FacePoint, PlayerLookAtPacket},
    plugin_message::PlayCustomPayloadPacket,
    resource_pack_pop::ResourcePackPopPacket,
    resource_pack_push::ResourcePackPushPacket,
    scoreboard_objective::ScoreboardObjectivePacket,
    set_camera::SetCameraPacket,
    set_chunk_cache_center::SetChunkCacheCenterPacket,
    set_entity_data::SetEntityDataPacket,
    set_subtitle_text::SetSubtitleTextPacket,
    set_title_text::SetTitleTextPacket,
    set_titles_animation::SetTitlesAnimationPacket,
    show_dialog::ShowDialogPacket,
    sound_effect::SoundEffectPacket,
    stop_sound::StopSoundPacket,
    system_chat::SystemChatPacket,
    tab_list::TabListPacket,
    world_event::WorldEventPacket,
};
use spinel_core::network::resource_pack::{ResourcePackInfo, ResourcePackStatus};
use spinel_macros::event_listener;
use spinel_network::types::chunk::ChunkData;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::light::LightData;
use spinel_network::types::sound::SoundEvent;
use spinel_network::types::{
    ClientInformation, Identifier, MainHand, Particle, Position, TeleportFlags, Vector3d, Velocity,
};
use spinel_network::{ConnectionState, DataType, PacketSender, VarIntWrapper};
use spinel_registry::dialog::Dialog;
use spinel_registry::{EntityType, ItemStack, Material};
use spinel_utils::component::Component;
use spinel_utils::component::events::HoverEvent;
use spinel_utils::component::text::TextComponent;
use spinel_utils::component::variant::ComponentType;
use std::collections::BTreeSet;
use std::io::{Cursor, Read};
use std::net::TcpListener;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
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

    assert_eq!(player.get_chunk_batch_lead(), 0);
    assert_eq!(player.get_max_chunk_batch_lead(), 1);
    assert_eq!(player.get_target_chunks_per_tick(), 9.0);

    player.on_chunk_batch_received(1000.0);

    assert_eq!(player.get_chunk_batch_lead(), -1);
    assert_eq!(player.get_max_chunk_batch_lead(), 10);
    assert_eq!(player.get_target_chunks_per_tick(), 64.0);

    player.on_chunk_batch_received(f32::NAN);

    assert_eq!(player.get_target_chunks_per_tick(), 0.01);
}

#[test]
fn chunk_queue_reset_preserves_minestom_batch_lead_state() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.send_chunk(empty_chunk_packet(0, 0));
    player.chunk_batch_lead = 4;
    player.max_chunk_batch_lead = 10;
    player.target_chunks_per_tick = 32.0;
    player.pending_chunk_count = 12.0;
    player.needs_chunk_position_sync = false;

    player.reset_chunk_queue();

    assert_eq!(player.get_queued_chunk_count(), 0);
    assert_eq!(player.get_chunk_batch_lead(), 4);
    assert_eq!(player.get_max_chunk_batch_lead(), 10);
    assert_eq!(player.get_target_chunks_per_tick(), 9.0);
    assert_eq!(player.get_pending_chunk_count(), 0.0);
    assert!(player.needs_chunk_position_sync);
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

    assert_eq!(player.get_queued_chunk_count(), 1);

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
    assert_eq!(player.get_queued_chunk_count(), 0);
    assert_eq!(player.get_chunk_batch_lead(), 1);
    assert_eq!(player.get_pending_chunk_count(), 8.0);
}

#[test]
fn offline_player_does_not_prepare_or_send_buffered_chunks() {
    let (mut client, _peer_stream) = test_client_pair();
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);
    player.send_chunk(empty_chunk_packet(0, 0));
    client.close_connection();
    let mut prepared_chunk_count = 0;

    player
        .send_pending_chunks_with(&mut client, |queued_chunk| {
            prepared_chunk_count += 1;
            Ok(queued_chunk.packet)
        })
        .unwrap();

    assert_eq!(prepared_chunk_count, 0);
    assert_eq!(player.get_queued_chunk_count(), 0);
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
            0,
            Identifier::minecraft("overworld"),
            vec![empty_chunk_packet(0, 0)],
            WorldBorder::DEFAULT.initialize_packet(WorldBorder::DEFAULT.diameter(), 0),
            SetTimePacket::new(42, 18000, true),
            Weather::THUNDER,
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
    let world_border_index = packet_ids
        .iter()
        .position(|packet_id| *packet_id == InitializeWorldBorderPacket::get_id())
        .unwrap();
    let weather_events = packet_frames
        .iter()
        .filter(|(packet_id, _)| *packet_id == GameEventPacket::get_id())
        .map(|(_, payload)| GameEventPacket::decode(&mut payload.as_slice()).unwrap())
        .collect::<Vec<_>>();
    let waiting_for_chunks_event_index = packet_frames
        .iter()
        .enumerate()
        .find_map(|(packet_index, (packet_id, payload))| {
            if *packet_id != GameEventPacket::get_id() {
                return None;
            }
            let game_event_packet = GameEventPacket::decode(&mut payload.as_slice()).unwrap();
            (game_event_packet.event == GameEvent::StartWaitingForLevelChunks.event_id())
                .then_some(packet_index)
        })
        .unwrap();
    let mut set_time_payload = Cursor::new(packet_frames[set_time_index].1.clone());

    assert_eq!(packet_ids[0], LoginPlayPacket::get_id());
    assert!(world_border_index < set_time_index);
    assert!(set_time_index < game_event_index);
    assert_eq!(weather_events[0].event, GameEvent::BeginRaining.event_id());
    assert_eq!(weather_events[0].value, GameEvent::BeginRaining.value());
    assert_eq!(
        weather_events[1].event,
        GameEvent::RainLevelChange(1.0).event_id()
    );
    assert_eq!(weather_events[1].value, 1.0);
    assert_eq!(
        weather_events[2].event,
        GameEvent::ThunderLevelChange(1.0).event_id()
    );
    assert_eq!(weather_events[2].value, 1.0);
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
    assert!(chunk_batch_start_index < waiting_for_chunks_event_index);
    assert!(player.has_entered_world());
    assert_eq!(player.get_queued_chunk_count(), 0);
    assert!(!player.needs_chunk_position_sync);
}

#[test]
fn first_spawn_inventory_sync_waits_until_client_starts_waiting_for_chunks() {
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);
    assert!(
        player
            .get_inventory()
            .set_item_stack(0, ItemStack::of(Material::DIAMOND_PICKAXE))
    );

    player
        .spawn_after_instance_transition(
            &mut client,
            Identifier::minecraft("overworld"),
            vec![PlayerChunk::new(0, 0)],
            SetTimePacket::new(42, 18000, true),
            WorldBorder::DEFAULT.initialize_packet(WorldBorder::DEFAULT.diameter(), 0),
            Weather::CLEAR,
            true,
            false,
            true,
        )
        .unwrap();

    let packet_frames = read_available_packet_frames(&mut peer_stream);
    let packet_ids = packet_frames
        .iter()
        .map(|(packet_id, _)| *packet_id)
        .collect::<Vec<_>>();
    let waiting_for_chunks_event_index = packet_frames
        .iter()
        .enumerate()
        .find_map(|(packet_index, (packet_id, payload))| {
            if *packet_id != GameEventPacket::get_id() {
                return None;
            }
            let game_event_packet = GameEventPacket::decode(&mut payload.as_slice()).unwrap();
            (game_event_packet.event == GameEvent::StartWaitingForLevelChunks.event_id())
                .then_some(packet_index)
        })
        .unwrap();
    let player_inventory_index = packet_ids
        .iter()
        .position(|packet_id| *packet_id == SetPlayerInventoryPacket::get_id())
        .unwrap();

    assert!(waiting_for_chunks_event_index < player_inventory_index);
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
    assert_eq!(player.get_pending_chunk_count(), 8.0);
    assert_eq!(player.get_queued_chunk_count(), 0);
}

#[test]
fn unavailable_queued_chunks_still_finish_an_empty_minestom_batch() {
    let (mut client, mut peer_stream) = test_client_pair();
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);
    player.send_chunk(empty_chunk_packet(0, 0));

    player
        .send_pending_chunks_with(&mut client, |_| Ok(None))
        .unwrap();

    let (chunk_batch_start_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (chunk_batch_finished_packet_id, chunk_batch_finished_payload) =
        read_packet_frame(&mut peer_stream);
    let mut chunk_batch_finished_payload = Cursor::new(chunk_batch_finished_payload);

    assert_eq!(chunk_batch_start_packet_id, ChunkBatchStartPacket::get_id());
    assert_eq!(
        chunk_batch_finished_packet_id,
        ChunkBatchFinishedPacket::get_id()
    );
    assert_eq!(
        VarIntWrapper::decode(&mut chunk_batch_finished_payload)
            .unwrap()
            .0,
        0
    );
    assert_eq!(player.get_chunk_batch_lead(), 1);
    assert_eq!(player.get_pending_chunk_count(), 9.0);
    assert_eq!(player.get_queued_chunk_count(), 0);
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

    assert_eq!(player.get_queued_chunk_count(), 2);
    assert!(
        player
            .chunk_queue
            .iter()
            .any(|queued_chunk| queued_chunk.chunk == PlayerChunk::new(8, 8))
    );
    assert_eq!(player.get_chunk_batch_lead(), 0);
    assert_eq!(player.get_pending_chunk_count(), 0.0);
    assert_eq!(player.get_target_chunks_per_tick(), 9.0);
    assert!(!player.needs_chunk_position_sync);
}

#[test]
fn ordinary_chunk_border_crossing_unloads_departing_chunk_without_pruning_queue() {
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
    let (forget_chunk_packet_id, _) = read_packet_frame(&mut peer_stream);

    assert_eq!(cache_center_packet_id, SetChunkCacheCenterPacket::get_id());
    assert_eq!(forget_chunk_packet_id, ForgetLevelChunkPacket::get_id());
    assert_eq!(player.get_queued_chunk_count(), 2);
    assert!(
        player
            .chunk_queue
            .iter()
            .any(|queued_chunk| queued_chunk.chunk == PlayerChunk::new(0, 0))
    );
}

#[test]
fn slow_chunk_acknowledgements_do_not_block_multi_border_player_movement() {
    let (mut client, _peer_stream) = test_client_pair();
    client.enable_outbound_packet_queue();
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_client(&mut client);
    player.max_chunk_batch_lead = 1;
    player.needs_chunk_position_sync = false;

    (1..=3).for_each(|chunk_x| {
        player
            .move_to(
                &mut client,
                chunk_x as f64 * 16.0 + 0.1,
                64.0,
                0.0,
                true,
                Some(PlayerChunkTransition {
                    next: PlayerChunk::new(chunk_x, 0),
                    arriving: vec![PlayerChunk::new(chunk_x + 1, 0)],
                    departing: vec![PlayerChunk::new(chunk_x - 1, 0)],
                }),
                vec![empty_chunk_packet(chunk_x + 1, 0)],
            )
            .unwrap();
    });

    assert_eq!(player.get_position().get_x(), 48.1);
    assert_eq!(player.get_chunk_batch_lead(), 1);
    assert!(player.get_queued_chunk_count() > 0);
    assert!(
        !client
            .queued_outbound_packet_ids()
            .contains(&SyncPlayerPositionPacket::get_id())
    );
}

#[test]
fn sharp_turn_keeps_completed_chunks_queued_like_minestom() {
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
        .move_to(
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
            vec![empty_chunk_packet(10, 0), empty_chunk_packet(11, 0)],
        )
        .unwrap();

    player
        .move_to(
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
            vec![empty_chunk_packet(0, 0), empty_chunk_packet(1, 0)],
        )
        .unwrap();

    assert_eq!(player.chunks_loaded_by_client, PlayerChunk::new(0, 0));
    assert_eq!(player.get_queued_chunk_count(), 4);
    assert!(
        player
            .chunk_queue
            .iter()
            .any(|queued_chunk| queued_chunk.chunk == PlayerChunk::new(10, 0))
    );
}

#[test]
fn departing_chunk_is_unloaded_even_when_it_was_previously_sent() {
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
    let (forget_chunk_packet_id, _) = read_packet_frame(&mut peer_stream);

    assert_eq!(cache_center_packet_id, SetChunkCacheCenterPacket::get_id());
    assert_eq!(forget_chunk_packet_id, ForgetLevelChunkPacket::get_id());
    assert!(!player.client_sent_chunks.contains(&PlayerChunk::new(0, 0)));
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
fn server_outbound_drain_flushes_packets_generated_after_player_packet_processing() {
    let mut server = MinecraftServer::new();
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    client.send_packet(99, &[1, 2, 3]).unwrap();
    let client_address = client.addr;
    let client = Arc::new(Mutex::new(client));
    server
        .connection_manager
        .add_connection(client_address, client.clone());

    server.flush_outbound_packets();

    let (packet_id, payload) = read_packet_frame(&mut peer_stream);
    assert_eq!(packet_id, 99);
    assert_eq!(payload, vec![1, 2, 3]);
    assert_eq!(client.lock().unwrap().queued_outbound_packet_count(), 0);
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

    assert_eq!(player.get_effective_chunk_view_distance(10), 9);

    player.set_client_chunk_view_distance(4);

    assert_eq!(player.get_client_chunk_view_distance(), 4);
    assert_eq!(player.get_effective_chunk_view_distance(10), 5);
    assert_eq!(player.get_effective_chunk_view_distance(2), 3);

    player.set_client_chunk_view_distance(-10);

    assert_eq!(player.get_client_chunk_view_distance(), 0);
    assert_eq!(player.get_effective_chunk_view_distance(10), 1);
}

#[test]
fn player_settings_locale_and_view_distance_match_minestom_refresh_surface() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let settings = ClientInformation {
        locale: "fr_FR".to_string(),
        view_distance: 64,
        chat_mode: 1,
        chat_colors: false,
        displayed_skin_parts: 3,
        main_hand: 0,
        enable_text_filtering: true,
        allow_server_listings: false,
        particle_status: 2,
    };

    player.refresh_settings(settings);

    assert_eq!(player.get_locale(), "fr_FR");
    assert_eq!(player.get_client_chunk_view_distance(), 32);
    assert_eq!(player.get_settings().view_distance, 32);
    assert_eq!(player.get_settings().chat_mode, 1);
    assert!(!player.get_settings().chat_colors);
    assert_eq!(player.get_settings().displayed_skin_parts, 3);
    assert_eq!(player.get_settings().main_hand, 0);
    assert!(player.get_settings().enable_text_filtering);
    assert!(!player.get_settings().allow_server_listings);
    assert_eq!(player.get_settings().particle_status, 2);

    player.set_locale("de_DE");

    assert_eq!(player.get_locale(), "de_DE");
    assert_eq!(player.get_settings().view_distance, 32);
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

    assert!(player.chunk_update_limit_checker.is_enabled());
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

    assert_eq!(player.get_game_mode(), GameMode::Survival);
    assert!(player.set_game_mode(GameMode::Creative));
    assert_eq!(player.get_game_mode(), GameMode::Creative);
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
    assert_eq!(player.get_game_mode(), GameMode::Creative);

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
    assert_eq!(player_info_uuid, player.get_uuid());
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

    assert_eq!(player.get_game_mode(), GameMode::Spectator);
    assert!(player.can_fly());
    assert!(player.is_flying());
    assert!(!player.has_instant_break());
    assert!(player.is_invulnerable());
    assert!(!player.has_entity_collision());
    assert!(!player.can_prevent_block_placement());
}

static PLAYER_GAME_MODE_EVENT_TARGET: Mutex<Option<Uuid>> = Mutex::new(None);
static PLAYER_GAME_MODE_EVENT_SEEN: Mutex<Vec<GameMode>> = Mutex::new(Vec::new());

#[event_listener]
fn player_game_mode_change_test_listener(
    event: &mut PlayerGameModeChangeEvent,
    _server: &mut MinecraftServer,
) {
    if *PLAYER_GAME_MODE_EVENT_TARGET.lock().unwrap() != Some(event.player().uuid) {
        return;
    }
    let new_game_mode = event.new_game_mode();
    PLAYER_GAME_MODE_EVENT_SEEN
        .lock()
        .unwrap()
        .push(new_game_mode);
    if new_game_mode == GameMode::Creative {
        event.set_new_game_mode(GameMode::Spectator);
    }
    if new_game_mode == GameMode::Adventure {
        event.set_cancelled(true);
    }
}

#[test]
fn player_game_mode_change_event_can_mutate_and_cancel_state_change() {
    *PLAYER_GAME_MODE_EVENT_TARGET.lock().unwrap() = None;
    PLAYER_GAME_MODE_EVENT_SEEN.lock().unwrap().clear();
    let mut server = MinecraftServer::new();
    let mut player = Player::new(
        Uuid::new_v4(),
        "GameModeEventPlayer".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let (mut client, _peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    client.server_ptr = Some(&mut server as *mut MinecraftServer as usize);
    let player_uuid = player.uuid;
    player.set_client(&mut client);
    player.mark_entered_world();
    *PLAYER_GAME_MODE_EVENT_TARGET.lock().unwrap() = Some(player_uuid);

    assert!(player.set_game_mode(GameMode::Creative));
    assert_eq!(player.get_game_mode(), GameMode::Spectator);
    assert!(!player.set_game_mode(GameMode::Adventure));
    assert_eq!(player.get_game_mode(), GameMode::Spectator);
    assert_eq!(
        PLAYER_GAME_MODE_EVENT_SEEN.lock().unwrap().as_slice(),
        [GameMode::Creative, GameMode::Adventure]
    );

    *PLAYER_GAME_MODE_EVENT_TARGET.lock().unwrap() = None;
    PLAYER_GAME_MODE_EVENT_SEEN.lock().unwrap().clear();
}

static PLAYER_RESPAWN_EVENT_TARGET: Mutex<Option<Uuid>> = Mutex::new(None);
static PLAYER_RESPAWN_EVENT_POSITION: Mutex<Option<PlayerSpawnPoint>> = Mutex::new(None);

#[event_listener]
fn player_respawn_test_listener(event: &mut PlayerRespawnEvent, _server: &mut MinecraftServer) {
    if *PLAYER_RESPAWN_EVENT_TARGET.lock().unwrap() != Some(event.player().uuid) {
        return;
    }
    event.set_respawn_position(PlayerSpawnPoint::new(12.0, 70.0, -4.0, 90.0, 15.0));
    *PLAYER_RESPAWN_EVENT_POSITION.lock().unwrap() = Some(event.respawn_position());
}

#[test]
fn player_identity_and_connection_getters_match_minestom_profile_surface() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565);
    let player = Player::new(Uuid::nil(), "Player".to_string(), 765, addr);

    assert_eq!(player.get_uuid(), Uuid::nil());
    assert_eq!(player.get_identity().get_uuid(), Uuid::nil());
    assert_eq!(player.get_pointers().get_uuid(), Uuid::nil());
    assert_eq!(
        player.get_pointers().get_entity_id(),
        player.get_entity_id()
    );
    assert_eq!(player.get_pointers().get_identity(), player.get_identity());
    assert_eq!(player.get_username(), "Player");
    assert_eq!(player.get_protocol_version(), 765);
    assert_eq!(player.get_address(), addr);
    assert!(player.can_pickup_item());
    assert!(!player.is_dead());
}

#[test]
fn player_state_apis_match_minestom_health_food_experience_and_respawn_surface() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );

    assert_eq!(player.get_health(), 1.0);
    assert_eq!(player.get_food(), 20);
    assert_eq!(player.get_food_saturation(), 5.0);
    assert!(player.is_respawn_screen_enabled());
    assert_eq!(player.get_experience(), 0.0);
    assert_eq!(player.get_experience_level(), 0);
    assert_eq!(player.get_total_experience(), 0);
    assert_eq!(player.get_portal_cooldown(), 0);
    assert_eq!(player.get_additional_hearts(), 0.0);

    player.set_health(12.5).unwrap();
    player.set_additional_hearts(4.0);
    player.set_food(27).unwrap();
    player.set_food_saturation(99.0).unwrap();
    player.set_respawn_screen_enabled(false).unwrap();
    player.set_experience(3.0).unwrap();
    player.set_experience_level(-8).unwrap();
    player.set_total_experience(-2).unwrap();
    player.set_portal_cooldown(-90);

    assert_eq!(player.get_health(), 12.5);
    assert_eq!(player.get_food(), 20);
    assert_eq!(player.get_food_saturation(), 20.0);
    assert!(!player.is_respawn_screen_enabled());
    assert_eq!(player.get_experience(), 1.0);
    assert_eq!(player.get_experience_level(), 0);
    assert_eq!(player.get_total_experience(), 0);
    assert_eq!(player.get_portal_cooldown(), 0);
    assert_eq!(player.get_additional_hearts(), 4.0);
    assert_eq!(player.get_player_metadata().get_entries().len(), 2);
}

#[test]
fn player_death_location_inputs_debug_and_keepalive_state_match_minestom_surface() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let death_position = EntityPosition::new(1.0, 64.0, -2.0, 90.0, 30.0);
    player.set_death_location(death_position);

    let death_location = player.get_death_location().unwrap();
    assert_eq!(
        death_location.get_dimension(),
        &Identifier::minecraft("overworld")
    );
    assert_eq!(death_location.get_position(), death_position);

    player.set_death_location_in_dimension(Identifier::minecraft("the_nether"), death_position);
    assert_eq!(
        player.get_death_location().unwrap().get_dimension(),
        &Identifier::minecraft("the_nether")
    );

    player.refresh_input(true, false, true, false, true, false, true);
    let inputs = player.get_inputs();
    assert!(inputs.forward);
    assert!(inputs.left);
    assert!(inputs.jump);
    assert!(inputs.sprint);
    assert!(!inputs.backward);
    assert!(!inputs.right);
    assert!(!inputs.shift);

    assert_eq!(player.get_eye_height(), 1.62);
    assert!(!player.has_reduced_debug_screen_information());
    player.set_reduced_debug_screen_information(true).unwrap();
    assert!(player.has_reduced_debug_screen_information());

    player.refresh_keep_alive(42);
    player.refresh_answer_keep_alive(true);
    assert_eq!(player.get_last_keep_alive(), 42);
    assert!(player.get_did_answer_keep_alive());
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

    assert_eq!(player.get_pending_spawning_world(), Some(spawning_world));
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

    client.close_connection();

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
    assert_eq!(player.get_queued_packet_count(), 10);
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
    assert_eq!(queued_player.get_queued_packet_count(), 1);
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
    let packet = PlayDisconnectPacket::decode(&mut payload.as_slice()).unwrap();
    let expected_reason = TextComponent::literal_with_color(
        "Too Many Packets",
        spinel_utils::component::color::TextColor::from_named(
            spinel_utils::component::color::NamedTextColor::Red,
        ),
    );
    assert_eq!(packet_id, PlayDisconnectPacket::get_id());
    assert_eq!(packet.reason, expected_reason);
    assert!(!client.is_online());
    peer_stream
        .set_read_timeout(Some(Duration::from_millis(25)))
        .unwrap();
    let mut trailing_packet_byte = [0u8; 1];
    let trailing_packet_read = peer_stream.read(&mut trailing_packet_byte);
    assert!(matches!(trailing_packet_read, Ok(0) | Err(_)));
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
        .get_inventory()
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
        player.get_inventory_ref().get_item_stack(9),
        Some(&ItemStack::of(Material::STONE))
    );
}

#[test]
fn player_respawn_is_noop_when_alive_and_refreshes_client_state_when_dead() {
    *PLAYER_RESPAWN_EVENT_TARGET.lock().unwrap() = None;
    *PLAYER_RESPAWN_EVENT_POSITION.lock().unwrap() = None;
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

    assert_eq!(respawn_packet_id, RespawnPacket::get_id());
    assert_eq!(game_event_packet_id, GameEventPacket::get_id());
    assert_eq!(
        difficulty_packet_id,
        spinel_core::network::clientbound::play::server_difficulty::ServerDifficultyPacket::get_id(
        )
    );
    assert_eq!(health_packet_id, SetHealthPacket::get_id());
    assert_eq!(experience_packet_id, SetExperiencePacket::get_id());
    assert_eq!(abilities_packet_id, PlayerAbilitiesPacket::get_id());
    assert!(!player.is_dead());
}

#[test]
fn player_respawn_event_can_mutate_respawn_position() {
    *PLAYER_RESPAWN_EVENT_TARGET.lock().unwrap() = None;
    *PLAYER_RESPAWN_EVENT_POSITION.lock().unwrap() = None;
    let mut server = MinecraftServer::new();
    let mut player = Player::new(
        Uuid::new_v4(),
        "RespawnEventPlayer".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    client.server_ptr = Some(&mut server as *mut MinecraftServer as usize);
    let player_uuid = player.uuid;
    player.set_client(&mut client);
    player.mark_entered_world();
    player.kill().unwrap();
    let _ = read_packet_frame(&mut peer_stream);
    *PLAYER_RESPAWN_EVENT_TARGET.lock().unwrap() = Some(player_uuid);

    assert!(player.respawn().unwrap());

    let event_position = PLAYER_RESPAWN_EVENT_POSITION.lock().unwrap().unwrap();
    assert_eq!(event_position.x, 12.0);
    assert_eq!(event_position.y, 70.0);
    assert_eq!(event_position.z, -4.0);
    assert_eq!(player.get_position().get_x(), 12.0);
    assert_eq!(player.get_position().get_y(), 70.0);
    assert_eq!(player.get_position().get_z(), -4.0);
    *PLAYER_RESPAWN_EVENT_TARGET.lock().unwrap() = None;
    *PLAYER_RESPAWN_EVENT_POSITION.lock().unwrap() = None;
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
fn queued_chunks_are_sent_nearest_to_the_latest_client_center_first() {
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
    player.chunks_loaded_by_client = PlayerChunk::new(5, 0);
    player.send_chunk(empty_chunk_packet(0, 0));
    player.send_chunk(empty_chunk_packet(4, 0));

    player.send_pending_chunks().unwrap();

    let (batch_start_packet_id, _) = read_packet_frame(&mut peer_stream);
    let (first_chunk_packet_id, first_chunk_payload) = read_packet_frame(&mut peer_stream);
    let first_chunk_x = i32::from_be_bytes(first_chunk_payload[0..4].try_into().unwrap());
    let first_chunk_z = i32::from_be_bytes(first_chunk_payload[4..8].try_into().unwrap());

    assert_eq!(batch_start_packet_id, ChunkBatchStartPacket::get_id());
    assert_eq!(
        first_chunk_packet_id,
        ChunkDataAndUpdateLightPacket::get_id()
    );
    assert_eq!((first_chunk_x, first_chunk_z), (4, 0));
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
    player.reset_chunk_queue();
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

    assert_eq!(respawn_packet_id, RespawnPacket::get_id());
    assert_eq!(game_event_packet_id, GameEventPacket::get_id());
    assert_eq!(
        difficulty_packet_id,
        spinel_core::network::clientbound::play::server_difficulty::ServerDifficultyPacket::get_id(
        )
    );
    assert_eq!(health_packet_id, SetHealthPacket::get_id());
    assert_eq!(experience_packet_id, SetExperiencePacket::get_id());
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
    let player_info_packet = player.get_player_info_packet();

    assert_eq!(player.get_skin().unwrap().get_textures(), "texture-data");
    assert_eq!(
        player.get_skin().unwrap().get_signature(),
        Some("signature-data")
    );
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
fn active_player_skin_refresh_matches_minestom_self_viewer_and_player_list_recipients() {
    let mut server = MinecraftServer::new();
    let (mut source_client, _source_peer) = test_client_pair();
    let (mut viewer_client, _viewer_peer) = test_client_pair();
    let (mut non_viewer_client, _non_viewer_peer) = test_client_pair();
    source_client.state = ConnectionState::Play;
    source_client.enable_outbound_packet_queue();
    viewer_client.state = ConnectionState::Play;
    viewer_client.enable_outbound_packet_queue();
    non_viewer_client.state = ConnectionState::Play;
    non_viewer_client.enable_outbound_packet_queue();
    let source_addr = source_client.addr;
    let viewer_addr = viewer_client.addr;
    let non_viewer_addr = non_viewer_client.addr;
    let source_client = Arc::new(Mutex::new(source_client));
    let viewer_client = Arc::new(Mutex::new(viewer_client));
    let non_viewer_client = Arc::new(Mutex::new(non_viewer_client));
    server
        .connection_manager
        .add_connection(source_addr, source_client.clone());
    server
        .connection_manager
        .add_connection(viewer_addr, viewer_client.clone());
    server
        .connection_manager
        .add_connection(non_viewer_addr, non_viewer_client.clone());
    let mut source = Player::new(Uuid::new_v4(), "Source".to_owned(), 0, source_addr);
    let mut viewer = Player::new(Uuid::new_v4(), "Viewer".to_owned(), 0, viewer_addr);
    let mut non_viewer = Player::new(Uuid::new_v4(), "NonViewer".to_owned(), 0, non_viewer_addr);
    {
        let mut source_client = source_client.lock().unwrap();
        source_client.server_ptr = Some(&mut server as *mut MinecraftServer as usize);
        source.set_client(&mut source_client);
    }
    {
        let mut viewer_client = viewer_client.lock().unwrap();
        viewer.set_client(&mut viewer_client);
    }
    {
        let mut non_viewer_client = non_viewer_client.lock().unwrap();
        non_viewer.set_client(&mut non_viewer_client);
    }
    source.get_view_mut().manual_add(viewer.get_entity_id());
    source.mark_entered_world();

    source
        .set_skin(Some(PlayerSkin::new(
            "texture-data",
            Some("signature-data".to_owned()),
        )))
        .unwrap();

    let source_packet_ids = source_client.lock().unwrap().queued_outbound_packet_ids();
    assert!(source_packet_ids.contains(&RespawnPacket::get_id()));
    assert!(source_packet_ids.contains(&SetHealthPacket::get_id()));
    assert!(source_packet_ids.contains(&SetExperiencePacket::get_id()));
    assert!(source_packet_ids.contains(&EntityStatusPacket::get_id()));
    assert!(source_packet_ids.contains(&PlayerAbilitiesPacket::get_id()));
    assert!(source_packet_ids.contains(&SyncPlayerPositionPacket::get_id()));
    assert!(
        source_packet_ids
            .iter()
            .filter(|packet_id| **packet_id == PlayerInfoRemovePacket::get_id())
            .count()
            >= 2
    );
    assert!(
        source_packet_ids
            .iter()
            .filter(|packet_id| **packet_id == PlayerInfoUpdatePacket::get_id())
            .count()
            >= 2
    );

    let viewer_packet_ids = viewer_client.lock().unwrap().queued_outbound_packet_ids();
    assert_eq!(
        viewer_packet_ids,
        vec![
            PlayerInfoRemovePacket::get_id(),
            RemoveEntitiesPacket::get_id(),
            PlayerInfoUpdatePacket::get_id(),
            SpawnEntityPacket::get_id(),
            SetEntityDataPacket::get_id(),
            spinel_core::network::clientbound::play::entity_head_look::EntityHeadLookPacket::get_id(
            ),
            SetEquipmentPacket::get_id(),
            UpdateAttributesPacket::get_id(),
        ]
    );
    assert_eq!(
        non_viewer_client
            .lock()
            .unwrap()
            .queued_outbound_packet_ids(),
        vec![
            PlayerInfoRemovePacket::get_id(),
            PlayerInfoUpdatePacket::get_id(),
        ]
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

    assert!(player.get_display_name().is_some());
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
    assert_eq!(packet.entries.0[0].uuid, player.get_uuid());
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

    assert_eq!(player.get_last_sent_teleport_id(), 0);
    assert_eq!(player.get_last_received_teleport_id(), 0);
    assert_eq!(player.get_next_teleport_id(), 1);
    assert!(player.has_pending_teleport_confirmation());
    player.set_last_received_teleport_id(-1);
    assert_eq!(player.get_last_received_teleport_id(), 0);
    player.set_last_received_teleport_id(1);
    assert_eq!(player.get_last_received_teleport_id(), 1);
    assert!(!player.has_pending_teleport_confirmation());
}

#[test]
fn player_teleport_overloads_preserve_or_resolve_velocity_like_minestom() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_position(EntityPosition::new(10.0, 20.0, 30.0, 40.0, 50.0));
    player.set_velocity(Velocity(Vector3d {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    }));

    let preserved_velocity = player
        .teleport(
            EntityPosition::new(4.0, 5.0, 6.0, 7.0, 8.0),
            None,
            TeleportFlags::absolute(),
            false,
        )
        .unwrap();

    assert_eq!(
        preserved_velocity.get_velocity(),
        Velocity(Vector3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        })
    );
    assert_eq!(
        preserved_velocity.get_flags().bitmask(),
        TeleportFlags::DELTA_COORD
    );

    let teleport = player
        .teleport_with_velocity_chunks_and_flags(
            EntityPosition::new(2.0, 70.0, -3.0, 15.0, 25.0),
            Velocity(Vector3d {
                x: 0.5,
                y: 8.0,
                z: -0.5,
            }),
            None,
            TeleportFlags::from_bitmask(
                TeleportFlags::X
                    | TeleportFlags::Z
                    | TeleportFlags::Y_ROTATION
                    | TeleportFlags::DELTA_X
                    | TeleportFlags::DELTA_Z
                    | TeleportFlags::ROTATE_DELTA,
            ),
            false,
        )
        .unwrap();

    assert_eq!(
        teleport.get_position(),
        EntityPosition::new(6.0, 70.0, 3.0, 22.0, 25.0)
    );
    assert_eq!(
        teleport.get_velocity(),
        Velocity(Vector3d {
            x: 1.5,
            y: 8.0,
            z: 2.5,
        })
    );
}

#[test]
fn player_velocity_packet_converts_blocks_per_second_to_blocks_per_tick() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    player.set_velocity(Velocity(Vector3d {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    }));

    assert_eq!(
        player.get_velocity_packet().velocity,
        Velocity(Vector3d {
            x: 0.05,
            y: 0.1,
            z: 0.15,
        })
    );
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
    assert_eq!(player.get_last_sent_teleport_id(), 1);
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
    assert_eq!(player.get_last_sent_teleport_id(), 0);
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
    assert_eq!(player.get_latency(), 42);
    assert!(player.can_fly());
    assert!(player.is_flying());
    assert!(player.has_instant_break());
    assert!(player.is_invulnerable());
    assert_eq!(player.get_flying_speed(), 0.2);
    assert_eq!(player.get_field_view_modifier(), 0.3);
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
        player.get_item_in_hand(PlayerHand::Main).material(),
        &Material::EMERALD
    );
    assert_eq!(
        player.get_item_in_hand(PlayerHand::Off).material(),
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

    let equipment_packet = player.get_visible_equipment_packet();

    assert_eq!(
        equipment_packet.entity_id,
        player.get_entity_id().get_value()
    );
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

    assert_eq!(
        player.get_client().map(|client| client.addr),
        Some(client.addr)
    );
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
        player.get_metadata_packet().entries.0[0].value,
        MetadataValue::Byte(0)
    );
    assert!(player.set_sprinting(true));
    assert_eq!(
        player.get_metadata_packet().entries.0[0].value,
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
            .get_metadata_packet()
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
            .get_metadata_packet()
            .entries
            .0
            .iter()
            .any(|entry| entry.value == MetadataValue::Pose(5))
    );
}

#[test]
fn player_base_metadata_api_matches_minestom_entity_meta_defaults() {
    let mut player = Player::new(
        Uuid::new_v4(),
        "Steve".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );

    assert_eq!(player.get_custom_name(), None);
    assert!(!player.is_custom_name_visible());
    assert!(!player.is_silent());
    assert!(!player.is_swimming());
    assert!(!player.is_invisible());
    assert!(!player.is_glowing());
    assert_eq!(player.get_air_ticks(), 300);
    assert_eq!(player.ticks_frozen(), 0);

    let metadata_custom_name = TextComponent::literal("metadata-name");
    let player_info_display_name = TextComponent::literal("display-name");
    player.set_custom_name(Some(metadata_custom_name.clone()));
    player.set_custom_name_visible(true);
    player.set_silent(true);
    player.set_swimming(true);
    player.set_invisible(true);
    player.set_glowing(true);
    player.set_air_ticks(12);
    player.set_ticks_frozen(9);
    player
        .set_display_name(Some(player_info_display_name.clone()))
        .unwrap();

    assert_eq!(player.get_custom_name(), Some(metadata_custom_name));
    assert_eq!(player.get_display_name(), Some(&player_info_display_name));
    assert!(player.is_custom_name_visible());
    assert!(player.is_silent());
    assert!(player.is_swimming());
    assert!(player.is_invisible());
    assert!(player.is_glowing());
    assert_eq!(player.get_air_ticks(), 12);
    assert_eq!(player.ticks_frozen(), 9);
    assert_eq!(player.get_pose(), EntityPose::Swimming);
    assert!(player.get_dirty_metadata_packet().is_some());
}

#[test]
fn player_living_metadata_api_matches_minestom_living_entity_meta_surface() {
    let mut player = Player::new(
        Uuid::new_v4(),
        "Steve".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let effect_particle = Particle::effect();
    let bed_position = Position { x: 1, y: 64, z: 2 };

    assert!(!player.is_hand_active());
    assert_eq!(player.get_active_hand(), PlayerHand::Main);
    assert!(!player.is_in_riptide_spin_attack());
    assert!(player.get_effect_particles().is_empty());
    assert!(!player.is_potion_effect_ambient());
    assert_eq!(player.get_arrow_count(), 0);
    assert_eq!(player.get_bee_stinger_count(), 0);
    assert_eq!(player.get_bed_in_which_sleeping_position(), None);

    player.set_hand_active(true);
    player.set_active_hand(PlayerHand::Off);
    player.set_in_riptide_spin_attack(true);
    player.set_effect_particles(vec![effect_particle.clone()]);
    player.set_potion_effect_ambient(true);
    player.set_arrow_count(3);
    player.set_bee_stinger_count(4);
    player.set_bed_in_which_sleeping_position(Some(bed_position));
    player.set_health(13.5).unwrap();

    assert!(player.is_hand_active());
    assert_eq!(player.get_active_hand(), PlayerHand::Off);
    assert!(player.is_in_riptide_spin_attack());
    assert_eq!(player.get_effect_particles(), vec![effect_particle]);
    assert!(player.is_potion_effect_ambient());
    assert_eq!(player.get_arrow_count(), 3);
    assert_eq!(player.get_bee_stinger_count(), 4);
    assert_eq!(
        player.get_bed_in_which_sleeping_position(),
        Some(bed_position)
    );
    assert_eq!(player.get_health(), 13.5);
    assert!(player.get_dirty_metadata_packet().is_some());
}

#[test]
fn player_avatar_and_player_metadata_api_matches_minestom_meta_surface() {
    let mut player = Player::new(
        Uuid::new_v4(),
        "Steve".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );

    {
        let player_meta = player.get_entity_meta_mut();
        assert_eq!(player_meta.get_main_hand(), MainHand::Right);
        assert_eq!(player_meta.get_displayed_skin_parts(), 0);
        assert_eq!(player_meta.get_score(), 0);
        assert_eq!(player_meta.get_left_shoulder_entity_data(), None);
        assert_eq!(player_meta.get_right_shoulder_entity_data(), None);
    }

    {
        let mut player_meta = player.get_entity_meta_mut();
        player_meta.set_main_hand(MainHand::Left);
        player_meta.set_cape_enabled(true);
        player_meta.set_jacket_enabled(true);
        player_meta.set_left_sleeve_enabled(true);
        player_meta.set_right_sleeve_enabled(true);
        player_meta.set_left_leg_enabled(true);
        player_meta.set_right_leg_enabled(true);
        player_meta.set_hat_enabled(true);
        player_meta.set_displayed_skin_parts(0x7f);
        player_meta.set_additional_hearts(2.5);
        player_meta.set_score(12);
        player_meta.set_left_shoulder_entity_data(Some(1));
        player_meta.set_right_shoulder_entity_data(Some(2));
    }

    {
        let player_meta = player.get_entity_meta_mut();
        assert_eq!(player_meta.get_main_hand(), MainHand::Left);
        assert!(player_meta.is_cape_enabled());
        assert!(player_meta.is_jacket_enabled());
        assert!(player_meta.is_left_sleeve_enabled());
        assert!(player_meta.is_right_sleeve_enabled());
        assert!(player_meta.is_left_leg_enabled());
        assert!(player_meta.is_right_leg_enabled());
        assert!(player_meta.is_hat_enabled());
        assert_eq!(player_meta.get_displayed_skin_parts(), 0x7f);
        assert_eq!(player_meta.get_additional_hearts(), 2.5);
        assert_eq!(player_meta.get_score(), 12);
        assert_eq!(player_meta.get_left_shoulder_entity_data(), Some(1));
        assert_eq!(player_meta.get_right_shoulder_entity_data(), Some(2));
    }
    assert!(player.get_dirty_metadata_packet().is_some());
}

#[test]
fn public_sneaking_and_sprinting_setters_broadcast_dirty_metadata_to_play_clients() {
    let mut server = MinecraftServer::new();
    let (source_client, _source_peer) = test_client_pair();
    let (mut viewer_client, _viewer_peer) = test_client_pair();
    let (mut non_viewer_client, _non_viewer_peer) = test_client_pair();
    let source_addr = source_client.addr;
    viewer_client.state = ConnectionState::Play;
    viewer_client.enable_outbound_packet_queue();
    non_viewer_client.state = ConnectionState::Play;
    non_viewer_client.enable_outbound_packet_queue();
    let source_client = Arc::new(Mutex::new(source_client));
    let viewer_client = Arc::new(Mutex::new(viewer_client));
    let non_viewer_client = Arc::new(Mutex::new(non_viewer_client));
    server
        .connection_manager
        .add_connection(source_addr, source_client.clone());
    let viewer_addr = viewer_client.lock().unwrap().addr;
    server
        .connection_manager
        .add_connection(viewer_addr, viewer_client.clone());
    let non_viewer_addr = non_viewer_client.lock().unwrap().addr;
    server
        .connection_manager
        .add_connection(non_viewer_addr, non_viewer_client.clone());
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, source_addr);
    let mut viewer = Player::new(Uuid::new_v4(), "Viewer".to_string(), 0, viewer_addr);
    let mut non_viewer = Player::new(Uuid::new_v4(), "NonViewer".to_string(), 0, non_viewer_addr);
    {
        let mut source_client = source_client.lock().unwrap();
        source_client.state = ConnectionState::Play;
        source_client.server_ptr = Some(&mut server as *mut MinecraftServer as usize);
        player.set_client(&mut source_client);
    }
    {
        let mut viewer_client = viewer_client.lock().unwrap();
        viewer.set_client(&mut viewer_client);
    }
    {
        let mut non_viewer_client = non_viewer_client.lock().unwrap();
        non_viewer.set_client(&mut non_viewer_client);
    }
    player.get_view_mut().manual_add(viewer.get_entity_id());
    player.mark_entered_world();

    player.set_sneaking(true);
    player.set_sprinting(true);

    assert_eq!(
        viewer_client.lock().unwrap().queued_outbound_packet_ids(),
        vec![SetEntityDataPacket::get_id(), SetEntityDataPacket::get_id()]
    );
    assert!(
        non_viewer_client
            .lock()
            .unwrap()
            .queued_outbound_packet_ids()
            .is_empty()
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
    assert_eq!(player.get_current_item_use_time(), 0);

    player.set_item_in_hand(PlayerHand::Main, ItemStack::of(Material::POTION));
    player.refresh_item_use(Some(PlayerHand::Main), 2);

    assert!(player.is_using_item());
    assert!(player.is_eating());
    assert_eq!(player.get_item_use_hand(), Some(PlayerHand::Main));
    assert_eq!(player.get_current_item_use_time(), 0);
    assert!(player.tick().is_none());
    assert_eq!(player.get_current_item_use_time(), 1);
    let item_use_completion = player.tick().unwrap();
    assert_eq!(
        item_use_completion.entity_id,
        player.get_entity_id().get_value()
    );
    assert_eq!(item_use_completion.status, 9);
    assert!(!player.is_using_item());
    assert_eq!(player.get_current_item_use_time(), 0);
}

#[test]
fn direct_packet_send_apis_match_minestom_player_surface() {
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    player.set_client(&mut client);

    player
        .send_packet(SystemChatPacket::new(TextComponent::literal("one"), false))
        .unwrap();
    player
        .send_packets(vec![
            SystemChatPacket::new(TextComponent::literal("two"), false),
            SystemChatPacket::new(TextComponent::literal("three"), true),
        ])
        .unwrap();
    player
        .send_plugin_message("minecraft:test", vec![1, 2, 3])
        .unwrap();
    player
        .send_plugin_message_string("minecraft:string_test", "payload")
        .unwrap();
    player.send_raw_packet(0x7f, &[1, 2, 3]).unwrap();

    let packet_ids: Vec<i32> = read_available_packet_frames(&mut peer_stream)
        .into_iter()
        .map(|(packet_id, _)| packet_id)
        .collect();

    assert_eq!(
        packet_ids,
        vec![
            SystemChatPacket::get_id(),
            SystemChatPacket::get_id(),
            SystemChatPacket::get_id(),
            PlayCustomPayloadPacket::get_id(),
            PlayCustomPayloadPacket::get_id(),
            0x7f,
        ]
    );
}

#[test]
fn direct_packet_send_apis_noop_when_player_has_no_connection() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );

    player
        .send_packet(SystemChatPacket::new(
            TextComponent::literal("offline"),
            false,
        ))
        .unwrap();
    player
        .send_packets(vec![SystemChatPacket::new(
            TextComponent::literal("offline batch"),
            false,
        )])
        .unwrap();
    player.send_raw_packet(0x7f, &[1, 2, 3]).unwrap();
}

#[test]
fn player_message_type_respects_minestom_chat_visibility_modes() {
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    let mut settings = ClientInformation::default();
    settings.chat_mode = 1;
    player.set_client(&mut client);
    player.refresh_settings(settings);

    player
        .send_message_from(
            Uuid::nil(),
            TextComponent::literal("chat"),
            PlayerMessageType::Chat,
        )
        .unwrap();
    player
        .send_message_from(
            Uuid::nil(),
            TextComponent::literal("system"),
            PlayerMessageType::System,
        )
        .unwrap();
    player
        .send_message_from(
            Uuid::nil(),
            TextComponent::literal("action"),
            PlayerMessageType::ActionBar,
        )
        .unwrap();

    let packet_ids = read_available_packet_frames(&mut peer_stream)
        .into_iter()
        .map(|(packet_id, _)| packet_id)
        .collect::<Vec<_>>();

    assert_eq!(
        packet_ids,
        vec![SystemChatPacket::get_id(), SystemChatPacket::get_id()]
    );
}

#[test]
fn player_resource_pack_api_matches_minestom_request_status_and_required_kick_flow() {
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    player.set_client(&mut client);
    let resource_pack_id = Uuid::from_u128(1);
    let resource_pack = ResourcePackInfo::new(
        resource_pack_id,
        "https://example.com/pack.zip",
        "0123456789abcdef0123456789abcdef01234567",
    );
    let request = ResourcePackRequest::new(vec![resource_pack.clone()])
        .required(true)
        .replace(true)
        .prompt(Some(TextComponent::literal("Required")));

    player.send_resource_packs(request).unwrap();

    assert_eq!(player.get_pending_resource_pack_count(), 1);
    assert_eq!(
        player
            .get_resource_pack_future()
            .unwrap()
            .get_pending_count(),
        1
    );
    let packet_frames = read_available_packet_frames(&mut peer_stream);
    assert_eq!(packet_frames[0].0, ResourcePackPopPacket::get_id());
    assert_eq!(packet_frames[1].0, ResourcePackPushPacket::get_id());
    let pushed_packet = ResourcePackPushPacket::decode(&mut packet_frames[1].1.as_slice()).unwrap();
    assert_eq!(pushed_packet.id, resource_pack_id);
    assert_eq!(pushed_packet.url, resource_pack.url());
    assert_eq!(pushed_packet.hash, resource_pack.hash());
    assert!(pushed_packet.forced);
    assert!(pushed_packet.prompt.is_some());

    player
        .on_resource_pack_status(resource_pack_id, ResourcePackStatus::Accepted)
        .unwrap();
    assert_eq!(player.get_pending_resource_pack_count(), 0);
    let (disconnect_packet_id, disconnect_payload) = read_packet_frame(&mut peer_stream);
    let disconnect_packet =
        PlayDisconnectPacket::decode(&mut disconnect_payload.as_slice()).unwrap();

    assert_eq!(disconnect_packet_id, PlayDisconnectPacket::get_id());
    assert_eq!(
        disconnect_packet.reason.content,
        ComponentType::Text("Required resource pack was not loaded.".to_string())
    );
}

#[test]
fn player_resource_pack_remove_clear_and_success_status_match_minestom_surface() {
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    player.set_client(&mut client);
    let first_pack_id = Uuid::from_u128(1);
    let second_pack_id = Uuid::from_u128(2);
    let request = ResourcePackRequest::new(vec![ResourcePackInfo::new(
        first_pack_id,
        "https://example.com/pack.zip",
        "hash",
    )])
    .required(true);

    player.send_resource_packs(request).unwrap();
    player
        .on_resource_pack_status(first_pack_id, ResourcePackStatus::SuccessfullyLoaded)
        .unwrap();
    assert!(player.get_resource_pack_future().is_none());

    player
        .remove_resource_packs(first_pack_id, [second_pack_id])
        .unwrap();
    player.clear_resource_packs().unwrap();

    let packet_ids: Vec<i32> = read_available_packet_frames(&mut peer_stream)
        .into_iter()
        .map(|(packet_id, _)| packet_id)
        .collect();

    assert_eq!(
        packet_ids,
        vec![
            ResourcePackPushPacket::get_id(),
            ResourcePackPopPacket::get_id(),
            ResourcePackPopPacket::get_id(),
            ResourcePackPopPacket::get_id(),
        ]
    );
}

#[test]
fn player_resource_pack_api_uses_configuration_packet_ids_during_configuration() {
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Configuration;
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    player.set_client(&mut client);
    let resource_pack_id = Uuid::from_u128(1);
    let request = ResourcePackRequest::new(vec![ResourcePackInfo::new(
        resource_pack_id,
        "https://example.com/pack.zip",
        "hash",
    )])
    .required(false)
    .replace(true);

    player.send_resource_packs(request).unwrap();

    let packet_ids: Vec<i32> = read_available_packet_frames(&mut peer_stream)
        .into_iter()
        .map(|(packet_id, _)| packet_id)
        .collect();

    assert_eq!(
        packet_ids,
        vec![
            ConfigurationResourcePackPopPacket::get_id(),
            ConfigurationResourcePackPushPacket::get_id(),
        ]
    );
}

#[test]
fn player_tab_list_and_title_apis_send_minestom_packets() {
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    player.set_client(&mut client);

    player
        .send_player_list_header_and_footer(
            TextComponent::literal("header"),
            TextComponent::literal("footer"),
        )
        .unwrap();
    player.send_title(TextComponent::literal("title")).unwrap();
    player
        .send_subtitle(TextComponent::literal("subtitle"))
        .unwrap();
    player.set_title_animation(1, 2, 3).unwrap();
    player.clear_title().unwrap();
    player.show_dialog(&Dialog::QUICK_ACTIONS).unwrap();
    player.close_dialog().unwrap();
    player
        .send_notification(Notification::from_material(
            TextComponent::literal("toast"),
            AdvancementFrameType::Task,
            Material::DIAMOND,
        ))
        .unwrap();
    player
        .set_below_name_tag(Some(BelowNameTag::new(
            "health",
            TextComponent::literal("Health"),
        )))
        .unwrap();
    player.set_below_name_tag(None).unwrap();
    player.reset_title().unwrap();

    let packet_ids: Vec<i32> = read_available_packet_frames(&mut peer_stream)
        .into_iter()
        .map(|(packet_id, _)| packet_id)
        .collect();

    assert_eq!(
        packet_ids,
        vec![
            TabListPacket::get_id(),
            SetTitleTextPacket::get_id(),
            SetSubtitleTextPacket::get_id(),
            SetTitlesAnimationPacket::get_id(),
            ClearTitlesPacket::get_id(),
            ShowDialogPacket::get_id(),
            ClearDialogPacket::get_id(),
            AdvancementsPacket::get_id(),
            AdvancementsPacket::get_id(),
            ScoreboardObjectivePacket::get_id(),
            DisplayScoreboardPacket::get_id(),
            ScoreboardObjectivePacket::get_id(),
            ClearTitlesPacket::get_id(),
        ]
    );
}

#[test]
fn player_permission_and_spectate_apis_send_minestom_packets() {
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    player.set_client(&mut client);
    let target_entity_id = EntityId::next();

    player.set_permission_level(3).unwrap();
    player.spectate(target_entity_id).unwrap();
    player.stop_spectating().unwrap();

    let packet_ids: Vec<i32> = read_available_packet_frames(&mut peer_stream)
        .into_iter()
        .map(|(packet_id, _)| packet_id)
        .collect();

    assert_eq!(player.get_permission_level(), 3);
    assert_eq!(
        packet_ids,
        vec![
            EntityStatusPacket::get_id(),
            SetCameraPacket::get_id(),
            SetCameraPacket::get_id(),
        ]
    );
}

#[test]
fn player_dimension_statistics_hover_and_leave_bed_api_match_minestom_surface() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );

    assert_eq!(
        player.get_dimension_type(),
        &spinel_registry::dimension_type::DimensionType::OVERWORLD
    );
    assert!(player.get_statistic_value_map().is_empty());
    assert_eq!(player.get_statistic_value("minecraft:jump"), 0);
    player.set_statistic_value("minecraft:jump", 4);
    assert_eq!(player.increment_statistic_value("minecraft:jump", 2), 6);
    assert_eq!(player.get_statistic_value("minecraft:jump"), 6);
    player.leave_bed();

    let HoverEvent::ShowEntity(hover_entity) = player.as_hover_event() else {
        panic!("expected show entity hover event");
    };

    assert_eq!(hover_entity.entity_type, "minecraft:player");
    assert_eq!(hover_entity.id, player.get_uuid().to_string());
    assert!(hover_entity.name.is_some());

    let snapshot = player.update_snapshot(|snapshot| {
        assert_eq!(snapshot.get_username(), "Player");
        assert_eq!(snapshot.get_game_mode(), GameMode::Survival);
    });

    assert_eq!(snapshot.get_uuid(), player.get_uuid());
    assert_eq!(
        snapshot.get_statistics(),
        &[("minecraft:jump".to_string(), 6)]
    );
}

#[test]
fn player_look_at_and_face_position_apis_send_minestom_face_player_packets() {
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    player.set_client(&mut client);
    let target_entity = EntityId::next();

    player
        .look_at_position(Vector3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        })
        .unwrap();
    player
        .face_position(
            FacePoint::Feet,
            Vector3d {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            },
        )
        .unwrap();
    player
        .look_at_entity(target_entity, EntityPosition::new(7.0, 8.0, 9.0, 0.0, 0.0))
        .unwrap();

    let packet_frames = read_available_packet_frames(&mut peer_stream);
    let packet_ids = packet_frames
        .iter()
        .map(|(packet_id, _)| *packet_id)
        .collect::<Vec<_>>();
    let first_packet = PlayerLookAtPacket::decode(&mut packet_frames[0].1.as_slice()).unwrap();
    let second_packet = PlayerLookAtPacket::decode(&mut packet_frames[1].1.as_slice()).unwrap();
    let third_packet = PlayerLookAtPacket::decode(&mut packet_frames[2].1.as_slice()).unwrap();

    assert_eq!(
        packet_ids,
        vec![
            PlayerLookAtPacket::get_id(),
            PlayerLookAtPacket::get_id(),
            PlayerLookAtPacket::get_id()
        ]
    );
    assert_eq!(first_packet.look_at.face_point, FacePoint::Eyes);
    assert!(first_packet.look_at.entity.is_none());
    assert_eq!(second_packet.look_at.face_point, FacePoint::Feet);
    assert_eq!(
        third_packet.look_at.entity.unwrap().entity_id,
        target_entity.get_value()
    );
}

#[test]
fn player_debug_subscriptions_and_vehicle_state_match_minestom_listener_surface() {
    let (client, _peer_stream) = test_client_pair();
    let mut worlds = WorldManager::new();
    let world = World::new(Identifier::minecraft("overworld"));
    let world_uuid = world.uuid();
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    let mut vehicle = GenericEntity::new(EntityType::MINECART);
    let vehicle_id = vehicle.get_entity_id();
    let vehicle_position = EntityPosition::new(1.0, 2.0, 3.0, 4.0, 5.0);

    player.set_debug_subscriptions(BTreeSet::from([0, 2]));
    player.set_vehicle(vehicle_id);
    vehicle.set_position(EntityPosition::new(0.0, 0.0, 0.0, 0.0, 0.0));
    worlds.register_world(world);
    worlds.add_entity(world_uuid, Entity::Player(player));
    worlds.add_entity(world_uuid, Entity::Generic(vehicle));

    assert!(
        worlds
            .player_pointer_for_client(&client)
            .map(|player| unsafe { &*player }.get_debug_subscriptions().contains(&2))
            .unwrap()
    );
    assert!(worlds.move_client_world_entity(&client, vehicle_id, vehicle_position));
    assert_eq!(
        worlds
            .world(world_uuid)
            .and_then(|world| world.get_entity(vehicle_id))
            .map(Entity::get_position),
        Some(vehicle_position)
    );
}

#[test]
fn player_sound_effect_action_bar_and_boss_bar_apis_send_minestom_packets() {
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    player.set_client(&mut client);
    let boss_bar = BossBar::new(
        TextComponent::literal("boss"),
        1.0,
        WorldBossBarColor::Purple,
        WorldBossBarOverlay::Progress,
        0,
    );

    player
        .send_action_bar(TextComponent::literal("action"))
        .unwrap();
    player.play_sound(SoundEvent::Id(2)).unwrap();
    player
        .play_sound_at_position(
            SoundEvent::Id(3),
            Vector3d {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
        )
        .unwrap();
    player
        .play_sound_from_entity(SoundEvent::Id(4), EntityId::next())
        .unwrap();
    player
        .stop_sound(Some(3), Some(Identifier::minecraft("entity.arrow.hit")))
        .unwrap();
    player
        .play_effect(2001, Position { x: 1, y: 2, z: 3 }, 4, false)
        .unwrap();
    player.show_boss_bar(&boss_bar).unwrap();
    player.hide_boss_bar(&boss_bar).unwrap();

    let packet_ids: Vec<i32> = read_available_packet_frames(&mut peer_stream)
        .into_iter()
        .map(|(packet_id, _)| packet_id)
        .collect();

    assert_eq!(
        packet_ids,
        vec![
            SystemChatPacket::get_id(),
            SoundEffectPacket::get_id(),
            SoundEffectPacket::get_id(),
            EntitySoundEffectPacket::get_id(),
            StopSoundPacket::get_id(),
            WorldEventPacket::get_id(),
            BossBarPacket::get_id(),
            BossBarPacket::get_id(),
        ]
    );
}

#[test]
fn consuming_block_placement_syncs_only_the_held_slot() {
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    assert!(player.set_item_in_hand(
        PlayerHand::Main,
        ItemStack::of(Material::STONE).with_amount(2),
    ));
    player.set_client(&mut client);

    assert!(
        crate::network::play::use_item_on::synchronize_placed_block_inventory(
            &mut player,
            PlayerHand::Main,
            true,
            &mut client,
        )
    );

    assert_eq!(player.get_item_in_hand(PlayerHand::Main).amount(), 1);
    let packet_ids = read_available_packet_frames(&mut peer_stream)
        .into_iter()
        .map(|(packet_id, _)| packet_id)
        .collect::<Vec<_>>();
    assert_eq!(
        packet_ids,
        vec![
            spinel_core::network::clientbound::play::set_player_inventory::SetPlayerInventoryPacket::get_id(),
            UpdateAttributesPacket::get_id(),
        ]
    );
}

#[test]
fn player_inventory_add_item_stacks_syncs_only_changed_slots() {
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let mut player = Player::new(Uuid::nil(), "Player".to_string(), 0, client.addr);
    let occupied_item = ItemStack::of(Material::DIAMOND);
    assert!(
        player
            .get_inventory()
            .set_item_stack(0, occupied_item.clone())
    );
    player.set_client(&mut client);
    player.mark_entered_world();

    assert_eq!(
        player.get_inventory().add_item_stacks(vec![
            ItemStack::of(Material::STICK),
            ItemStack::of(Material::EMERALD),
        ]),
        vec![true, true]
    );
    assert_eq!(
        player.get_inventory_ref().get_item_stack(0),
        Some(&occupied_item)
    );

    player.tick();

    assert_eq!(
        read_packet_frame(&mut peer_stream).0,
        spinel_core::network::clientbound::play::set_player_inventory::SetPlayerInventoryPacket::get_id()
    );
    assert_eq!(
        read_packet_frame(&mut peer_stream).0,
        spinel_core::network::clientbound::play::set_player_inventory::SetPlayerInventoryPacket::get_id()
    );
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

#[test]
fn player_schedule_remove_matches_shared_entity_lifecycle() {
    let mut player = Player::new(
        Uuid::new_v4(),
        "ScheduledRemoval".to_owned(),
        0,
        "127.0.0.1:25567".parse().unwrap(),
    );

    assert_eq!(player.get_alive_ticks(), 0);
    assert!(!player.is_removed());
    player.schedule_remove_after_ticks(1);
    player.tick();
    assert_eq!(player.get_alive_ticks(), 1);
    assert!(player.is_removed());

    let mut duration_player = Player::new(
        Uuid::new_v4(),
        "DurationRemoval".to_owned(),
        0,
        "127.0.0.1:25568".parse().unwrap(),
    );
    assert_eq!(duration_player.get_alive_ticks(), 0);
    duration_player.schedule_remove_after_duration(std::time::Duration::from_millis(51));
    duration_player.tick();
    assert_eq!(duration_player.get_alive_ticks(), 1);
    assert!(!duration_player.is_removed());
    duration_player.tick();
    assert_eq!(duration_player.get_alive_ticks(), 2);
    assert!(duration_player.is_removed());
}
