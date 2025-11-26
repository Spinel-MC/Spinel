use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: "server_links", state: ConnectionState::Configuration)]
pub struct ServerLinksPacket {
    pub url: String,
}
