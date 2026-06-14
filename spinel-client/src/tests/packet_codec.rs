use crate::events::network::packet_error::{PacketErrorEvent, PacketErrorStage};
use crate::instance::MinecraftClient;
use crate::network::server::instance::Server;
use spinel_core::network::clientbound::play::clear_dialog::ClearDialogPacket;
use spinel_macros::event_listener;
use spinel_network::{ConnectionState, Recipient};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::Mutex;

static PACKET_ERROR_EVENT: Mutex<Option<(Recipient, PacketErrorStage)>> = Mutex::new(None);

#[event_listener]
fn capture_packet_error_event(event: &mut PacketErrorEvent, _client: &mut MinecraftClient) {
    if event.packet_id != Some(ClearDialogPacket::get_id()) {
        return;
    }
    *PACKET_ERROR_EVENT.lock().unwrap() = Some((event.recipient, event.stage));
}

#[test]
fn defined_clientbound_packet_is_accepted_without_a_gameplay_listener() {
    let client = MinecraftClient::new();
    let mut server = test_server();
    server.state = ConnectionState::Play;
    let payload = ClearDialogPacket.encode_to_buffer().unwrap().into_buffer();

    assert!(!client.has_listener_for(ClearDialogPacket::get_id(), &ConnectionState::Play));
    assert!(client.dispatch_packet(ClearDialogPacket::get_id(), &mut server, payload));
}

#[test]
fn malformed_clientbound_packet_dispatches_packet_error_event() {
    *PACKET_ERROR_EVENT.lock().unwrap() = None;
    let client = MinecraftClient::new();
    let mut server = test_server();
    server.state = ConnectionState::Play;

    assert!(!client.dispatch_packet(ClearDialogPacket::get_id(), &mut server, vec![0]));
    assert_eq!(
        *PACKET_ERROR_EVENT.lock().unwrap(),
        Some((Recipient::Client, PacketErrorStage::PacketDecode))
    );
}

fn test_server() -> Server {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let address = listener.local_addr().unwrap();
    let stream = TcpStream::connect(address).unwrap();
    let _ = listener.accept().unwrap();
    Server::new(stream, address)
}
