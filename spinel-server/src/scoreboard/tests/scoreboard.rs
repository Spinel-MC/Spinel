use crate::scoreboard::Team;
use spinel_core::network::clientbound::play::set_player_team::{
    SetPlayerTeamPacket, TeamAction, TeamCollisionRule, TeamNameTagVisibility,
};
use spinel_network::DataType;
use spinel_utils::component::text::TextComponent;

#[test]
fn team_owner_creates_updates_removes_and_deduplicates_member_packets() {
    let mut team = Team::new("red");
    team.set_display_name(TextComponent::literal("Red"));
    team.set_name_tag_visibility(TeamNameTagVisibility::Never);
    team.set_collision_rule(TeamCollisionRule::Never);
    team.set_prefix(TextComponent::literal("[R]"));
    let add_packet = team.add_member("Player").unwrap();
    let duplicate_packet = team.add_member("Player");
    let create_packet = team.create_packet();
    let update_packet = team.update_packet();
    let remove_member_packet = team.remove_member("Player").unwrap();
    let remove_packet = team.remove_packet();

    assert!(duplicate_packet.is_none());
    assert!(matches!(add_packet.action, TeamAction::AddEntities(_)));
    assert!(matches!(create_packet.action, TeamAction::Create(_)));
    assert!(matches!(update_packet.action, TeamAction::Update(_)));
    assert!(matches!(
        remove_member_packet.action,
        TeamAction::RemoveEntities(_)
    ));
    assert!(matches!(remove_packet.action, TeamAction::Remove));

    let mut payload = Vec::new();
    create_packet.encode(&mut payload).unwrap();
    let decoded_packet = SetPlayerTeamPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(SetPlayerTeamPacket::get_id(), 0x6b);
    assert!(matches!(decoded_packet.action, TeamAction::Create(_)));
}
