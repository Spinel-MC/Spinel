use crate::events::network::packet::PacketEvent;
use crate::events::network::packet_error::{PacketErrorEvent, PacketErrorStage};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::query_entity_tag::QueryEntityTagPacket;
use spinel_macros::event_listener;
use spinel_network::{ConnectionState, Recipient};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::Mutex;

static PACKET_EVENT_RECIPIENT: Mutex<Option<Recipient>> = Mutex::new(None);
static PACKET_ERROR_EVENT: Mutex<Option<(Recipient, PacketErrorStage)>> = Mutex::new(None);

#[event_listener]
fn capture_packet_event(event: &mut PacketEvent, _server: &mut MinecraftServer) {
    if event.id != QueryEntityTagPacket::get_id() {
        return;
    }
    *PACKET_EVENT_RECIPIENT.lock().unwrap() = Some(event.recipient);
}

#[event_listener]
fn capture_packet_error_event(event: &mut PacketErrorEvent, _server: &mut MinecraftServer) {
    if event.packet_id != Some(QueryEntityTagPacket::get_id()) {
        return;
    }
    *PACKET_ERROR_EVENT.lock().unwrap() = Some((event.recipient, event.stage));
}

#[test]
fn defined_serverbound_packet_is_accepted_without_a_gameplay_listener() {
    *PACKET_EVENT_RECIPIENT.lock().unwrap() = None;
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    client.state = ConnectionState::Play;
    let packet = QueryEntityTagPacket {
        transaction_id: 17,
        entity_id: 23,
    };
    let payload = packet.encode_to_buffer().unwrap().into_buffer();

    assert!(!server.has_listener_for(QueryEntityTagPacket::get_id(), &ConnectionState::Play));
    assert!(server.dispatch_packet(QueryEntityTagPacket::get_id(), &mut client, payload));
    assert_eq!(
        *PACKET_EVENT_RECIPIENT.lock().unwrap(),
        Some(Recipient::Server)
    );
}

#[test]
fn malformed_serverbound_packet_dispatches_packet_error_event() {
    *PACKET_ERROR_EVENT.lock().unwrap() = None;
    let mut server = MinecraftServer::new();
    let mut client = test_client();
    client.state = ConnectionState::Play;

    assert!(!server.dispatch_packet(QueryEntityTagPacket::get_id(), &mut client, vec![0x80]));
    assert_eq!(
        *PACKET_ERROR_EVENT.lock().unwrap(),
        Some((Recipient::Server, PacketErrorStage::PacketDecode))
    );
}

fn test_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let address = listener.local_addr().unwrap();
    let stream = TcpStream::connect(address).unwrap();
    let _ = listener.accept().unwrap();
    Client::new(stream, address)
}
