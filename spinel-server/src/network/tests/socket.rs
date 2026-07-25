use crate::network::Client;
use crate::network::socket::packet_should_be_queued;
use spinel_core::network::serverbound::play::keep_alive::KeepAlivePacket;
use spinel_core::network::serverbound::play::use_item::UseItemPacket;
use spinel_network::ConnectionState;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};

#[test]
fn use_item_packet_is_not_queued_during_play() {
    let mut client = test_client();
    client.state = ConnectionState::Play;

    assert!(!packet_should_be_queued(&client, UseItemPacket::get_id()));
}

#[test]
fn keep_alive_packet_is_not_queued_during_play() {
    let mut client = test_client();
    client.state = ConnectionState::Play;

    assert!(!packet_should_be_queued(&client, KeepAlivePacket::get_id()));
}

#[test]
fn ordinary_play_packets_still_queue() {
    let mut client = test_client();
    client.state = ConnectionState::Play;

    assert!(packet_should_be_queued(&client, -1));
}

#[test]
fn non_play_packets_do_not_queue() {
    let client = test_client();

    assert!(!packet_should_be_queued(&client, -1));
}

fn test_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let address = listener.local_addr().unwrap();
    let stream = TcpStream::connect(address).unwrap();
    let _ = listener.accept().unwrap();
    Client::new(stream, address)
}
