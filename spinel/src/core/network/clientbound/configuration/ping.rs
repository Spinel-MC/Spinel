use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: "ping", state: ConnectionState::Configuration)]
pub struct PingPacket {
    pub id: i32,
}
