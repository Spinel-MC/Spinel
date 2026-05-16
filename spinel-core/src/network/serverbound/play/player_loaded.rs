use spinel_macros::packet;

#[packet(id: "player_loaded", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct PlayerLoadedPacket;
