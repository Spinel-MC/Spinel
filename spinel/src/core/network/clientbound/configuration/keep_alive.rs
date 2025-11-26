use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: "keep_alive", state: ConnectionState::Configuration)]
pub struct KeepAlivePacket {
    pub keep_alive_id: i64,
}
