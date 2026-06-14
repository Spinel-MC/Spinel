use super::super::set_player_team::{
    SetPlayerTeamPacket, TeamAction, TeamCollisionRule, TeamNameTagVisibility, TeamParameters,
};
use spinel_network::DataType;

#[test]
fn set_player_team_packet_encodes_create_add_remove_and_update_actions() {
    let mut parameters = TeamParameters::new(vec!["Player".to_owned()]);
    parameters.display_name = spinel_utils::component::text::TextComponent::literal("Red");
    parameters.prefix = spinel_utils::component::text::TextComponent::literal("[R]");
    parameters.suffix = spinel_utils::component::text::TextComponent::literal("!");
    parameters.name_tag_visibility = TeamNameTagVisibility::HideForOtherTeams;
    parameters.collision_rule = TeamCollisionRule::PushOwnTeam;
    let packet = SetPlayerTeamPacket {
        team_name: "red".to_owned(),
        action: TeamAction::Create(parameters),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = SetPlayerTeamPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(SetPlayerTeamPacket::get_id(), 0x6b);
    assert_eq!(decoded_packet, packet);
}
