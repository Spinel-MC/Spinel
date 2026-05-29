use spinel_macros::packet;
use spinel_network::types::{VarInt, VarLong};

#[packet(id: "initialize_border", state: ConnectionState::Play, recipient: Recipient::Client)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitializeWorldBorderPacket {
    pub center_x: f64,
    pub center_z: f64,
    pub old_diameter: f64,
    pub new_diameter: f64,
    pub lerp_time: VarLong,
    pub absolute_max_size: VarInt,
    pub warning_blocks: VarInt,
    pub warning_time: VarInt,
}

impl InitializeWorldBorderPacket {
    pub fn new(
        center_x: f64,
        center_z: f64,
        old_diameter: f64,
        new_diameter: f64,
        lerp_time: i64,
        absolute_max_size: i32,
        warning_blocks: i32,
        warning_time: i32,
    ) -> Self {
        Self {
            center_x,
            center_z,
            old_diameter,
            new_diameter,
            lerp_time,
            absolute_max_size,
            warning_blocks,
            warning_time,
        }
    }
}
