use crate::events::login::PreLoginEvent;
use crate::network::client::instance::Client;
use crate::network::client::metadata::LoginMetadata;
use crate::network::login::login_start::on_login_start;
use crate::server::{Auth, MinecraftServer, OnlineAuth};
use spinel_core::network::clientbound::login::disconnect::LoginDisconnectPacket;
use spinel_core::network::clientbound::login::encryption_request::EncryptionRequestPacket;
use spinel_core::network::clientbound::login::set_compression::SetCompressionPacket;
use spinel_core::network::serverbound::login::login_start::LoginStartPacket;
use spinel_macros::event_listener;
use spinel_network::types::game_profile::GameProfile;
use spinel_network::{ConnectionState, DataType, PacketDecoder, VarIntWrapper};
use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use uuid::Uuid;

struct RejectedAuthenticatedPreLoginTestListener;
struct GameProfileChangingPreLoginTestListener;

#[event_listener]
impl RejectedAuthenticatedPreLoginTestListener {
    #[event_handler]
    fn on_pre_login(event: &mut PreLoginEvent, _server: &mut MinecraftServer) {
        if event.username() != "RejectedPlayer" {
            return;
        }
        let _ = event
            .client()
            .kick(spinel_utils::component::Component::text(
                "Rejected after authentication",
            ));
        event.cancelled = true;
    }
}

#[event_listener]
impl GameProfileChangingPreLoginTestListener {
    #[event_handler]
    fn on_pre_login(event: &mut PreLoginEvent, _server: &mut MinecraftServer) {
        if event.username() != "Player" {
            return;
        }
        event.set_game_profile(GameProfile {
            uuid: Uuid::from_u128(0x99999999_8888_7777_6666_555555555555),
            username: "ChangedPlayer".to_owned(),
            properties: Vec::new(),
        });
    }
}
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
fn offline_login_rejection_sends_login_disconnect_after_authenticated_pre_login_event() {
    let (mut client, mut peer_stream) = connected_login_client();
    let mut server = MinecraftServer::init(Auth::Offline);
    server.register_event_handler(RejectedAuthenticatedPreLoginTestListener);
    let login_start_packet = LoginStartPacket {
        name: "RejectedPlayer".to_owned(),
        uuid: Uuid::from_u128(0xaaaaaaaa_bbbb_cccc_dddd_eeeeeeeeeeee),
    };

    assert!(on_login_start(&mut client, login_start_packet, &mut server));

    let packet_frame = first_clientbound_packet_frame(&mut peer_stream);
    let mut packet_cursor = Cursor::new(packet_frame);
    let packet_id = VarIntWrapper::decode(&mut packet_cursor).unwrap().0;
    let disconnect_packet = LoginDisconnectPacket::decode(&mut packet_cursor).unwrap();
    assert_eq!(packet_id, LoginDisconnectPacket::get_id());
    assert_eq!(
        disconnect_packet.reason.to_plain_string(),
        "Rejected after authentication"
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

#[test]
fn offline_login_start_stores_pre_login_changed_game_profile() {
    let (mut client, _peer_stream) = connected_login_client();
    let mut server = MinecraftServer::init(Auth::Offline);
    server.register_event_handler(GameProfileChangingPreLoginTestListener);
    let login_start_packet = login_start_packet();

    assert!(on_login_start(&mut client, login_start_packet, &mut server));

    let game_profile = client
        .login_metadata
        .as_ref()
        .and_then(|login_metadata| login_metadata.game_profile.as_ref())
        .expect("offline login should store a game profile");
    assert_eq!(
        game_profile.uuid,
        Uuid::from_u128(0x99999999_8888_7777_6666_555555555555)
    );
    assert_eq!(game_profile.username, "ChangedPlayer");
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
    let packet_frame = first_clientbound_packet_frame(peer_stream);
    let mut packet_cursor = Cursor::new(packet_frame);
    VarIntWrapper::decode(&mut packet_cursor).unwrap().0
}

fn first_clientbound_packet_frame(peer_stream: &mut TcpStream) -> Vec<u8> {
    PacketDecoder::new().read_frame(peer_stream).unwrap()
}

fn online_auth() -> Auth {
    Auth::Online(OnlineAuth::new().expect("online authentication key pair should be generated"))
}
