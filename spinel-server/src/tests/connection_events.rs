use crate::entity::{Entity, Player};
use crate::events::player_disconnect::PlayerDisconnectEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::play::disconnect::PlayDisconnectPacket;
use spinel_macros::event_listener;
use spinel_network::types::Identifier;
use spinel_network::{ConnectionState, DataType, PacketDecoder, VarIntWrapper};
use spinel_utils::component::Component;
use std::io::Cursor;
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use uuid::Uuid;

static PLAYER_DISCONNECT_EVENT_TARGET: Mutex<Option<Uuid>> = Mutex::new(None);
static PLAYER_DISCONNECT_EVENT_SEEN: Mutex<Option<Uuid>> = Mutex::new(None);

#[event_listener]
fn player_disconnect_test_listener(
    event: &mut PlayerDisconnectEvent,
    _server: &mut MinecraftServer,
) {
    let player_uuid = event.player().get_uuid();
    if *PLAYER_DISCONNECT_EVENT_TARGET.lock().unwrap() != Some(player_uuid) {
        return;
    }
    *PLAYER_DISCONNECT_EVENT_SEEN.lock().unwrap() = Some(player_uuid);
}

#[test]
fn player_disconnect_event_fires_before_player_is_removed_from_world() {
    *PLAYER_DISCONNECT_EVENT_TARGET.lock().unwrap() = None;
    *PLAYER_DISCONNECT_EVENT_SEEN.lock().unwrap() = None;
    let mut server = MinecraftServer::new();
    let (client, mut peer_stream) = test_client_pair();
    let client_address = client.addr;
    let client = Arc::new(Mutex::new(client));
    let player_uuid = Uuid::new_v4();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let player = Player::new(player_uuid, "Disconnecting".to_string(), 0, client_address);
    server
        .world_manager
        .add_entity(world_uuid, Entity::Player(player));
    assert!(!server.on_connection(client));
    *PLAYER_DISCONNECT_EVENT_TARGET.lock().unwrap() = Some(player_uuid);

    server.handle_connection_closed(client_address);

    peer_stream
        .set_read_timeout(Some(Duration::from_millis(25)))
        .unwrap();
    let mut trailing_packet_byte = [0u8; 1];
    let trailing_packet_read = peer_stream.read(&mut trailing_packet_byte);
    assert_eq!(
        *PLAYER_DISCONNECT_EVENT_SEEN.lock().unwrap(),
        Some(player_uuid)
    );
    assert!(matches!(trailing_packet_read, Ok(0) | Err(_)));
    assert!(
        server
            .world_manager
            .world(world_uuid)
            .unwrap()
            .player_by_uuid(player_uuid)
            .is_none()
    );
    *PLAYER_DISCONNECT_EVENT_TARGET.lock().unwrap() = None;
    *PLAYER_DISCONNECT_EVENT_SEEN.lock().unwrap() = None;
}

#[test]
fn connection_manager_removal_does_not_send_a_disconnect_packet() {
    let mut server = MinecraftServer::new();
    let (client, mut peer_stream) = test_client_pair();
    let client_address = client.addr;
    let client = Arc::new(Mutex::new(client));
    server
        .connection_manager
        .add_connection(client_address, client);

    server.connection_manager.remove_connection(&client_address);

    peer_stream
        .set_read_timeout(Some(Duration::from_millis(25)))
        .unwrap();
    let mut packet_byte = [0u8; 1];
    assert!(matches!(peer_stream.read(&mut packet_byte), Ok(0) | Err(_)));
    assert!(!server.connection_manager.has_connection(&client_address));
}

#[test]
fn connection_tick_removes_an_offline_client_without_waiting_for_keep_alive_timeout() {
    let mut server = MinecraftServer::new();
    let (mut client, _peer_stream) = test_client_pair();
    let client_address = client.addr;
    client.state = ConnectionState::Play;
    client.close_connection();
    server
        .connection_manager
        .add_connection(client_address, Arc::new(Mutex::new(client)));

    server.tick_connections();

    assert!(!server.connection_manager.has_connection(&client_address));
}

#[test]
fn server_kick_sends_play_disconnect_packet_before_removing_connection() {
    let mut server = MinecraftServer::new();
    let (mut client, mut peer_stream) = test_client_pair();
    client.state = ConnectionState::Play;
    let client_address = client.addr;
    let registered_client = Arc::new(Mutex::new(client.stream.try_clone().unwrap()));
    let managed_client = Client::new(
        registered_client.lock().unwrap().try_clone().unwrap(),
        client_address,
    );
    server
        .connection_manager
        .add_connection(client_address, Arc::new(Mutex::new(managed_client)));

    server
        .kick(&mut client, Component::text("Removed by server"))
        .unwrap();

    peer_stream
        .set_read_timeout(Some(Duration::from_millis(100)))
        .unwrap();
    let frame = PacketDecoder::new().read_frame(&mut peer_stream).unwrap();
    let mut frame = Cursor::new(frame);
    let packet_id = VarIntWrapper::decode(&mut frame).unwrap().0;
    assert_eq!(packet_id, PlayDisconnectPacket::get_id());
    assert!(!server.connection_manager.has_connection(&client_address));
}

fn test_client_pair() -> (Client, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = TcpStream::connect(addr).unwrap();
    let (peer_stream, _) = listener.accept().unwrap();
    (Client::new(stream, addr), peer_stream)
}
