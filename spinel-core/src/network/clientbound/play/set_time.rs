use spinel_macros::packet;

#[packet(id: "set_time", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SetTimePacket {
    pub world_age: i64,
    pub time: i64,
    pub tick_day_time: bool,
}

impl SetTimePacket {
    pub const fn new(world_age: i64, time: i64, tick_day_time: bool) -> Self {
        Self {
            world_age,
            time,
            tick_day_time,
        }
    }
}
