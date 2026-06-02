use spinel_macros::packet;

#[packet(id: "rename_item", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct RenameItemPacket {
    pub item_name: String,
}
