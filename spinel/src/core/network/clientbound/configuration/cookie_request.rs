use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: "cookie_request", state: ConnectionState::Configuration)]
pub struct CookieRequestPacket {
    pub key: String,
}
