use crate::entity::Entity;
use crate::entity::player::Player;
use crate::entity::player::PlayerChunk;
use crate::network::client::instance::Client;
use crate::scoreboard::Team;
use crate::world::World;
use spinel_core::network::clientbound::play::set_player_team::{SetPlayerTeamPacket, TeamAction};
use spinel_network::types::Identifier;
use spinel_network::{DataType, VarIntWrapper};
use spinel_registry::EntityType;
use std::io::{Cursor, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::time::Duration;
use uuid::Uuid;

#[test]
fn world_scoreboard_team_assignment_updates_registry_members_without_duplicates() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let entity = Entity::new(EntityType::ZOMBIE);
    let entity_id = entity.entity_id();
    let member = entity.uuid().to_string();

    world.add_entity(entity);
    assert!(world.register_scoreboard_team(Team::new("red")).unwrap());
    assert!(world.register_scoreboard_team(Team::new("blue")).unwrap());

    assert!(
        world
            .set_entity_scoreboard_team(entity_id, Some("red"))
            .unwrap()
    );
    assert!(
        !world
            .set_entity_scoreboard_team(entity_id, Some("red"))
            .unwrap()
    );
    assert!(
        world
            .set_entity_scoreboard_team(entity_id, Some("blue"))
            .unwrap()
    );

    let red_members = world
        .scoreboard_team("red")
        .unwrap()
        .members()
        .collect::<Vec<_>>();
    let blue_members = world
        .scoreboard_team("blue")
        .unwrap()
        .members()
        .collect::<Vec<_>>();

    assert!(red_members.is_empty());
    assert_eq!(blue_members, vec![member.as_str()]);
}

#[test]
fn world_scoreboard_team_assignment_refreshes_current_viewers_in_minestom_order() {
    let (mut viewer_client, mut viewer_stream) = test_client_pair();
    let mut world = World::new(Identifier::minecraft("overworld"));
    let target = Entity::new(EntityType::ZOMBIE);
    let target_id = target.entity_id();
    let target_member = target.uuid().to_string();
    let viewer = entered_viewer(&mut viewer_client);

    world.add_entity(target);
    world.add_entity(Entity::Player(viewer));
    world.register_scoreboard_team(Team::new("red")).unwrap();
    world.register_scoreboard_team(Team::new("blue")).unwrap();
    drain_available_packet_frames(&mut viewer_stream);

    world
        .set_entity_scoreboard_team(target_id, Some("red"))
        .unwrap();
    let red_packet = read_team_packet(&mut viewer_stream);
    world
        .set_entity_scoreboard_team(target_id, Some("blue"))
        .unwrap();
    let remove_red_packet = read_team_packet(&mut viewer_stream);
    let add_blue_packet = read_team_packet(&mut viewer_stream);

    assert!(matches!(
        red_packet.action,
        TeamAction::AddEntities(members) if members == vec![target_member.clone()]
    ));
    assert!(matches!(
        remove_red_packet.action,
        TeamAction::RemoveEntities(members) if remove_red_packet.team_name == "red" && members == vec![target_member.clone()]
    ));
    assert!(matches!(
        add_blue_packet.action,
        TeamAction::AddEntities(members) if add_blue_packet.team_name == "blue" && members == vec![target_member]
    ));
}

#[test]
fn world_scoreboard_team_assignment_does_not_send_to_non_viewers() {
    let (mut far_client, mut far_stream) = test_client_pair();
    let mut world = World::new(Identifier::minecraft("overworld"));
    let target = Entity::new(EntityType::ZOMBIE);
    let target_id = target.entity_id();
    let mut far_viewer = entered_player(&mut far_client);

    far_viewer.set_position(crate::entity::EntityPosition::new(
        256.0, 64.0, 256.0, 0.0, 0.0,
    ));
    world.add_entity(target);
    world.add_entity(Entity::Player(far_viewer));
    world.register_scoreboard_team(Team::new("red")).unwrap();
    drain_available_packet_frames(&mut far_stream);

    assert!(
        world
            .set_entity_scoreboard_team(target_id, Some("red"))
            .unwrap()
    );
    assert!(drain_available_packet_frames(&mut far_stream).is_empty());
}

fn entered_viewer(client: &mut Client) -> Player {
    let mut player = entered_player(client);
    player.mark_chunk_sent_to_client(PlayerChunk::new(0, 0));
    player
}

fn entered_player(client: &mut Client) -> Player {
    let mut player = Player::new(
        Uuid::new_v4(),
        "Viewer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), client.addr.port()),
    );
    player.set_client(client);
    player.mark_entered_world();
    player
}

fn read_team_packet(peer_stream: &mut TcpStream) -> SetPlayerTeamPacket {
    let (packet_id, payload) = read_packet_frame(peer_stream);
    assert_eq!(packet_id, SetPlayerTeamPacket::get_id());
    SetPlayerTeamPacket::decode(&mut payload.as_slice()).unwrap()
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

fn drain_available_packet_frames(peer_stream: &mut TcpStream) -> Vec<(i32, Vec<u8>)> {
    peer_stream
        .set_read_timeout(Some(Duration::from_millis(25)))
        .unwrap();
    let mut packet_frames = Vec::new();
    loop {
        let frame_length = match VarIntWrapper::decode(peer_stream) {
            Ok(frame_length) => frame_length.0 as usize,
            Err(error)
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut =>
            {
                break;
            }
            Err(error) => panic!("packet frame length decode failed: {error}"),
        };
        let mut frame = vec![0; frame_length];
        peer_stream.read_exact(&mut frame).unwrap();
        let mut frame_cursor = Cursor::new(frame);
        let packet_id = VarIntWrapper::decode(&mut frame_cursor).unwrap().0;
        let payload_start = frame_cursor.position() as usize;
        let payload = frame_cursor.into_inner()[payload_start..].to_vec();
        packet_frames.push((packet_id, payload));
    }
    peer_stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    packet_frames
}

fn test_client_pair() -> (Client, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = TcpStream::connect(addr).unwrap();
    let (peer_stream, _) = listener.accept().unwrap();
    peer_stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    (Client::new(stream, addr), peer_stream)
}
