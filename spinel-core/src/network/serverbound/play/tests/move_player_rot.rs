use super::super::move_player_rot::MovePlayerRotPacket;

#[test]
fn move_player_rotation_decodes_minestom_status_flags() {
    let packet = MovePlayerRotPacket {
        y_rot: 1.0,
        x_rot: 2.0,
        flags: 0,
    };

    assert!(!packet.on_ground());
    assert!(!packet.horizontal_collision());
}
