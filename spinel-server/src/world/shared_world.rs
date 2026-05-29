use crate::world::World;
use uuid::Uuid;

pub struct SharedWorld {
    source_world: Uuid,
    world: World,
}

impl SharedWorld {
    pub fn new(source_world: Uuid, mut world: World) -> Self {
        world.set_source_world(source_world);
        Self {
            source_world,
            world,
        }
    }

    pub const fn source_world(&self) -> Uuid {
        self.source_world
    }

    pub const fn uuid(&self) -> Uuid {
        self.world.uuid()
    }

    pub const fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub const fn is_registered(&self) -> bool {
        self.world.is_registered()
    }

    pub(crate) fn set_registered(&mut self, registered: bool) {
        self.world.set_registered(registered);
    }
}
