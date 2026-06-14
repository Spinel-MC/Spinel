use crate::entity::GenericEntity;
use crate::scoreboard::Team;
use spinel_core::network::clientbound::play::set_player_team::TeamAction;
use spinel_registry::EntityType;

#[test]
fn generic_entity_scoreboard_team_uses_uuid_membership_and_removes_previous_team() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let member = entity.uuid().to_string();
    let mut first_team = Team::new("red");
    let mut second_team = Team::new("blue");

    let first_packets = entity.set_scoreboard_team(None, Some(&mut first_team));
    let second_packets = entity.set_scoreboard_team(Some(&mut first_team), Some(&mut second_team));

    assert_eq!(entity.team(), Some("blue"));
    assert!(first_team.members().next().is_none());
    assert!(second_team.members().eq([member.as_str()].into_iter()));
    assert!(matches!(
        first_packets.as_slice(),
        [packet] if matches!(&packet.action, TeamAction::AddEntities(members) if members == &vec![member.clone()])
    ));
    assert!(matches!(
        second_packets.as_slice(),
        [remove_packet, add_packet]
            if matches!(&remove_packet.action, TeamAction::RemoveEntities(members) if members == &vec![member.clone()])
                && matches!(&add_packet.action, TeamAction::AddEntities(members) if members == &vec![member.clone()])
    ));
}
