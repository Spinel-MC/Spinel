use spinel::{
    network::types::sound::SoundEvent,
    registry::BuiltinSoundEvent,
    server::{
        MinecraftServer,
        entity::EntityPosition,
        world::{Block, BlockPosition, Weather},
    },
};
use std::io;
use uuid::Uuid;

pub struct WorldShowcase;

impl WorldShowcase {
    pub fn apply(
        server: &mut MinecraftServer,
        world_id: Uuid,
        origin: EntityPosition,
    ) -> io::Result<()> {
        let Some(world) = server.world_manager.world_mut(world_id) else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Player world was not found.",
            ));
        };
        let base_x = origin.get_x() as i32;
        let base_y = origin.get_y() as i32;
        let base_z = origin.get_z() as i32;
        world.set_time(6_000)?;
        let weather = Weather::new(0.5, 0.0)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
        world.set_weather(weather)?;
        world.set_block(
            BlockPosition::new(base_x, base_y - 1, base_z + 3),
            Block::EMERALD_BLOCK,
        )?;
        world.set_block(
            BlockPosition::new(base_x + 1, base_y - 1, base_z + 3),
            Block::DIAMOND_BLOCK,
        )?;
        world.set_block(
            BlockPosition::new(base_x + 2, base_y - 1, base_z + 3),
            Block::GOLD_BLOCK,
        )?;
        world.play_sound_except(
            None,
            SoundEvent::Id(BuiltinSoundEvent::BLOCK_NOTE_BLOCK_PLING.id()),
            0,
            origin,
            1.0,
            1.0,
            0,
        )?;
        Ok(())
    }
}
