use spinel_macros::packet;

#[packet(id: "set_entity_link", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct AttachEntityPacket {
    pub attached_entity_id: i32,
    pub holding_entity_id: i32,
}
