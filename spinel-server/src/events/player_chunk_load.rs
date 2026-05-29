use crate::entity::Player;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerChunkLoadEvent {
    player: *mut Player,
    chunk_x: i32,
    chunk_z: i32,
}

impl PlayerChunkLoadEvent {
    pub fn new(player: *mut Player, chunk_x: i32, chunk_z: i32) -> Self {
        Self {
            player,
            chunk_x,
            chunk_z,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub const fn chunk_x(&self) -> i32 {
        self.chunk_x
    }

    pub const fn chunk_z(&self) -> i32 {
        self.chunk_z
    }
}
