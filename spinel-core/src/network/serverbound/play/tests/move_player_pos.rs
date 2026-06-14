use super::super::move_player_pos::MovePlayerPosPacket;

#[test]
fn move_player_position_decodes_minestom_status_flags() {
    let packet = MovePlayerPosPacket {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        flags: MovePlayerPosPacket::FLAG_ON_GROUND,
    };

    assert!(packet.on_ground());
    assert!(!packet.horizontal_collision());
}
