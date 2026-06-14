use spinel_macros::packet;
use uuid::Uuid;

#[packet(
    id: "teleport_to_entity",
    state: ConnectionState::Play,
    recipient: Recipient::Server
)]
pub struct TeleportToEntityPacket {
    pub target: Uuid,
}
