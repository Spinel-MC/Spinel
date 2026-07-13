use crate::entity::Player;
use crate::network::client::instance::Client;
use crate::network::client::metadata::LoginMetadata;
use crate::network::connection_manager::ConnectionManager;
use spinel_network::types::game_profile::GameProfile;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use uuid::Uuid;

#[test]
fn connection_manager_default_player_provider_uses_game_profile_and_connection() {
    let connection_manager = ConnectionManager::new();
    let mut client = test_client();
    client.login_metadata = Some(LoginMetadata::new(765));
    let game_profile = game_profile(Uuid::from_u128(1), "DefaultPlayer");

    let player = connection_manager.create_player(&mut client, &game_profile);

    assert_eq!(player.get_uuid(), game_profile.uuid);
    assert_eq!(player.get_username(), game_profile.username);
    assert_eq!(player.get_protocol_version(), 765);
    assert_eq!(player.get_address(), client.addr);
}

#[test]
fn connection_manager_custom_player_provider_controls_created_player() {
    let mut connection_manager = ConnectionManager::new();
    let custom_uuid = Uuid::from_u128(2);
    connection_manager.set_player_provider(
        move |connection: &mut Client, game_profile: &GameProfile| {
            Player::new(
                custom_uuid,
                format!("custom-{}", game_profile.username),
                999,
                connection.addr,
            )
        },
    );
    let mut client = test_client();
    let game_profile = game_profile(Uuid::from_u128(1), "Player");

    let player = connection_manager.create_player(&mut client, &game_profile);

    assert_eq!(player.get_uuid(), custom_uuid);
    assert_eq!(player.get_username(), "custom-Player");
    assert_eq!(player.get_protocol_version(), 999);
    assert_eq!(player.get_address(), client.addr);
}

#[test]
fn connection_manager_none_player_provider_restores_default_provider() {
    let mut connection_manager = ConnectionManager::new();
    connection_manager.set_player_provider(
        |connection: &mut Client, game_profile: &GameProfile| {
            Player::new(
                Uuid::from_u128(99),
                game_profile.username.clone(),
                999,
                connection.addr,
            )
        },
    );
    connection_manager.set_player_provider(None::<fn(&mut Client, &GameProfile) -> Player>);
    let mut client = test_client();
    client.login_metadata = Some(LoginMetadata::new(765));
    let game_profile = game_profile(Uuid::from_u128(1), "DefaultAgain");

    let player = connection_manager.create_player(&mut client, &game_profile);

    assert_eq!(player.get_uuid(), game_profile.uuid);
    assert_eq!(player.get_username(), game_profile.username);
    assert_eq!(player.get_protocol_version(), 765);
}

fn test_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let server_address = listener.local_addr().unwrap();
    let client_stream = TcpStream::connect(server_address).unwrap();
    let (server_stream, client_address) = listener.accept().unwrap();
    drop(client_stream);
    Client::new(
        server_stream,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), client_address.port()),
    )
}

fn game_profile(uuid: Uuid, username: &str) -> GameProfile {
    GameProfile {
        uuid,
        username: username.to_owned(),
        properties: Vec::new(),
    }
}
