use spinel_macros::packet;

#[packet(id: "entity_event", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct EntityStatusPacket {
    pub entity_id: i32,
    pub status: i8,
}
