use crate::network::client::instance::Client;
use crate::network::client::metadata::LoginMetadata;
use crate::network::login::login_start::on_login_start;
use crate::server::{Auth, MinecraftServer, OnlineAuth};
use spinel_core::network::clientbound::login::encryption_request::EncryptionRequestPacket;
use spinel_core::network::clientbound::login::set_compression::SetCompressionPacket;
use spinel_core::network::serverbound::login::login_start::LoginStartPacket;
use spinel_network::{ConnectionState, DataType, PacketDecoder, VarIntWrapper};
use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use uuid::Uuid;

#[test]
fn login_start_sends_encryption_request_when_server_auth_is_online() {
    let (mut client, mut peer_stream) = connected_login_client();
    let mut server = MinecraftServer::init(online_auth());
    let login_start_packet = login_start_packet();

    assert!(on_login_start(&mut client, login_start_packet, &mut server));

    assert_eq!(
        first_clientbound_packet_id(&mut peer_stream),
        EncryptionRequestPacket::get_id()
    );
}

#[test]
fn login_start_enters_configuration_without_encryption_when_server_auth_is_offline() {
    let (mut client, mut peer_stream) = connected_login_client();
    let mut server = MinecraftServer::init(Auth::Offline);
    let login_start_packet = login_start_packet();

    assert!(on_login_start(&mut client, login_start_packet, &mut server));

    assert_eq!(
        first_clientbound_packet_id(&mut peer_stream),
        SetCompressionPacket::get_id()
    );
}

fn connected_login_client() -> (Client, TcpStream) {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    let server_address = listener.local_addr().unwrap();
    let client_stream = TcpStream::connect(server_address).unwrap();
    let (peer_stream, client_address) = listener.accept().unwrap();
    let mut client = Client::new(
        client_stream,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), client_address.port()),
    );
    client.state = ConnectionState::Login;
    client.login_metadata = Some(LoginMetadata::new(0));
    (client, peer_stream)
}

fn login_start_packet() -> LoginStartPacket {
    LoginStartPacket {
        name: "Player".to_owned(),
        uuid: Uuid::from_u128(0x11111111_2222_3333_4444_555555555555),
    }
}

fn first_clientbound_packet_id(peer_stream: &mut TcpStream) -> i32 {
    let mut packet_decoder = PacketDecoder::new();
    let packet_frame = packet_decoder.read_frame(peer_stream).unwrap();
    let mut packet_cursor = Cursor::new(packet_frame);
    VarIntWrapper::decode(&mut packet_cursor).unwrap().0
}

fn online_auth() -> Auth {
    Auth::Online(OnlineAuth::new().expect("online authentication key pair should be generated"))
}
