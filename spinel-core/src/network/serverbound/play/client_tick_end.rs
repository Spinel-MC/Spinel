use spinel_macros::packet;

#[packet(id: "client_tick_end", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct ClientTickEndPacket;
