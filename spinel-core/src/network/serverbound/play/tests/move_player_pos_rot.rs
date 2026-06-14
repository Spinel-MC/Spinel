use super::super::move_player_pos_rot::MovePlayerPosRotPacket;

#[test]
fn move_player_position_and_rotation_decodes_minestom_status_flags() {
    let packet = MovePlayerPosRotPacket {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        y_rot: 4.0,
        x_rot: 5.0,
        flags: MovePlayerPosRotPacket::FLAG_HORIZONTAL_COLLISION,
    };

    assert!(!packet.on_ground());
    assert!(packet.horizontal_collision());
}
