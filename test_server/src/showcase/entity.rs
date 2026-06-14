use spinel::{
    nbt::{Tag, TagReadable},
    registry::{EntityType, ItemStack, Material},
    server::{
        MinecraftServer,
        entity::{
            CreatureEntity, Entity, EntityId, EntityPosition,
            ai::{EntityAiGroupBuilder, goal::RandomLookAroundGoal},
        },
        world::{BlockPosition, World},
    },
    utils::component::Component,
};
use std::io;
use uuid::Uuid;

pub struct EntityShowcase;

impl EntityShowcase {
    pub fn spawn(
        server: &mut MinecraftServer,
        world_id: Uuid,
        origin: EntityPosition,
    ) -> io::Result<ItemStack> {
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
        let mut zombie = Self::zombie(zombie_position);
        zombie.add_ai_group(
            EntityAiGroupBuilder::default()
                .add_goal_selector(RandomLookAroundGoal::new(20))
                .build(),
        );
        let zombie_id = zombie.entity_id();
        if !world.add_entity(Entity::Creature(zombie)) {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Zombie add cancelled.",
            ));
        }
        world.spawn_item_entity(ItemStack::of(Material::DIAMOND), item_position)?;
        Ok(Self::pathfinding_stick(zombie_id))
    }

    pub fn pathfind(
        world: &mut World,
        pathfinding_stick: &ItemStack,
        block_position: BlockPosition,
    ) -> bool {
        let Some(zombie_id) = pathfinding_stick.get_tag(&Self::zombie_id_tag()) else {
            return false;
        };
        let destination = EntityPosition::new(
            f64::from(block_position.x) + 0.5,
            f64::from(block_position.y) + 1.0,
            f64::from(block_position.z) + 0.5,
            0.0,
            0.0,
        );
        let world_snapshot = world.update_snapshot();
        let Some(zombie) = world.creature_by_id_mut(EntityId::from_raw(zombie_id)) else {
            return false;
        };
        zombie.set_path_to_default(&world_snapshot, Some(destination))
    }

    pub(super) fn zombie(position: EntityPosition) -> CreatureEntity {
        let mut zombie = CreatureEntity::new(EntityType::ZOMBIE);
        zombie.set_position(position);
        zombie
    }

    fn pathfinding_stick(zombie_id: EntityId) -> ItemStack {
        ItemStack::of(Material::STICK)
            .with_custom_name(Component::text("Zombie Pathfinder").build())
            .with_tag(&Self::zombie_id_tag(), Some(zombie_id.value()))
    }

    fn zombie_id_tag() -> Tag<i32> {
        Tag::<i32>::integer("spinel_showcase_zombie_id")
    }
}
