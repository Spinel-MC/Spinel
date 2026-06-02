use spinel_macros::packet;

#[packet(id: "set_titles_animation", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SetTitlesAnimationPacket {
    pub fade_in: i32,
    pub stay: i32,
    pub fade_out: i32,
}

impl SetTitlesAnimationPacket {
    pub const fn new(fade_in: i32, stay: i32, fade_out: i32) -> Self {
        Self {
            fade_in,
            stay,
            fade_out,
        }
    }
}
