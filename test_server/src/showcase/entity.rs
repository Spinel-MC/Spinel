use spinel::{
    nbt::NbtCompound,
    registry::{EntityType, ItemStack, Material},
    server::{
        MinecraftServer,
        entity::{EntityId, EntityPosition},
        world::World,
    },
};
use std::io;
use std::sync::{Mutex, MutexGuard, OnceLock};
use uuid::Uuid;

pub struct EntityShowcase;

#[derive(Clone, Copy)]
struct EntityShowcaseZombie {
    world_id: Uuid,
    player_id: EntityId,
    zombie_id: EntityId,
    ticks: u64,
}

impl EntityShowcase {
    pub fn spawn(
        server: &mut MinecraftServer,
        world_id: Uuid,
        origin: EntityPosition,
        player_id: EntityId,
    ) -> io::Result<()> {
        let zombie_position = EntityPosition::new(
            origin.x() + 3.0,
            origin.y(),
            origin.z(),
            origin.yaw(),
            origin.pitch(),
        );
        let item_position = EntityPosition::new(
            origin.x() + 5.0,
            origin.y() + 1.0,
            origin.z() + 5.0,
            origin.yaw(),
            origin.pitch(),
        );
        let Some(world) = server.world_manager.world_mut(world_id) else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Player world was not found.",
            ));
        };
        world
            .spawn_entity(
                EntityType::ZOMBIE,
                zombie_position,
                Some(&NbtCompound::new()),
            )
            .map(|zombie_id| {
                let _ = Self::track_zombie(world_id, player_id, zombie_id);
            })?;
        world.spawn_item_entity(ItemStack::of(Material::DIAMOND), item_position)?;
        Ok(())
    }

    pub fn tick_world(world: &mut World) -> io::Result<()> {
        let world_id = world.uuid();
        let mut zombies = Self::zombies()?;
        zombies
            .iter_mut()
            .filter(|zombie| zombie.world_id == world_id)
            .try_for_each(|zombie| {
                let Some(player_position) = world
                    .entity_by_id(zombie.player_id)
                    .map(|entity| entity.position())
                else {
                    return Ok::<(), io::Error>(());
                };
                zombie.ticks += 1;
                let Some(zombie_position) = world
                    .entity_by_id(zombie.zombie_id)
                    .map(|entity| entity.position())
                else {
                    return Ok::<(), io::Error>(());
                };
                let next_position = zombie.next_position(zombie_position, player_position);
                world.move_generic_entity(zombie.zombie_id, next_position, true)?;
                world.look_generic_entity_at_position(
                    zombie.zombie_id,
                    EntityShowcaseZombie::player_eye_target_from_zombie_origin(player_position),
                    true,
                )?;
                if zombie.should_swing_main_hand() {
                    world.swing_generic_entity_main_hand(zombie.zombie_id)?;
                }
                Ok(())
            })?;
        zombies.retain(|zombie| {
            zombie.world_id != world_id
                || (world.entity_by_id(zombie.player_id).is_some()
                    && world.entity_by_id(zombie.zombie_id).is_some())
        });
        Ok(())
    }

    fn track_zombie(world_id: Uuid, player_id: EntityId, zombie_id: EntityId) -> io::Result<()> {
        let mut zombies = Self::zombies()?;
        zombies.retain(|zombie| zombie.zombie_id != zombie_id);
        zombies.push(EntityShowcaseZombie {
            world_id,
            player_id,
            zombie_id,
            ticks: 0,
        });
        Ok(())
    }

    fn zombies() -> io::Result<MutexGuard<'static, Vec<EntityShowcaseZombie>>> {
        active_zombies().lock().map_err(|_| {
            io::Error::new(
                io::ErrorKind::Other,
                "Entity showcase zombie state was poisoned.",
            )
        })
    }
}

impl EntityShowcaseZombie {
    fn next_position(
        &self,
        zombie_position: EntityPosition,
        player_position: EntityPosition,
    ) -> EntityPosition {
        let target_position = self.target_position_around_player(player_position);
        let delta_x = target_position.x() - zombie_position.x();
        let delta_z = target_position.z() - zombie_position.z();
        let horizontal_distance = (delta_x.mul_add(delta_x, delta_z * delta_z)).sqrt();
        if horizontal_distance <= f64::EPSILON {
            return EntityPosition::new(
                zombie_position.x(),
                player_position.y(),
                zombie_position.z(),
                zombie_position.yaw(),
                zombie_position.pitch(),
            );
        }
        let step_distance = horizontal_distance.min(0.32);
        let x = zombie_position.x() + delta_x / horizontal_distance * step_distance;
        let z = zombie_position.z() + delta_z / horizontal_distance * step_distance;
        EntityPosition::new(
            x,
            player_position.y(),
            z,
            zombie_position.yaw(),
            zombie_position.pitch(),
        )
    }

    fn target_position_around_player(&self, player_position: EntityPosition) -> EntityPosition {
        let ticks = self.ticks as f64;
        let angle = ticks * 0.12;
        let radius = 3.0 + (ticks * 0.18).sin() * 0.35;
        EntityPosition::new(
            player_position.x() + angle.cos() * radius,
            player_position.y(),
            player_position.z() + angle.sin() * radius,
            player_position.yaw(),
            player_position.pitch(),
        )
    }

    fn player_eye_target_from_zombie_origin(player_position: EntityPosition) -> EntityPosition {
        EntityPosition::new(
            player_position.x(),
            player_position.y() + EntityType::PLAYER.eye_height() - EntityType::ZOMBIE.eye_height(),
            player_position.z(),
            player_position.yaw(),
            player_position.pitch(),
        )
    }

    fn should_swing_main_hand(self) -> bool {
        self.ticks % 18 == 0
    }
}

fn active_zombies() -> &'static Mutex<Vec<EntityShowcaseZombie>> {
    static ACTIVE_ZOMBIES: OnceLock<Mutex<Vec<EntityShowcaseZombie>>> = OnceLock::new();

    ACTIVE_ZOMBIES.get_or_init(|| Mutex::new(Vec::new()))
}
