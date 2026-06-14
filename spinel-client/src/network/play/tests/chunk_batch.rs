use crate::instance::MinecraftClient;
use crate::network::server::instance::Server;
use spinel_core::network::clientbound::play::chunk_batch_finished::ChunkBatchFinishedPacket;
use spinel_core::network::clientbound::play::chunk_batch_start::ChunkBatchStartPacket;
use spinel_core::network::serverbound::play::chunk_batch_received::ChunkBatchReceivedPacket;
use spinel_network::{ConnectionState, DataType, VarIntWrapper};
use std::io::{Cursor, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::time::Duration;

#[test]
fn chunk_batch_finished_acknowledges_the_batch_like_the_vanilla_client() {
    let (mut server, mut peer_stream) = test_server_pair();
    let client = MinecraftClient::new();
    server.state = ConnectionState::Play;

    assert!(client.dispatch_packet(ChunkBatchStartPacket::get_id(), &mut server, Vec::new(),));
    assert!(client.dispatch_packet(
        ChunkBatchFinishedPacket::get_id(),
        &mut server,
        encoded_batch_size(9),
    ));

    let (packet_id, payload) = read_packet_frame(&mut peer_stream);
    let packet = ChunkBatchReceivedPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(packet_id, ChunkBatchReceivedPacket::get_id());
    assert!(packet.desired_chunks_per_tick.is_finite());
    assert!(packet.desired_chunks_per_tick > 0.0);
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

fn encoded_batch_size(batch_size: i32) -> Vec<u8> {
    let mut payload = Vec::new();
    VarIntWrapper(batch_size).encode(&mut payload).unwrap();
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
