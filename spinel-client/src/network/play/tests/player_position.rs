use crate::instance::MinecraftClient;
use crate::network::server::instance::Server;
use spinel_core::network::clientbound::play::sync_player_pos::SyncPlayerPositionPacket;
use spinel_core::network::serverbound::play::accept_teleportation::AcceptTeleportationPacket;
use spinel_network::types::TeleportFlags;
use spinel_network::{ConnectionState, DataType, VarIntWrapper};
use std::io::{Cursor, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::time::Duration;

#[test]
fn player_position_sync_is_acknowledged_with_the_same_teleport_id() {
    let (mut server, mut peer_stream) = test_server_pair();
    let client = MinecraftClient::new();
    server.state = ConnectionState::Play;
    let teleport_id = 42;
    let packet = SyncPlayerPositionPacket {
        teleport_id,
        x: 12.0,
        y: 80.0,
        z: -4.0,
        velocity_x: 0.0,
        velocity_y: 0.0,
        velocity_z: 0.0,
        yaw: 90.0,
        pitch: 0.0,
        flags: TeleportFlags::absolute(),
    };
    let mut payload = Vec::new();
    packet.encode(&mut payload).unwrap();

    assert!(client.dispatch_packet(SyncPlayerPositionPacket::get_id(), &mut server, payload));

    let (packet_id, payload) = read_packet_frame(&mut peer_stream);
    let packet = AcceptTeleportationPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(packet_id, AcceptTeleportationPacket::get_id());
    assert_eq!(packet.id, teleport_id);
    assert_eq!(client.position().x(), 12.0);
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
