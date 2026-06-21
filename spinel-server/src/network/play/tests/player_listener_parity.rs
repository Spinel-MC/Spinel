use crate::entity::metadata::definitions;
use crate::entity::{Entity, EntityId, EntityPosition, GenericEntity, Player};
use crate::events::entity_attack::EntityAttackEvent;
use crate::events::player_chat::PlayerChatEvent;
use crate::events::player_entity_interact::PlayerEntityInteractEvent;
use crate::events::player_game_mode_request::PlayerGameModeRequestEvent;
use crate::events::player_input::PlayerInputEvent;
use crate::events::player_leave_bed::PlayerLeaveBedEvent;
use crate::events::player_loaded::PlayerLoadedEvent;
use crate::events::player_pick_entity::PlayerPickEntityEvent;
use crate::events::player_plugin_message::PlayerPluginMessageEvent;
use crate::events::player_spectate::PlayerSpectateEvent;
use crate::events::player_stab::PlayerStabEvent;
use crate::events::player_start_flying::PlayerStartFlyingEvent;
use crate::events::player_start_flying_with_elytra::PlayerStartFlyingWithElytraEvent;
use crate::events::player_start_sneaking::PlayerStartSneakingEvent;
use crate::events::player_start_sprinting::PlayerStartSprintingEvent;
use crate::events::player_stop_flying::PlayerStopFlyingEvent;
use crate::events::player_stop_flying_with_elytra::PlayerStopFlyingWithElytraEvent;
use crate::events::player_stop_sneaking::PlayerStopSneakingEvent;
use crate::events::player_stop_sprinting::PlayerStopSprintingEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::{Block, ChunkPosition};
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::play::set_camera::SetCameraPacket;
use spinel_core::network::clientbound::play::sync_player_pos::SyncPlayerPositionPacket;
use spinel_core::network::clientbound::play::system_chat::SystemChatPacket;
use spinel_core::network::serverbound::play::change_game_mode::ChangeGameModePacket;
use spinel_core::network::serverbound::play::chat::ChatPacket;
use spinel_core::network::serverbound::play::chat_command_signed::SignedCommandChatPacket;
use spinel_core::network::serverbound::play::interact::{InteractAction, InteractPacket};
use spinel_core::network::serverbound::play::move_player_pos::MovePlayerPosPacket;
use spinel_core::network::serverbound::play::move_player_pos_rot::MovePlayerPosRotPacket;
use spinel_core::network::serverbound::play::move_player_rot::MovePlayerRotPacket;
use spinel_core::network::serverbound::play::move_player_status_only::MovePlayerStatusOnlyPacket;
use spinel_core::network::serverbound::play::move_vehicle::ServerboundVehicleMovePacket;
use spinel_core::network::serverbound::play::pick_item_from_entity::PickItemFromEntityPacket;
use spinel_core::network::serverbound::play::player_abilities::ServerboundPlayerAbilitiesPacket;
use spinel_core::network::serverbound::play::player_action::PlayerActionPacket;
use spinel_core::network::serverbound::play::player_command::PlayerCommandPacket;
use spinel_core::network::serverbound::play::player_input::PlayerInputPacket;
use spinel_core::network::serverbound::play::player_loaded::PlayerLoadedPacket;
use spinel_core::network::serverbound::play::plugin_message::ServerboundPlayCustomPayloadPacket;
use spinel_core::network::serverbound::play::steer_boat::SteerBoatPacket;
use spinel_core::network::serverbound::play::teleport_to_entity::TeleportToEntityPacket;
use spinel_macros::event_listener;
use spinel_network::types::Identifier;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::{ConnectionState, DataType, PacketStruct, VarIntWrapper};
use spinel_registry::data_components::vanilla_components::PIERCING_WEAPON;
use spinel_registry::{EntityType, ItemStack, Material, PiercingWeapon};
use spinel_utils::component::text::TextComponent;
use std::io::{Cursor, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard};
use uuid::Uuid;

static LISTENER_PARITY_ENABLED: AtomicBool = AtomicBool::new(false);
static LISTENER_PARITY_PLUGIN_MESSAGES: Mutex<Vec<(String, Vec<u8>)>> = Mutex::new(Vec::new());
static LISTENER_PARITY_ATTACKS: Mutex<Vec<(EntityId, EntityId)>> = Mutex::new(Vec::new());
static LISTENER_PARITY_INTERACTIONS: Mutex<Vec<(EntityId, i32, (f32, f32, f32))>> =
    Mutex::new(Vec::new());
static LISTENER_PARITY_ENTITY_PICKS: Mutex<Vec<(Option<EntityId>, bool)>> = Mutex::new(Vec::new());
static LISTENER_PARITY_SPECTATES: Mutex<Vec<EntityId>> = Mutex::new(Vec::new());
static LISTENER_PARITY_CHAT_MESSAGES: Mutex<Vec<String>> = Mutex::new(Vec::new());
static LISTENER_PARITY_GAME_MODE_REQUESTS: Mutex<Vec<GameMode>> = Mutex::new(Vec::new());
static LISTENER_PARITY_START_SPRINTING: AtomicUsize = AtomicUsize::new(0);
static LISTENER_PARITY_STOP_SPRINTING: AtomicUsize = AtomicUsize::new(0);
static LISTENER_PARITY_START_FLYING: AtomicUsize = AtomicUsize::new(0);
static LISTENER_PARITY_STOP_FLYING: AtomicUsize = AtomicUsize::new(0);
static LISTENER_PARITY_START_ELYTRA: AtomicUsize = AtomicUsize::new(0);
static LISTENER_PARITY_STOP_ELYTRA: AtomicUsize = AtomicUsize::new(0);
static LISTENER_PARITY_LEAVE_BED: AtomicUsize = AtomicUsize::new(0);
static LISTENER_PARITY_LOADED: AtomicUsize = AtomicUsize::new(0);
static LISTENER_PARITY_STABS: AtomicUsize = AtomicUsize::new(0);
static LISTENER_PARITY_INPUTS: Mutex<Vec<(bool, bool, bool, bool, bool, bool, bool)>> =
    Mutex::new(Vec::new());
static LISTENER_PARITY_START_SNEAKING: AtomicUsize = AtomicUsize::new(0);
static LISTENER_PARITY_STOP_SNEAKING: AtomicUsize = AtomicUsize::new(0);
static LISTENER_PARITY_LOCK: Mutex<()> = Mutex::new(());

#[event_listener]
fn player_plugin_message_test_listener(
    event: &mut PlayerPluginMessageEvent,
    _server: &mut MinecraftServer,
) {
    if !LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    LISTENER_PARITY_PLUGIN_MESSAGES
        .lock()
        .unwrap()
        .push((event.channel().to_string(), event.data().to_vec()));
}

#[event_listener]
fn entity_attack_test_listener(event: &mut EntityAttackEvent, _server: &mut MinecraftServer) {
    if !LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    LISTENER_PARITY_ATTACKS
        .lock()
        .unwrap()
        .push((event.get_entity_id(), event.target_id()));
}

#[event_listener]
fn player_entity_interact_test_listener(
    event: &mut PlayerEntityInteractEvent,
    _server: &mut MinecraftServer,
) {
    if !LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    LISTENER_PARITY_INTERACTIONS.lock().unwrap().push((
        event.target().get_entity_id(),
        event.hand().get_protocol_id(),
        event.interact_position(),
    ));
}

#[event_listener]
fn player_pick_entity_test_listener(
    event: &mut PlayerPickEntityEvent,
    _server: &mut MinecraftServer,
) {
    if !LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    let target_id = event.target().map(|target| target.get_entity_id());
    LISTENER_PARITY_ENTITY_PICKS
        .lock()
        .unwrap()
        .push((target_id, event.include_data()));
}

#[event_listener]
fn player_stab_test_listener(event: &mut PlayerStabEvent, _server: &mut MinecraftServer) {
    if LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) && event.get_item_stack().has(PIERCING_WEAPON) {
        LISTENER_PARITY_STABS.fetch_add(1, Ordering::SeqCst);
    }
}

#[event_listener]
fn player_spectate_test_listener(event: &mut PlayerSpectateEvent, _server: &mut MinecraftServer) {
    if !LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    LISTENER_PARITY_SPECTATES
        .lock()
        .unwrap()
        .push(event.target_id());
}

#[event_listener]
fn player_game_mode_request_test_listener(
    event: &mut PlayerGameModeRequestEvent,
    _server: &mut MinecraftServer,
) {
    if !LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    LISTENER_PARITY_GAME_MODE_REQUESTS
        .lock()
        .unwrap()
        .push(event.requested_game_mode());
}

#[event_listener]
fn player_start_sprinting_test_listener(
    _event: &mut PlayerStartSprintingEvent,
    _server: &mut MinecraftServer,
) {
    if LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) {
        LISTENER_PARITY_START_SPRINTING.fetch_add(1, Ordering::SeqCst);
    }
}

#[event_listener]
fn player_stop_sprinting_test_listener(
    _event: &mut PlayerStopSprintingEvent,
    _server: &mut MinecraftServer,
) {
    if LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) {
        LISTENER_PARITY_STOP_SPRINTING.fetch_add(1, Ordering::SeqCst);
    }
}

#[event_listener]
fn player_start_flying_test_listener(
    event: &mut PlayerStartFlyingEvent,
    _server: &mut MinecraftServer,
) {
    if LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) && event.player().is_flying() {
        LISTENER_PARITY_START_FLYING.fetch_add(1, Ordering::SeqCst);
    }
}

#[event_listener]
fn player_stop_flying_test_listener(
    event: &mut PlayerStopFlyingEvent,
    _server: &mut MinecraftServer,
) {
    if LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) && !event.player().is_flying() {
        LISTENER_PARITY_STOP_FLYING.fetch_add(1, Ordering::SeqCst);
    }
}

#[event_listener]
fn player_start_flying_with_elytra_test_listener(
    event: &mut PlayerStartFlyingWithElytraEvent,
    _server: &mut MinecraftServer,
) {
    if LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) && event.player().is_flying_with_elytra() {
        LISTENER_PARITY_START_ELYTRA.fetch_add(1, Ordering::SeqCst);
    }
}

#[event_listener]
fn player_stop_flying_with_elytra_test_listener(
    event: &mut PlayerStopFlyingWithElytraEvent,
    _server: &mut MinecraftServer,
) {
    if LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) && !event.player().is_flying_with_elytra() {
        LISTENER_PARITY_STOP_ELYTRA.fetch_add(1, Ordering::SeqCst);
    }
}

#[event_listener]
fn player_leave_bed_test_listener(event: &mut PlayerLeaveBedEvent, _server: &mut MinecraftServer) {
    if LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) {
        LISTENER_PARITY_LEAVE_BED.fetch_add(1, Ordering::SeqCst);
        event.set_cancelled(false);
    }
}

#[event_listener]
fn player_loaded_test_listener(event: &mut PlayerLoadedEvent, _server: &mut MinecraftServer) {
    if LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) && event.player().has_entered_world() {
        LISTENER_PARITY_LOADED.fetch_add(1, Ordering::SeqCst);
    }
}

#[event_listener]
fn player_input_test_listener(event: &mut PlayerInputEvent, _server: &mut MinecraftServer) {
    if !LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    LISTENER_PARITY_INPUTS.lock().unwrap().push((
        event.is_holding_forward_key(),
        event.has_pressed_forward_key(),
        event.has_released_forward_key(),
        event.has_pressed_shift_key(),
        event.has_released_shift_key(),
        event.has_pressed_sprint_key(),
        event.has_released_sprint_key(),
    ));
}

#[event_listener]
fn player_start_sneaking_test_listener(
    _event: &mut PlayerStartSneakingEvent,
    _server: &mut MinecraftServer,
) {
    if LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) {
        LISTENER_PARITY_START_SNEAKING.fetch_add(1, Ordering::SeqCst);
    }
}

#[event_listener]
fn player_stop_sneaking_test_listener(
    _event: &mut PlayerStopSneakingEvent,
    _server: &mut MinecraftServer,
) {
    if LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) {
        LISTENER_PARITY_STOP_SNEAKING.fetch_add(1, Ordering::SeqCst);
    }
}

#[event_listener]
fn player_chat_test_listener(event: &mut PlayerChatEvent, _server: &mut MinecraftServer) {
    if !LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    LISTENER_PARITY_CHAT_MESSAGES
        .lock()
        .unwrap()
        .push(event.raw_message().to_owned());
    event.set_formatted_message(TextComponent::literal("mutated chat"));
}

#[test]
fn play_plugin_message_listener_dispatches_exact_channel_and_raw_payload() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, _world_uuid, _player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);
    let packet = ServerboundPlayCustomPayloadPacket {
        channel: "minecraft:brand".to_string(),
        data: vec![1, 2, 3, 4],
    };

    assert!(dispatch_packet(&mut server, &mut client, packet));

    assert_eq!(
        LISTENER_PARITY_PLUGIN_MESSAGES.lock().unwrap().as_slice(),
        [("minecraft:brand".to_string(), vec![1, 2, 3, 4])]
    );
}

#[test]
fn movement_packet_listeners_preserve_each_minestom_variant_shape() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, world_uuid, _player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        MovePlayerPosPacket {
            x: 1.0,
            y: 65.0,
            z: 2.0,
            flags: MovePlayerPosPacket::FLAG_ON_GROUND,
        },
    ));
    let player = server
        .world_manager
        .world(world_uuid)
        .unwrap()
        .player_by_uuid(Uuid::nil())
        .unwrap();
    assert_eq!(
        (
            player.get_position().get_x(),
            player.get_position().get_y(),
            player.get_position().get_z(),
            player.get_position().get_yaw(),
            player.get_position().get_pitch(),
            player.is_on_ground(),
        ),
        (1.0, 65.0, 2.0, 0.0, 0.0, true)
    );

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        MovePlayerRotPacket {
            y_rot: 90.0,
            x_rot: 30.0,
            flags: 0,
        },
    ));
    let player = server
        .world_manager
        .world(world_uuid)
        .unwrap()
        .player_by_uuid(Uuid::nil())
        .unwrap();
    assert_eq!(
        (
            player.get_position().get_x(),
            player.get_position().get_y(),
            player.get_position().get_z(),
            player.get_position().get_yaw(),
            player.get_position().get_pitch(),
            player.is_on_ground(),
        ),
        (1.0, 65.0, 2.0, 90.0, 30.0, false)
    );

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        MovePlayerPosRotPacket {
            x: 3.0,
            y: 66.0,
            z: 4.0,
            y_rot: 180.0,
            x_rot: 10.0,
            flags: MovePlayerPosRotPacket::FLAG_ON_GROUND,
        },
    ));
    let player = server
        .world_manager
        .world(world_uuid)
        .unwrap()
        .player_by_uuid(Uuid::nil())
        .unwrap();
    assert_eq!(
        (
            player.get_position().get_x(),
            player.get_position().get_y(),
            player.get_position().get_z(),
            player.get_position().get_yaw(),
            player.get_position().get_pitch(),
            player.is_on_ground(),
        ),
        (3.0, 66.0, 4.0, 180.0, 10.0, true)
    );

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        MovePlayerStatusOnlyPacket { flags: 0 },
    ));
    let player = server
        .world_manager
        .world(world_uuid)
        .unwrap()
        .player_by_uuid(Uuid::nil())
        .unwrap();
    assert_eq!(
        (
            player.get_position().get_x(),
            player.get_position().get_y(),
            player.get_position().get_z(),
            player.get_position().get_yaw(),
            player.get_position().get_pitch(),
            player.is_on_ground(),
        ),
        (3.0, 66.0, 4.0, 180.0, 10.0, false)
    );
}

#[test]
fn use_entity_listener_dispatches_attack_and_interact_for_viewable_targets() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, world_uuid, player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);
    let mut target = GenericEntity::new(EntityType::ZOMBIE);
    let target_id = target.get_entity_id();
    target.get_view_mut().manual_add(player_id);
    server
        .world_manager
        .world_mut(world_uuid)
        .unwrap()
        .add_entity(Entity::Generic(target));

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        InteractPacket {
            entity_id: target_id.get_value(),
            action: InteractAction::Attack,
            using_secondary_action: false,
        },
    ));
    assert!(dispatch_packet(
        &mut server,
        &mut client,
        InteractPacket {
            entity_id: target_id.get_value(),
            action: InteractAction::InteractAt {
                target_x: 0.25,
                target_y: 0.5,
                target_z: 0.75,
                hand: 1,
            },
            using_secondary_action: false,
        },
    ));

    assert_eq!(
        LISTENER_PARITY_ATTACKS.lock().unwrap().as_slice(),
        [(player_id, target_id)]
    );
    assert_eq!(
        LISTENER_PARITY_INTERACTIONS.lock().unwrap().as_slice(),
        [(target_id, 1, (0.25, 0.5, 0.75))]
    );
}

#[test]
fn use_entity_listener_enforces_configurable_interaction_range() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, world_uuid, player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);
    let mut target = GenericEntity::new(EntityType::ZOMBIE);
    let target_id = target.get_entity_id();
    target.set_position(EntityPosition::new(32.0, 0.0, 0.0, 0.0, 0.0));
    target.get_view_mut().manual_add(player_id);
    server
        .world_manager
        .world_mut(world_uuid)
        .unwrap()
        .add_entity(Entity::Generic(target));
    let attack_packet = InteractPacket {
        entity_id: target_id.get_value(),
        action: InteractAction::Attack,
        using_secondary_action: false,
    };

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        attack_packet.clone(),
    ));
    assert!(LISTENER_PARITY_ATTACKS.lock().unwrap().is_empty());

    server.enforce_entity_interaction_range = false;
    assert!(dispatch_packet(&mut server, &mut client, attack_packet));
    assert_eq!(
        LISTENER_PARITY_ATTACKS.lock().unwrap().as_slice(),
        [(player_id, target_id)]
    );
}

#[test]
fn player_pick_listener_dispatches_nullable_entity_targets() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, world_uuid, _player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        PickItemFromEntityPacket {
            entity_id: i32::MAX,
            include_data: true,
        },
    ));

    let target = GenericEntity::new(EntityType::ZOMBIE);
    let target_id = target.get_entity_id();
    server
        .world_manager
        .world_mut(world_uuid)
        .unwrap()
        .add_entity(Entity::Generic(target));
    assert!(dispatch_packet(
        &mut server,
        &mut client,
        PickItemFromEntityPacket {
            entity_id: target_id.get_value(),
            include_data: false,
        },
    ));

    assert_eq!(
        LISTENER_PARITY_ENTITY_PICKS.lock().unwrap().as_slice(),
        [(None, true), (Some(target_id), false)]
    );
}

#[test]
fn vehicle_move_listener_refreshes_the_ridden_vehicle_position() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, world_uuid, player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);
    let vehicle = GenericEntity::new(EntityType::MINECART);
    let vehicle_id = vehicle.get_entity_id();
    server
        .world_manager
        .world_mut(world_uuid)
        .unwrap()
        .add_entity(Entity::Generic(vehicle));
    server
        .world_manager
        .add_passenger(vehicle_id, player_id)
        .unwrap();

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        ServerboundVehicleMovePacket {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            yaw: 4.0,
            pitch: 5.0,
            on_ground: true,
        },
    ));

    assert_eq!(
        server
            .world_manager
            .world(world_uuid)
            .unwrap()
            .get_entity(vehicle_id)
            .unwrap()
            .get_position(),
        EntityPosition::new(1.0, 2.0, 3.0, 4.0, 5.0)
    );
}

#[test]
fn steer_boat_listener_refreshes_the_ridden_boat_paddles() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, world_uuid, player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);
    let boat = GenericEntity::new(EntityType::OAK_BOAT);
    let boat_id = boat.get_entity_id();
    server
        .world_manager
        .world_mut(world_uuid)
        .unwrap()
        .add_entity(Entity::Generic(boat));
    server
        .world_manager
        .add_passenger(boat_id, player_id)
        .unwrap();

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        SteerBoatPacket {
            left_paddle_turning: true,
            right_paddle_turning: false,
        },
    ));

    let Entity::Generic(boat) = server
        .world_manager
        .world(world_uuid)
        .unwrap()
        .get_entity(boat_id)
        .unwrap()
    else {
        panic!("expected boat entity");
    };
    assert_eq!(
        boat.get_metadata()
            .get_value(&definitions::boat::is_left_paddle_turning()),
        MetadataValue::Boolean(true)
    );
    assert_eq!(
        boat.get_metadata()
            .get_value(&definitions::boat::is_right_paddle_turning()),
        MetadataValue::Boolean(false)
    );
}

#[test]
fn steer_boat_listener_ignores_non_boat_vehicle_like_minestom() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, world_uuid, player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);
    let minecart = GenericEntity::new(EntityType::MINECART);
    let minecart_id = minecart.get_entity_id();
    server
        .world_manager
        .world_mut(world_uuid)
        .unwrap()
        .add_entity(Entity::Generic(minecart));
    server
        .world_manager
        .add_passenger(minecart_id, player_id)
        .unwrap();

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        SteerBoatPacket {
            left_paddle_turning: true,
            right_paddle_turning: true,
        },
    ));

    let Entity::Generic(minecart) = server
        .world_manager
        .world(world_uuid)
        .unwrap()
        .get_entity(minecart_id)
        .unwrap()
    else {
        panic!("expected minecart entity");
    };
    assert_eq!(
        minecart
            .get_metadata()
            .get_value(&definitions::boat::is_left_paddle_turning()),
        MetadataValue::Boolean(false)
    );
    assert_eq!(
        minecart
            .get_metadata()
            .get_value(&definitions::boat::is_right_paddle_turning()),
        MetadataValue::Boolean(false)
    );
}

#[test]
fn player_action_listener_dispatches_stab_only_for_piercing_weapon() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, _world_uuid, _player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);
    assert!(dispatch_packet(
        &mut server,
        &mut client,
        PlayerActionPacket {
            status: PlayerActionPacket::STAB,
            block_position: spinel_network::types::Position { x: 0, y: 0, z: 0 },
            block_face: 0,
            sequence: 0,
        },
    ));
    assert_eq!(LISTENER_PARITY_STABS.load(Ordering::SeqCst), 0);

    unsafe {
        &mut *server
            .world_manager
            .player_pointer_for_client(&client)
            .unwrap()
    }
    .set_item_in_hand(
        crate::entity::PlayerHand::Main,
        ItemStack::of(Material::TRIDENT).with(
            PIERCING_WEAPON,
            PiercingWeapon::new(true, false, None, None),
        ),
    );
    assert!(dispatch_packet(
        &mut server,
        &mut client,
        PlayerActionPacket {
            status: PlayerActionPacket::STAB,
            block_position: spinel_network::types::Position { x: 0, y: 0, z: 0 },
            block_face: 0,
            sequence: 0,
        },
    ));
    assert_eq!(LISTENER_PARITY_STABS.load(Ordering::SeqCst), 1);
}

#[test]
fn player_action_listener_does_not_acknowledge_unloaded_chunk_digging() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, mut peer_stream, _world_uuid, _player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);

    for status in [
        PlayerActionPacket::START_DESTROY_BLOCK,
        PlayerActionPacket::ABORT_DESTROY_BLOCK,
        PlayerActionPacket::STOP_DESTROY_BLOCK,
    ] {
        assert!(dispatch_packet(
            &mut server,
            &mut client,
            PlayerActionPacket {
                status,
                block_position: spinel_network::types::Position {
                    x: 1024,
                    y: 64,
                    z: 1024,
                },
                block_face: 0,
                sequence: 300,
            },
        ));
    }

    peer_stream.set_nonblocking(true).unwrap();
    let mut packet_byte = [0];
    assert_eq!(
        peer_stream.read(&mut packet_byte).unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}

#[test]
fn failed_digging_correction_teleports_player_when_block_is_under_position() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, mut peer_stream, world_uuid, player_id) =
        server_with_play_player(GameMode::Survival);
    {
        let world = server.world_manager.world_mut(world_uuid).unwrap();
        world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
        world
            .set_block(crate::world::BlockPosition::new(0, 64, 0), Block::STONE)
            .unwrap();
        world
            .get_entity_mut(player_id)
            .unwrap()
            .set_position(EntityPosition::new(0.0, 65.0, 0.0, 0.0, 0.0));
        world
            .load_chunk(ChunkPosition::new(0, 0))
            .unwrap()
            .set_read_only(true);
    }
    attach_client_to_player(&mut server, &mut client);

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        PlayerActionPacket {
            status: PlayerActionPacket::STOP_DESTROY_BLOCK,
            block_position: spinel_network::types::Position { x: 0, y: 64, z: 0 },
            block_face: 1,
            sequence: 300,
        },
    ));

    let _ = read_packet_frame_with_id(&mut peer_stream, SyncPlayerPositionPacket::get_id());
}

#[test]
fn player_command_listener_dispatches_action_events_after_state_changes() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, _world_uuid, _player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        PlayerCommandPacket {
            entity_id: 0,
            action: PlayerCommandPacket::START_SPRINTING,
            data: 0,
        },
    ));
    assert!(dispatch_packet(
        &mut server,
        &mut client,
        PlayerCommandPacket {
            entity_id: 0,
            action: PlayerCommandPacket::STOP_SPRINTING,
            data: 0,
        },
    ));
    assert!(dispatch_packet(
        &mut server,
        &mut client,
        PlayerCommandPacket {
            entity_id: 0,
            action: PlayerCommandPacket::START_FLYING_ELYTRA,
            data: 0,
        },
    ));
    assert!(dispatch_packet(
        &mut server,
        &mut client,
        PlayerCommandPacket {
            entity_id: 0,
            action: PlayerCommandPacket::LEAVE_BED,
            data: 0,
        },
    ));

    assert_eq!(LISTENER_PARITY_START_SPRINTING.load(Ordering::SeqCst), 1);
    assert_eq!(LISTENER_PARITY_STOP_SPRINTING.load(Ordering::SeqCst), 1);
    assert_eq!(LISTENER_PARITY_START_ELYTRA.load(Ordering::SeqCst), 1);
    assert_eq!(LISTENER_PARITY_LEAVE_BED.load(Ordering::SeqCst), 1);
}

#[test]
fn player_status_listener_dispatches_stop_flying_with_elytra_after_landing() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, world_uuid, player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        PlayerCommandPacket {
            entity_id: 0,
            action: PlayerCommandPacket::START_FLYING_ELYTRA,
            data: 0,
        },
    ));
    assert!(dispatch_packet(
        &mut server,
        &mut client,
        MovePlayerStatusOnlyPacket {
            flags: MovePlayerStatusOnlyPacket::FLAG_ON_GROUND,
        },
    ));

    let player = server
        .world_manager
        .world(world_uuid)
        .unwrap()
        .get_entity(player_id)
        .unwrap();
    assert!(!matches!(player, Entity::Player(player) if player.is_flying_with_elytra()));
    assert_eq!(LISTENER_PARITY_START_ELYTRA.load(Ordering::SeqCst), 1);
    assert_eq!(LISTENER_PARITY_STOP_ELYTRA.load(Ordering::SeqCst), 1);
}

#[test]
fn player_input_listener_dispatches_current_and_transition_state_after_refresh() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, _world_uuid, _player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        PlayerInputPacket { flags: 0x61 },
    ));
    assert!(dispatch_packet(
        &mut server,
        &mut client,
        PlayerInputPacket { flags: 0x61 },
    ));
    assert!(dispatch_packet(
        &mut server,
        &mut client,
        PlayerInputPacket { flags: 0 },
    ));

    assert_eq!(
        LISTENER_PARITY_INPUTS.lock().unwrap().as_slice(),
        [
            (true, true, false, true, false, true, false),
            (true, false, false, false, false, false, false),
            (false, false, true, false, true, false, true),
        ]
    );
    assert_eq!(LISTENER_PARITY_START_SNEAKING.load(Ordering::SeqCst), 1);
    assert_eq!(LISTENER_PARITY_STOP_SNEAKING.load(Ordering::SeqCst), 1);
}

#[test]
fn player_loaded_listener_dispatches_after_client_loaded_packet() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, _world_uuid, _player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        PlayerLoadedPacket
    ));

    assert_eq!(LISTENER_PARITY_LOADED.load(Ordering::SeqCst), 1);
}

#[test]
fn change_game_mode_listener_dispatches_request_without_mutating_player_mode() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, world_uuid, player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        ChangeGameModePacket {
            game_mode: GameMode::Creative,
        },
    ));

    let player = server
        .world_manager
        .world(world_uuid)
        .unwrap()
        .get_entity(player_id)
        .unwrap();
    assert!(matches!(player, Entity::Player(player) if player.get_game_mode() == GameMode::Survival));
    assert_eq!(
        LISTENER_PARITY_GAME_MODE_REQUESTS
            .lock()
            .unwrap()
            .as_slice(),
        [GameMode::Creative]
    );
}

#[test]
fn player_abilities_listener_dispatches_flying_events_after_can_fly_gate() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, world_uuid, player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        ServerboundPlayerAbilitiesPacket {
            flags: ServerboundPlayerAbilitiesPacket::FLYING,
        },
    ));
    let player = server
        .world_manager
        .world(world_uuid)
        .unwrap()
        .get_entity(player_id)
        .unwrap();
    assert!(!matches!(player, Entity::Player(player) if player.is_flying()));
    assert_eq!(LISTENER_PARITY_START_FLYING.load(Ordering::SeqCst), 0);

    let player = server
        .world_manager
        .player_pointer_for_client(&client)
        .unwrap();
    unsafe { &mut *player }.set_allow_flying(true).unwrap();
    assert!(dispatch_packet(
        &mut server,
        &mut client,
        ServerboundPlayerAbilitiesPacket {
            flags: ServerboundPlayerAbilitiesPacket::FLYING,
        },
    ));
    assert!(dispatch_packet(
        &mut server,
        &mut client,
        ServerboundPlayerAbilitiesPacket { flags: 0 },
    ));

    let player = server
        .world_manager
        .world(world_uuid)
        .unwrap()
        .get_entity(player_id)
        .unwrap();
    assert!(!matches!(player, Entity::Player(player) if player.is_flying()));
    assert_eq!(LISTENER_PARITY_START_FLYING.load(Ordering::SeqCst), 1);
    assert_eq!(LISTENER_PARITY_STOP_FLYING.load(Ordering::SeqCst), 1);
}

#[test]
fn spectate_listener_requires_spectator_target_in_same_world_and_sets_camera() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, mut peer_stream, world_uuid, _player_id) =
        server_with_play_player(GameMode::Spectator);
    attach_client_to_player(&mut server, &mut client);
    let target = GenericEntity::new(EntityType::ZOMBIE);
    let target_id = target.get_entity_id();
    let target_uuid = target.get_uuid();
    server
        .world_manager
        .world_mut(world_uuid)
        .unwrap()
        .add_entity(Entity::Generic(target));

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        TeleportToEntityPacket {
            target: target_uuid,
        },
    ));

    let (packet_id, payload) =
        read_packet_frame_with_id(&mut peer_stream, SetCameraPacket::get_id());
    let camera_packet = SetCameraPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(packet_id, SetCameraPacket::get_id());
    assert_eq!(camera_packet.camera_id, target_id.get_value());
    assert_eq!(
        LISTENER_PARITY_SPECTATES.lock().unwrap().as_slice(),
        [target_id]
    );
}

#[test]
fn chat_message_listener_dispatches_event_and_sends_mutated_message() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, mut peer_stream, _world_uuid, _player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        ChatPacket {
            message: "raw chat".to_owned(),
            timestamp: 1,
            salt: 2,
            signature: None,
            ack_offset: 0,
            ack_list: [0, 0, 0],
            checksum: 0,
        },
    ));

    let (packet_id, payload) = read_packet_frame(&mut peer_stream);
    let chat_packet = SystemChatPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(packet_id, SystemChatPacket::get_id());
    assert_eq!(chat_packet.message.to_plain_string(), "mutated chat");
    assert_eq!(
        LISTENER_PARITY_CHAT_MESSAGES.lock().unwrap().as_slice(),
        ["raw chat".to_owned()]
    );
}

#[test]
fn signed_command_chat_listener_uses_minestom_chat_rejection_gate() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, mut peer_stream, _world_uuid, _player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);
    let player = server
        .world_manager
        .player_pointer_for_client(&client)
        .unwrap();
    let mut settings = unsafe { &*player }.get_settings().clone();
    settings.chat_mode = 2;
    unsafe { &mut *player }.refresh_settings(settings);

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        SignedCommandChatPacket {
            command: "help".to_owned(),
            timestamp: 1,
            salt: 2,
            signatures: Vec::new(),
            ack_offset: 0,
            ack_list: [0, 0, 0],
            checksum: 0,
        },
    ));

    let (packet_id, payload) = read_packet_frame(&mut peer_stream);
    let chat_packet = SystemChatPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(packet_id, SystemChatPacket::get_id());
    assert!(
        chat_packet
            .message
            .to_json_string()
            .contains("chat.cannotSend")
    );
}

struct ListenerParityScope {
    _lock: MutexGuard<'static, ()>,
}

impl ListenerParityScope {
    fn new() -> Self {
        let lock = LISTENER_PARITY_LOCK.lock().unwrap();
        LISTENER_PARITY_PLUGIN_MESSAGES.lock().unwrap().clear();
        LISTENER_PARITY_ATTACKS.lock().unwrap().clear();
        LISTENER_PARITY_INTERACTIONS.lock().unwrap().clear();
        LISTENER_PARITY_ENTITY_PICKS.lock().unwrap().clear();
        LISTENER_PARITY_SPECTATES.lock().unwrap().clear();
        LISTENER_PARITY_CHAT_MESSAGES.lock().unwrap().clear();
        LISTENER_PARITY_GAME_MODE_REQUESTS.lock().unwrap().clear();
        LISTENER_PARITY_START_SPRINTING.store(0, Ordering::SeqCst);
        LISTENER_PARITY_STOP_SPRINTING.store(0, Ordering::SeqCst);
        LISTENER_PARITY_START_FLYING.store(0, Ordering::SeqCst);
        LISTENER_PARITY_STOP_FLYING.store(0, Ordering::SeqCst);
        LISTENER_PARITY_START_ELYTRA.store(0, Ordering::SeqCst);
        LISTENER_PARITY_STOP_ELYTRA.store(0, Ordering::SeqCst);
        LISTENER_PARITY_LEAVE_BED.store(0, Ordering::SeqCst);
        LISTENER_PARITY_LOADED.store(0, Ordering::SeqCst);
        LISTENER_PARITY_STABS.store(0, Ordering::SeqCst);
        LISTENER_PARITY_INPUTS.lock().unwrap().clear();
        LISTENER_PARITY_START_SNEAKING.store(0, Ordering::SeqCst);
        LISTENER_PARITY_STOP_SNEAKING.store(0, Ordering::SeqCst);
        LISTENER_PARITY_ENABLED.store(true, Ordering::SeqCst);
        Self { _lock: lock }
    }
}

impl Drop for ListenerParityScope {
    fn drop(&mut self) {
        LISTENER_PARITY_ENABLED.store(false, Ordering::SeqCst);
    }
}

fn server_with_play_player(
    game_mode: GameMode,
) -> (MinecraftServer, Client, TcpStream, Uuid, EntityId) {
    let mut server = MinecraftServer::new();
    let (mut client, peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let mut player = Player::new(Uuid::nil(), "Listener".to_string(), 0, client.addr);
    player.set_game_mode(game_mode);
    player.mark_entered_world();
    let player_id = player.get_entity_id();
    server
        .world_manager
        .add_entity(world_uuid, Entity::Player(player));
    (server, client, peer_stream, world_uuid, player_id)
}

fn attach_client_to_player(server: &mut MinecraftServer, client: &mut Client) {
    let player = server
        .world_manager
        .player_pointer_for_client(client)
        .unwrap();
    unsafe { &mut *player }.set_client(client);
}

fn dispatch_packet<T: DataType + PacketStruct>(
    server: &mut MinecraftServer,
    client: &mut Client,
    packet: T,
) -> bool {
    let mut payload = Vec::new();
    packet.encode(&mut payload).unwrap();
    server.dispatch_packet(T::get_id(), client, payload)
}

fn test_client_pair() -> (Client, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = TcpStream::connect(addr).unwrap();
    let (peer_stream, _) = listener.accept().unwrap();
    peer_stream
        .set_read_timeout(Some(std::time::Duration::from_secs(1)))
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

fn read_packet_frame_with_id(
    peer_stream: &mut TcpStream,
    expected_packet_id: i32,
) -> (i32, Vec<u8>) {
    loop {
        let packet = read_packet_frame(peer_stream);
        if packet.0 == expected_packet_id {
            return packet;
        }
    }
}
