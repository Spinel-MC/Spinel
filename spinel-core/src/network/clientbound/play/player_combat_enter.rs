use spinel_macros::packet;

#[packet(
    id: "player_combat_enter",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
pub struct PlayerCombatEnterPacket;
