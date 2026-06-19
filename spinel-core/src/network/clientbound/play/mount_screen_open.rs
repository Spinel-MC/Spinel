use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "mount_screen_open", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct MountScreenOpenPacket {
    pub container_id: VarInt,
    pub inventory_columns: VarInt,
    pub entity_id: i32,
}
