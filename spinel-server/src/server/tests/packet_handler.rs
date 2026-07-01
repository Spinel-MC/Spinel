use crate::network::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::client_command::ClientCommandPacket;
use spinel_macros::packet_listener;
use spinel_network::ConnectionState;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::Mutex;

static REGISTERED_PACKET_HANDLER_ACTION: Mutex<Option<i32>> = Mutex::new(None);
static GLOBAL_PACKET_HANDLER_ACTION: Mutex<Option<i32>> = Mutex::new(None);

struct ClientCommandPacketHandler;

#[packet_listener]
impl ClientCommandPacketHandler {
    #[packet_handler]
    pub fn capture_client_command(
        _client: &mut Client,
        packet: ClientCommandPacket,
        _server: &mut MinecraftServer,
    ) -> bool {
        *REGISTERED_PACKET_HANDLER_ACTION.lock().unwrap() = Some(packet.action);
        true
    }
}

#[test]
fn registered_packet_handler_dispatches_from_server_packet_registry() {
    *REGISTERED_PACKET_HANDLER_ACTION.lock().unwrap() = None;
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    client.state = ConnectionState::Play;
    let packet = ClientCommandPacket {
        action: ClientCommandPacket::REQUEST_STATS,
    };
    let payload = packet.encode_to_buffer().unwrap().into_buffer();

    server.register_packet_handler(ClientCommandPacketHandler);

    assert!(server.has_listener_for(ClientCommandPacket::get_id(), &ConnectionState::Play));
    assert!(server.dispatch_packet(ClientCommandPacket::get_id(), &mut client, payload));
    assert_eq!(
        *REGISTERED_PACKET_HANDLER_ACTION.lock().unwrap(),
        Some(ClientCommandPacket::REQUEST_STATS)
    );
}

fn capture_global_client_command(
    _client: &mut Client,
    packet: ClientCommandPacket,
    _server: &mut MinecraftServer,
) -> bool {
    *GLOBAL_PACKET_HANDLER_ACTION.lock().unwrap() = Some(packet.action);
    true
}

#[test]
fn global_packet_handler_listener_dispatches_from_server_packet_registry() {
    *GLOBAL_PACKET_HANDLER_ACTION.lock().unwrap() = None;
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    client.state = ConnectionState::Play;
    let packet = ClientCommandPacket {
        action: ClientCommandPacket::PERFORM_RESPAWN,
    };
    let payload = packet.encode_to_buffer().unwrap().into_buffer();

    server
        .get_global_packet_handler()
        .add_listener::<ClientCommandPacket>(capture_global_client_command);

    assert!(server.has_listener_for(ClientCommandPacket::get_id(), &ConnectionState::Play));
    assert!(server.dispatch_packet(ClientCommandPacket::get_id(), &mut client, payload));
    assert_eq!(
        *GLOBAL_PACKET_HANDLER_ACTION.lock().unwrap(),
        Some(ClientCommandPacket::PERFORM_RESPAWN)
    );
}

fn test_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let address = listener.local_addr().unwrap();
    let stream = TcpStream::connect(address).unwrap();
    let _ = listener.accept().unwrap();
    Client::new(stream, address)
}
