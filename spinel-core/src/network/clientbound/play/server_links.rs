use spinel_macros::packet;
use spinel_network::types::ServerLinkEntry;

#[packet(id: "server_links", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ServerLinksPacket {
    pub links: Vec<ServerLinkEntry>,
}
