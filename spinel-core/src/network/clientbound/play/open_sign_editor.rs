use spinel_macros::packet;
use spinel_network::types::Position;

#[packet(id: "open_sign_editor", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct OpenSignEditorPacket {
    pub pos: Position,
    pub is_front_text: bool,
}
