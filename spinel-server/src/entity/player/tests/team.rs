use crate::entity::Player;
use crate::scoreboard::Team;
use spinel_core::network::clientbound::play::set_player_team::TeamAction;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

#[test]
fn player_scoreboard_team_uses_username_membership_and_removes_previous_team() {
    let mut player = Player::new(
        Uuid::new_v4(),
        "ScoreName".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let mut first_team = Team::new("red");
    let mut second_team = Team::new("blue");

    let first_packets = player.set_scoreboard_team(None, Some(&mut first_team));
    let second_packets = player.set_scoreboard_team(Some(&mut first_team), Some(&mut second_team));

    assert_eq!(player.team(), Some("blue"));
    assert!(first_team.members().next().is_none());
    assert!(second_team.members().eq(["ScoreName"].into_iter()));
    assert!(matches!(
        first_packets.as_slice(),
        [packet] if matches!(&packet.action, TeamAction::AddEntities(members) if members == &vec!["ScoreName".to_owned()])
    ));
    assert!(matches!(
        second_packets.as_slice(),
        [remove_packet, add_packet]
            if matches!(&remove_packet.action, TeamAction::RemoveEntities(members) if members == &vec!["ScoreName".to_owned()])
                && matches!(&add_packet.action, TeamAction::AddEntities(members) if members == &vec!["ScoreName".to_owned()])
    ));
}
