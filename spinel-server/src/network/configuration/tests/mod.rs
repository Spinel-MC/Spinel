use crate::network::client::instance::Client;
use crate::network::client::metadata::LoginMetadata;
use crate::network::configuration::known_packs::create_player;
use crate::server::MinecraftServer;
use spinel_network::types::game_profile::{GameProfile, GameProfileProperty};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use uuid::Uuid;

#[test]
fn create_player_applies_authenticated_profile_texture_property() {
    let (mut client, _peer_stream) = connected_client();
    let mut login_metadata = LoginMetadata::new(0);
    login_metadata.game_profile = Some(GameProfile {
        uuid: Uuid::from_u128(0x11111111_2222_3333_4444_555555555555),
        username: "Player".to_string(),
        properties: vec![GameProfileProperty {
            name: "textures".to_string(),
            value: "texture-data".to_string(),
            signature: Some("texture-signature".to_string()),
        }],
    });
    client.login_metadata = Some(login_metadata);
    let mut server = MinecraftServer::new();

    let player = create_player(&mut client, &mut server).unwrap();

    let player_info_packet = player.get_player_info_packet();
    let properties = &player_info_packet.entries.0[0].properties;
    assert_eq!(properties.len(), 1);
    assert_eq!(properties[0].name, "textures");
    assert_eq!(properties[0].value, "texture-data");
    assert_eq!(
        properties[0].signature.as_deref(),
        Some("texture-signature")
    );
}

fn connected_client() -> (Client, TcpStream) {
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
