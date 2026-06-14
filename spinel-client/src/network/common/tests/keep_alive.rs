use crate::instance::MinecraftClient;
use crate::network::server::instance::Server;
use spinel_core::network::clientbound::play::keep_alive::KeepAlivePacket as ClientboundKeepAlivePacket;
use spinel_core::network::serverbound::play::keep_alive::KeepAlivePacket as ServerboundKeepAlivePacket;
use spinel_network::{ConnectionState, DataType, VarIntWrapper};
use std::io::{Cursor, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::time::Duration;

#[test]
fn play_keep_alive_is_answered_with_the_same_identifier() {
    let (mut server, mut peer_stream) = test_server_pair();
    let client = MinecraftClient::new();
    server.state = ConnectionState::Play;
    let keep_alive_id = 981_247_312_i64;

    assert!(client.dispatch_packet(
        ClientboundKeepAlivePacket::get_id(),
        &mut server,
        encoded_keep_alive_id(keep_alive_id),
    ));

    let (packet_id, payload) = read_packet_frame(&mut peer_stream);
    let packet = ServerboundKeepAlivePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(packet_id, ServerboundKeepAlivePacket::get_id());
    assert_eq!(packet.id, keep_alive_id);
}

fn test_server_pair() -> (Server, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let address = listener.local_addr().unwrap();
    let stream = TcpStream::connect(address).unwrap();
    let (peer_stream, _) = listener.accept().unwrap();
    peer_stream
        .set_read_timeout(Some(Duration::from_millis(100)))
        .unwrap();
    (Server::new(stream, address), peer_stream)
}

fn encoded_keep_alive_id(keep_alive_id: i64) -> Vec<u8> {
    let mut payload = Vec::new();
    keep_alive_id.encode(&mut payload).unwrap();
    payload
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
