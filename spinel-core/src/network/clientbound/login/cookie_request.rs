use spinel_macros::packet;

#[packet(id: "cookie_request", generate_fields: true, state: ConnectionState::Login, recipient: Recipient::Client)]
pub struct CookieRequestPacket;
