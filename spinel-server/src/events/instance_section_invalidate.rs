use crate::world::World;
use spinel_macros::event_dispatcher;

#[event_dispatcher]
pub struct InstanceSectionInvalidateEvent {
    world: *mut World,
    section_x: i32,
    section_y: i32,
    section_z: i32,
}

impl InstanceSectionInvalidateEvent {
    pub fn new(world: *mut World, section_x: i32, section_y: i32, section_z: i32) -> Self {
        Self {
            world,
            section_x,
            section_y,
            section_z,
        }
    }

    pub fn world(&mut self) -> &mut World {
        unsafe { &mut *self.world }
    }

    pub const fn section_x(&self) -> i32 {
        self.section_x
    }

    pub const fn section_y(&self) -> i32 {
        self.section_y
    }

    pub const fn section_z(&self) -> i32 {
        self.section_z
    }
}
