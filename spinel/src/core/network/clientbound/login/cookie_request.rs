use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: 0x05)]
pub struct CookieRequestPacket {
    pub key: String,
}