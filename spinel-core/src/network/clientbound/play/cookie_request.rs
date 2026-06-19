use spinel_macros::packet;
use spinel_network::types::Identifier;

#[packet(id: "cookie_request", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct CookieRequestPacket {
    pub key: Identifier,
}
