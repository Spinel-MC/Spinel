use spinel_macros::packet;

#[packet(id: "client_command", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct ClientCommandPacket {
    pub action: i32,
}

impl ClientCommandPacket {
    pub const PERFORM_RESPAWN: i32 = 0;
    pub const REQUEST_STATS: i32 = 1;
}
