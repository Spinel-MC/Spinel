use spinel_macros::packet;

#[packet(id: "pick_item_from_entity", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct PickItemFromEntityPacket {
    pub entity_id: i32,
    pub include_data: bool,
}
