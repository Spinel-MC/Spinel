use crate::network::client::instance::Client;
use crate::network::handshake::intention::on_intention;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::login::disconnect::LoginDisconnectPacket;
use spinel_core::network::serverbound::handshake::intention::IntentionPacket;
use spinel_network::{ConnectionState, DataType, PacketDecoder, VarIntWrapper};
use spinel_utils::constants::{MINECRAFT_VERSION, PROTOCOL_VERSION};
use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};

#[test]
fn mismatched_login_protocol_uses_login_disconnect_during_handshake() {
    let (mut client, mut peer_stream) = connected_handshaking_client();
    let mut server = MinecraftServer::new();
    let intention_packet = mismatched_login_intention_packet();

    assert!(on_intention(&mut client, intention_packet, &mut server));

    let disconnect_packet = first_login_disconnect_packet(&mut peer_stream);
    assert_eq!(client.state, ConnectionState::Login);
    assert_eq!(
        disconnect_packet.reason.to_plain_string(),
        format!("Outdated client! Please use {MINECRAFT_VERSION}")
    );
}

fn connected_handshaking_client() -> (Client, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let server_address = listener.local_addr().unwrap();
    let client_stream = TcpStream::connect(server_address).unwrap();
    let (peer_stream, client_address) = listener.accept().unwrap();
    let client = Client::new(
        client_stream,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), client_address.port()),
    );
    (client, peer_stream)
}

fn mismatched_login_intention_packet() -> IntentionPacket {
    IntentionPacket {
        protocol_version: PROTOCOL_VERSION as i32 - 1,
        server_address: "localhost".to_owned(),
        server_port: 25565,
        intention: 2,
    }
}

fn first_login_disconnect_packet(peer_stream: &mut TcpStream) -> LoginDisconnectPacket {
    let mut packet_decoder = PacketDecoder::new();
    let packet_frame = packet_decoder.read_frame(peer_stream).unwrap();
    let mut packet_cursor = Cursor::new(packet_frame);
    let packet_id = VarIntWrapper::decode(&mut packet_cursor).unwrap().0;
    assert_eq!(packet_id, LoginDisconnectPacket::get_id());
    LoginDisconnectPacket::decode(&mut packet_cursor).unwrap()
}
