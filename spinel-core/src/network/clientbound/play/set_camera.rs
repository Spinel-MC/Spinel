use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "set_camera", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SetCameraPacket {
    pub camera_id: VarInt,
}

impl SetCameraPacket {
    pub const fn new(camera_id: VarInt) -> Self {
        Self { camera_id }
    }
}
