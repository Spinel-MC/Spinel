use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "move_entity_pos", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct EntityPositionPacket {
    pub entity_id: VarInt,
    pub delta_x: i16,
    pub delta_y: i16,
    pub delta_z: i16,
    pub on_ground: bool,
}

impl EntityPositionPacket {
    pub fn delta(new_coordinate: f64, old_coordinate: f64) -> i16 {
        ((new_coordinate * 32.0 - old_coordinate * 32.0) * 128.0) as i16
    }
}
