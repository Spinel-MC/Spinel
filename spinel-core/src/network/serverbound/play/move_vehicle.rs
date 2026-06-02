use spinel_macros::packet;

#[packet(id: "move_vehicle", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct ServerboundVehicleMovePacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}
