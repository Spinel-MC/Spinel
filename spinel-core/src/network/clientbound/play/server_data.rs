use spinel_macros::packet;
use spinel_utils::component::text::TextComponent;

#[packet(id: "server_data", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ServerDataPacket {
    pub motd: TextComponent,
    pub icon_bytes: Option<Vec<u8>>,
}
