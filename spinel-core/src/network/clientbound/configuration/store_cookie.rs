use spinel_macros::packet;

#[packet(
    id: "store_cookie",
    state: ConnectionState::Configuration,
    recipient: Recipient::Client,
    generate_fields: true
)]
pub struct StoreCookiePacket {}
