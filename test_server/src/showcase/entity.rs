use spinel::{
    nbt::{Tag, TagReadable},
    registry::{EntityType, ItemStack, Material},
    server::{
        MinecraftServer,
        entity::{
            CreatureEntity, Entity, EntityId, EntityPosition,
            ai::{EntityAiGroupBuilder, goal::RandomLookAroundGoal},
            pathfinding::VanillaGroundNodeFollower,
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
        let minestom_zombie_position = EntityPosition::new(
            origin.x() + 3.0,
            origin.y(),
            origin.z(),
            origin.yaw(),
            origin.pitch(),
        );
        let vanilla_zombie_position = EntityPosition::new(
            origin.x() + 3.0,
            origin.y(),
            origin.z() + 2.0,
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
        let mut minestom_zombie = Self::zombie(minestom_zombie_position, "Minestom Physics");
        minestom_zombie.add_ai_group(
            EntityAiGroupBuilder::default()
                .add_goal_selector(RandomLookAroundGoal::new(20))
                .build(),
        );
        let minestom_zombie_id = minestom_zombie.entity_id();
        if !world.add_entity(Entity::Creature(minestom_zombie)) {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Minestom physics zombie add cancelled.",
            ));
        }
        let mut vanilla_zombie = Self::zombie(vanilla_zombie_position, "Vanilla Physics");
        vanilla_zombie
            .navigator_mut()
            .set_node_follower(VanillaGroundNodeFollower::default());
        vanilla_zombie.add_ai_group(
            EntityAiGroupBuilder::default()
                .add_goal_selector(RandomLookAroundGoal::new(20))
                .build(),
        );
        let vanilla_zombie_id = vanilla_zombie.entity_id();
        if !world.add_entity(Entity::Creature(vanilla_zombie)) {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Vanilla physics zombie add cancelled.",
            ));
        }
        world.spawn_item_entity(ItemStack::of(Material::DIAMOND), item_position)?;
        Ok(Self::pathfinding_stick(
            minestom_zombie_id,
            vanilla_zombie_id,
        ))
    }

    pub fn pathfind(
        world: &mut World,
        pathfinding_stick: &ItemStack,
        block_position: BlockPosition,
    ) -> bool {
        let Some(minestom_zombie_id) = pathfinding_stick.get_tag(&Self::minestom_zombie_id_tag())
        else {
            return false;
        };
        let Some(vanilla_zombie_id) = pathfinding_stick.get_tag(&Self::vanilla_zombie_id_tag())
        else {
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
        let minestom_path_was_accepted = world
            .creature_by_id_mut(EntityId::from_raw(minestom_zombie_id))
            .is_some_and(|zombie| zombie.set_path_to_default(&world_snapshot, Some(destination)));
        let vanilla_path_was_accepted = world
            .creature_by_id_mut(EntityId::from_raw(vanilla_zombie_id))
            .is_some_and(|zombie| zombie.set_path_to_default(&world_snapshot, Some(destination)));
        minestom_path_was_accepted || vanilla_path_was_accepted
    }

    pub(super) fn zombie(position: EntityPosition, physics_name: &str) -> CreatureEntity {
        let mut zombie = CreatureEntity::new(EntityType::ZOMBIE);
        zombie.set_position(position);
        zombie.set_custom_name(Some(Component::text(physics_name).build()));
        zombie.set_custom_name_visible(true);
        zombie
    }

    fn pathfinding_stick(minestom_zombie_id: EntityId, vanilla_zombie_id: EntityId) -> ItemStack {
        ItemStack::of(Material::STICK)
            .with_custom_name(Component::text("Dual Zombie Pathfinder").build())
            .with_tag(
                &Self::minestom_zombie_id_tag(),
                Some(minestom_zombie_id.value()),
            )
            .with_tag(
                &Self::vanilla_zombie_id_tag(),
                Some(vanilla_zombie_id.value()),
            )
    }

    fn minestom_zombie_id_tag() -> Tag<i32> {
        Tag::<i32>::integer("spinel_showcase_minestom_zombie_id")
    }

    fn vanilla_zombie_id_tag() -> Tag<i32> {
        Tag::<i32>::integer("spinel_showcase_vanilla_zombie_id")
    }
}
