use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: "store_cookie", state: ConnectionState::Configuration)]
pub struct StoreCookiePacket {
    pub key: String,
    pub payload: Vec<u8>,
}
