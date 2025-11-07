use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: 0x10)]
pub struct ServerLinksPacket {
    pub url: String,
}
