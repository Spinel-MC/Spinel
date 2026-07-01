use crate::entity::{Entity, Player};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_network::ConnectionState;
use spinel_network::DataType;
use spinel_network::VarIntWrapper;
use spinel_network::types::Identifier;
use std::io::{ErrorKind, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use uuid::Uuid;

#[test]
fn play_state_connection_does_not_send_keep_alive_before_player_enters_world() {
    let (mut client, mut peer_stream) = test_client_pair();
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let player = Player::new(Uuid::new_v4(), "Pending".to_string(), 0, client.addr);
    client.state = ConnectionState::Play;
    server
        .world_manager
        .add_entity(world_uuid, Entity::Player(player));

    let client_address = client.addr;
    server
        .connection_manager
        .add_connection(client_address, Arc::new(Mutex::new(client)));
    server.tick_connections();

    assert_no_packet_is_available(&mut peer_stream);
}

fn test_client_pair() -> (Client, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let address = listener.local_addr().unwrap();
    let stream = TcpStream::connect(address).unwrap();
    let (peer_stream, _) = listener.accept().unwrap();
    (Client::new(stream, address), peer_stream)
}

fn assert_no_packet_is_available(peer_stream: &mut TcpStream) {
    peer_stream
        .set_read_timeout(Some(Duration::from_millis(100)))
        .unwrap();
    match VarIntWrapper::decode(peer_stream) {
        Ok(_) => panic!("connection unexpectedly sent a packet"),
        Err(error)
            if matches!(
                error.kind(),
                ErrorKind::WouldBlock | ErrorKind::TimedOut | ErrorKind::UnexpectedEof
            ) => {}
        Err(error) => panic!("unexpected packet read failure: {error}"),
    }
    let mut trailing_byte = [0];
    assert!(peer_stream.read(&mut trailing_byte).is_err());
}
