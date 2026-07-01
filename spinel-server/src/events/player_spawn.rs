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

    pub fn get_player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn get_world(&mut self) -> &mut World {
        unsafe { &mut *self.world }
    }

    pub fn is_first_spawn(&self) -> bool {
        self.first_spawn
    }

    pub fn player(&mut self) -> &mut Player {
        self.get_player()
    }

    pub fn world(&mut self) -> &mut World {
        self.get_world()
    }

    pub fn first_spawn(&self) -> bool {
        self.is_first_spawn()
    }
}
