use spinel_macros::packet;

#[packet(
    id: "login_acknowledged",
    state: ConnectionState::Login,
    recipient: Recipient::Server,
    generate_fields: true,
    generate_constructor: true,
)]
pub struct LoginAcknowledgedPacket;
