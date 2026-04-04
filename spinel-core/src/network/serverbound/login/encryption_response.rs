use spinel_macros::packet;

#[packet(
    id: "key",
    state: ConnectionState::Login,
    recipient: Recipient::Server,
    generate_fields: true
)]
pub struct EncryptionResponsePacket;
