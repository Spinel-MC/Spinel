use super::super::move_player_status_only::MovePlayerStatusOnlyPacket;

#[test]
fn move_player_status_only_decodes_minestom_status_flags() {
    let packet = MovePlayerStatusOnlyPacket {
        flags: MovePlayerStatusOnlyPacket::FLAG_ON_GROUND
            | MovePlayerStatusOnlyPacket::FLAG_HORIZONTAL_COLLISION,
    };

    assert!(packet.on_ground());
    assert!(packet.horizontal_collision());
}
