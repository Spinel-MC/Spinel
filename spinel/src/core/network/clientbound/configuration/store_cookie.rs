use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: 0x0A)]
pub struct StoreCookiePacket {
    pub key: String,
    pub payload: Vec<u8>,
}