use spinel_macros::packet;
use spinel_network::VarInt;
use spinel_network::wrappers::NbtTextComponent;

#[packet(id: "open_screen", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct OpenScreenPacket {
    pub container_id: VarInt,
    pub container_type: VarInt,
    pub title: NbtTextComponent,
}
