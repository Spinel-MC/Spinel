use spinel_macros::packet;
use uuid::Uuid;

#[packet(
    id: "hello",
    state: ConnectionState::Login,
    recipient: Recipient::Server
)]
pub struct LoginStartPacket {
    pub name: String,
    pub uuid: Uuid,
}
