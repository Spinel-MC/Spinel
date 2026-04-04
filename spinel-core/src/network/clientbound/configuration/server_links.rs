use spinel_macros::packet;

#[packet(id: "server_links", state: ConnectionState::Configuration, recipient: Recipient::Client)]
pub struct ServerLinksPacket {
    pub url: String,
}
