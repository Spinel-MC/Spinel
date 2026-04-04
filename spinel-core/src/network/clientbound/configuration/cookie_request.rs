use spinel_macros::packet;

#[packet(id: "cookie_request", state: ConnectionState::Configuration, recipient: Recipient::Client)]
pub struct CookieRequestPacket {
    pub key: String,
}
