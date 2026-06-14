use spinel_macros::packet;

#[packet(id: "edit_book", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct EditBookPacket {
    pub slot: i32,
    pub pages: Vec<String>,
    pub title: Option<String>,
}
