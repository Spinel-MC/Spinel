use crate::showcase::EntityShowcase;
use spinel::{
    macros::event_listener,
    server::{MinecraftServer, events::instance_tick_end::InstanceTickEndEvent},
};

#[event_listener]
pub fn on_instance_tick_end(event: &mut InstanceTickEndEvent, _server: &mut MinecraftServer) {
    let _ = EntityShowcase::tick_world(event.world());
}
