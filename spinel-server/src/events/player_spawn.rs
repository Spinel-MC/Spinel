use crate::entity::Player;
use crate::world::World;
use spinel_macros::event_dispatcher;

#[event_dispatcher(with_client: true)]
pub struct PlayerSpawnEvent {
    player: *mut Player,
    world: *mut World,
    first_spawn: bool,
}

impl PlayerSpawnEvent {
    pub fn new(player: *mut Player, world: *mut World, first_spawn: bool) -> Self {
        Self {
            player,
            world,
            first_spawn,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn world(&mut self) -> &mut World {
        unsafe { &mut *self.world }
    }

    pub fn first_spawn(&self) -> bool {
        self.first_spawn
    }
}
