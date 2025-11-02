use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: 0x04)]
pub struct KeepAlivePacket {
    pub keep_alive_id: i64,
}