use crate::network::client::instance::Client;
use spinel_network::{ConnectionState, PacketSender};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc;
use std::time::Duration;

#[test]
fn outbound_queue_preserves_packets_beyond_minestom_inbound_limit() {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let address = listener.local_addr().unwrap();
    let stream = TcpStream::connect(address).unwrap();
    let (_peer_stream, _) = listener.accept().unwrap();
    let mut client = Client::new(stream, address);
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    let packet_payload = vec![0; 32];

    (0..1_100).for_each(|_| {
        client.send_packet(44, &packet_payload).unwrap();
    });

    assert_eq!(client.pending_outbound_packet_count(), 1_100);
}

#[test]
fn flushing_play_packets_does_not_wait_for_the_peer_to_read() {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let address = listener.local_addr().unwrap();
    let stream = TcpStream::connect(address).unwrap();
    let (_peer_stream, _) = listener.accept().unwrap();
    let mut client = Client::new(stream, address);
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    client.set_compression(256);
    let packet_payload = vec![0; 1_048_576];

    (0..128).for_each(|_| {
        client.send_packet(44, &packet_payload).unwrap();
    });

    let (completion_sender, completion_receiver) = mpsc::channel();
    std::thread::spawn(move || {
        let flush_result = client.flush_outbound_packets();
        completion_sender.send(flush_result).unwrap();
    });

    let flush_result = completion_receiver
        .recv_timeout(Duration::from_millis(500))
        .expect("outbound flush blocked while the peer was not reading");

    assert_eq!(flush_result.unwrap(), 128);
}

#[test]
fn closed_connection_discards_buffered_play_packets_without_writer_errors() {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let address = listener.local_addr().unwrap();
    let stream = TcpStream::connect(address).unwrap();
    let (_peer_stream, _) = listener.accept().unwrap();
    let mut client = Client::new(stream, address);
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();

    client.send_packet(44, &[1, 2, 3]).unwrap();
    client.close_connection();

    assert!(client.send_packet(44, &[4, 5, 6]).is_err());
    assert_eq!(client.queued_outbound_packet_count(), 0);
    assert_eq!(client.flush_outbound_packets().unwrap(), 0);
}

#[test]
fn closing_connection_cancels_pending_compression_work() {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let address = listener.local_addr().unwrap();
    let stream = TcpStream::connect(address).unwrap();
    let (_peer_stream, _) = listener.accept().unwrap();
    let mut client = Client::new(stream, address);
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    client.set_compression(256);
    let packet_payload = vec![0; 1_048_576];

    (0..128).for_each(|_| {
        client.send_packet(44, &packet_payload).unwrap();
    });
    client.flush_outbound_packets().unwrap();
    client.close_connection();

    let cancellation_deadline = std::time::Instant::now() + Duration::from_millis(500);
    while client.pending_outbound_packet_count() > 0
        && std::time::Instant::now() < cancellation_deadline
    {
        std::thread::yield_now();
    }

    assert_eq!(client.pending_outbound_packet_count(), 0);
}
