use crate::entity::{Entity, EntityId, GenericEntity, Player};
use crate::events::entity_attack::EntityAttackEvent;
use crate::events::player_chat::PlayerChatEvent;
use crate::events::player_entity_interact::PlayerEntityInteractEvent;
use crate::events::player_leave_bed::PlayerLeaveBedEvent;
use crate::events::player_plugin_message::PlayerPluginMessageEvent;
use crate::events::player_spectate::PlayerSpectateEvent;
use crate::events::player_start_flying_with_elytra::PlayerStartFlyingWithElytraEvent;
use crate::events::player_start_sprinting::PlayerStartSprintingEvent;
use crate::events::player_stop_sprinting::PlayerStopSprintingEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::play::set_camera::SetCameraPacket;
use spinel_core::network::clientbound::play::system_chat::SystemChatPacket;
use spinel_core::network::serverbound::play::chat::ChatPacket;
use spinel_core::network::serverbound::play::chat_command_signed::SignedCommandChatPacket;
use spinel_core::network::serverbound::play::interact::{InteractAction, InteractPacket};
use spinel_core::network::serverbound::play::player_command::PlayerCommandPacket;
use spinel_core::network::serverbound::play::plugin_message::ServerboundPlayCustomPayloadPacket;
use spinel_core::network::serverbound::play::teleport_to_entity::TeleportToEntityPacket;
use spinel_macros::event_listener;
use spinel_network::types::Identifier;
use spinel_network::{ConnectionState, DataType, PacketStruct, VarIntWrapper};
use spinel_registry::EntityType;
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
static LISTENER_PARITY_SPECTATES: Mutex<Vec<EntityId>> = Mutex::new(Vec::new());
static LISTENER_PARITY_CHAT_MESSAGES: Mutex<Vec<String>> = Mutex::new(Vec::new());
static LISTENER_PARITY_START_SPRINTING: AtomicUsize = AtomicUsize::new(0);
static LISTENER_PARITY_STOP_SPRINTING: AtomicUsize = AtomicUsize::new(0);
static LISTENER_PARITY_ELYTRA: AtomicUsize = AtomicUsize::new(0);
static LISTENER_PARITY_LEAVE_BED: AtomicUsize = AtomicUsize::new(0);
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
        .push((event.entity_id(), event.target_id()));
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
        event.target_id(),
        event.hand().protocol_id(),
        event.interact_position(),
    ));
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
fn player_start_flying_with_elytra_test_listener(
    _event: &mut PlayerStartFlyingWithElytraEvent,
    _server: &mut MinecraftServer,
) {
    if LISTENER_PARITY_ENABLED.load(Ordering::SeqCst) {
        LISTENER_PARITY_ELYTRA.fetch_add(1, Ordering::SeqCst);
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
fn use_entity_listener_dispatches_attack_and_interact_for_viewable_targets() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, _peer_stream, world_uuid, player_id) =
        server_with_play_player(GameMode::Survival);
    attach_client_to_player(&mut server, &mut client);
    let mut target = GenericEntity::new(EntityType::ZOMBIE);
    let target_id = target.entity_id();
    target.add_viewer(player_id);
    server
        .world_manager
        .world_mut(world_uuid)
        .unwrap()
        .add_entity(Entity::Generic(target));

    assert!(dispatch_packet(
        &mut server,
        &mut client,
        InteractPacket {
            entity_id: target_id.value(),
            action: InteractAction::Attack,
            using_secondary_action: false,
        },
    ));
    assert!(dispatch_packet(
        &mut server,
        &mut client,
        InteractPacket {
            entity_id: target_id.value(),
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
    assert_eq!(LISTENER_PARITY_ELYTRA.load(Ordering::SeqCst), 1);
    assert_eq!(LISTENER_PARITY_LEAVE_BED.load(Ordering::SeqCst), 1);
}

#[test]
fn spectate_listener_requires_spectator_target_in_same_world_and_sets_camera() {
    let _scope = ListenerParityScope::new();
    let (mut server, mut client, mut peer_stream, world_uuid, _player_id) =
        server_with_play_player(GameMode::Spectator);
    attach_client_to_player(&mut server, &mut client);
    let target = GenericEntity::new(EntityType::ZOMBIE);
    let target_id = target.entity_id();
    let target_uuid = target.uuid();
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
    assert_eq!(camera_packet.camera_id, target_id.value());
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
    let mut settings = unsafe { &*player }.settings().clone();
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
        LISTENER_PARITY_SPECTATES.lock().unwrap().clear();
        LISTENER_PARITY_CHAT_MESSAGES.lock().unwrap().clear();
        LISTENER_PARITY_START_SPRINTING.store(0, Ordering::SeqCst);
        LISTENER_PARITY_STOP_SPRINTING.store(0, Ordering::SeqCst);
        LISTENER_PARITY_ELYTRA.store(0, Ordering::SeqCst);
        LISTENER_PARITY_LEAVE_BED.store(0, Ordering::SeqCst);
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
    let player_id = player.entity_id();
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
