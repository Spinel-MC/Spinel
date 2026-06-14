use super::super::login_play::LoginPlayPacket;
use crate::entity::game_mode::GameMode;
use spinel_network::types::Identifier;

#[test]
fn default_login_game_mode_is_survival() {
    let packet = LoginPlayPacket::new_default(1);

    assert_eq!(
        packet.common_player_spawn_info.game_mode,
        GameMode::Survival.id()
    );
}

#[test]
fn login_uses_the_registered_world_dimension_type() {
    let world_name = Identifier::minecraft("overworld");
    let packet = LoginPlayPacket::new(1, GameMode::Creative, 0, world_name.clone());

    assert_eq!(packet.levels.0, vec![world_name.clone()]);
    assert_eq!(packet.common_player_spawn_info.dimension_type, 0);
    assert_eq!(packet.common_player_spawn_info.dimension, world_name);
}
