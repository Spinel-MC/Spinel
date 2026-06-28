use spinel::{
    nbt::{Tag, TagReadable},
    registry::{EntityType, ItemStack, Material},
    server::{
        MinecraftServer,
        entity::{
            Entity, EntityCreature, EntityId, EntityPosition, Player,
            pathfinding::{PathRequest, VanillaGroundNodeFollower},
        },
        world::{BlockPosition, World},
    },
    utils::component::Component,
};
use std::io;
use uuid::Uuid;

pub struct EntityShowcase;

pub struct EntityShowcaseControls {
    dual_pathfinding_stick: ItemStack,
    minestom_pathfinding_stick: ItemStack,
    vanilla_pathfinding_stick: ItemStack,
}

impl EntityShowcaseControls {
    pub fn give_to_player(self, player: &mut Player) -> Vec<bool> {
        player.add_item_stacks(vec![
            self.dual_pathfinding_stick,
            self.minestom_pathfinding_stick,
            self.vanilla_pathfinding_stick,
        ])
    }

    pub fn dual_pathfinding_stick(&self) -> &ItemStack {
        &self.dual_pathfinding_stick
    }

    pub fn minestom_pathfinding_stick(&self) -> &ItemStack {
        &self.minestom_pathfinding_stick
    }

    pub fn vanilla_pathfinding_stick(&self) -> &ItemStack {
        &self.vanilla_pathfinding_stick
    }
}

impl EntityShowcase {
    pub fn spawn(
        server: &mut MinecraftServer,
        world_id: Uuid,
        origin: EntityPosition,
    ) -> io::Result<EntityShowcaseControls> {
        let minestom_zombie_position = EntityPosition::new(
            origin.get_x() + 3.0,
            origin.get_y(),
            origin.get_z(),
            origin.get_yaw(),
            origin.get_pitch(),
        );
        let vanilla_zombie_position = EntityPosition::new(
            origin.get_x() + 3.0,
            origin.get_y(),
            origin.get_z() + 2.0,
            origin.get_yaw(),
            origin.get_pitch(),
        );
        let item_position = EntityPosition::new(
            origin.get_x() + 5.0,
            origin.get_y() + 1.0,
            origin.get_z() + 5.0,
            origin.get_yaw(),
            origin.get_pitch(),
        );
        let Some(world) = server.world_manager.world_mut(world_id) else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Player world was not found.",
            ));
        };
        let minestom_zombie = Self::zombie(minestom_zombie_position, "Minestom Physics");
        let minestom_zombie_id = minestom_zombie.get_entity_id();
        if !minestom_zombie.set_instance(world) {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Minestom physics zombie add cancelled.",
            ));
        }
        let mut vanilla_zombie = Self::zombie(vanilla_zombie_position, "Vanilla Physics");
        vanilla_zombie
            .get_navigator_mut()
            .set_node_follower(VanillaGroundNodeFollower::default());
        let vanilla_zombie_id = vanilla_zombie.get_entity_id();
        if !vanilla_zombie.set_instance(world) {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Vanilla physics zombie add cancelled.",
            ));
        }
        world.spawn_item_entity(ItemStack::of(Material::DIAMOND), item_position)?;
        Ok(EntityShowcaseControls {
            dual_pathfinding_stick: Self::dual_pathfinding_stick(
                minestom_zombie_id,
                vanilla_zombie_id,
            ),
            minestom_pathfinding_stick: Self::single_pathfinding_stick(
                "Minestom Zombie Pathfinder",
                Self::minestom_zombie_id_tag(),
                minestom_zombie_id,
            ),
            vanilla_pathfinding_stick: Self::single_pathfinding_stick(
                "Vanilla Zombie Pathfinder",
                Self::vanilla_zombie_id_tag(),
                vanilla_zombie_id,
            ),
        })
    }

    pub fn pathfind(
        world: &mut World,
        pathfinding_stick: &ItemStack,
        block_position: BlockPosition,
    ) -> bool {
        let minestom_zombie_id = pathfinding_stick.get_tag(&Self::minestom_zombie_id_tag());
        let vanilla_zombie_id = pathfinding_stick.get_tag(&Self::vanilla_zombie_id_tag());
        if minestom_zombie_id.is_none() && vanilla_zombie_id.is_none() {
            return false;
        }
        let destination = EntityPosition::new(
            f64::from(block_position.x) + 0.5,
            f64::from(block_position.y) + 1.0,
            f64::from(block_position.z) + 0.5,
            0.0,
            0.0,
        );
        let minestom_path_was_accepted = minestom_zombie_id
            .is_some_and(|zombie_id| Self::pathfind_zombie(world, zombie_id, destination));
        let vanilla_path_was_accepted = vanilla_zombie_id
            .is_some_and(|zombie_id| Self::pathfind_zombie(world, zombie_id, destination));
        minestom_path_was_accepted || vanilla_path_was_accepted
    }

    pub(super) fn zombie(position: EntityPosition, physics_name: &str) -> EntityCreature {
        let mut zombie = EntityCreature::new(EntityType::ZOMBIE);
        zombie.set_position(position);
        zombie.set_custom_name(Some(Component::text(physics_name).build()));
        zombie.set_custom_name_visible(true);
        zombie.get_entity_meta_mut().as_zombie().expect("msg");

        zombie
    }

    fn dual_pathfinding_stick(
        minestom_zombie_id: EntityId,
        vanilla_zombie_id: EntityId,
    ) -> ItemStack {
        ItemStack::of(Material::STICK)
            .with_custom_name(Component::text("Dual Zombie Pathfinder").build())
            .with_tag(
                &Self::minestom_zombie_id_tag(),
                Some(minestom_zombie_id.get_value()),
            )
            .with_tag(
                &Self::vanilla_zombie_id_tag(),
                Some(vanilla_zombie_id.get_value()),
            )
    }

    fn single_pathfinding_stick(
        name: &str,
        zombie_id_tag: Tag<i32>,
        zombie_id: EntityId,
    ) -> ItemStack {
        ItemStack::of(Material::STICK)
            .with_custom_name(Component::text(name).build())
            .with_tag(&zombie_id_tag, Some(zombie_id.get_value()))
    }

    fn pathfind_zombie(world: &mut World, zombie_id: i32, destination: EntityPosition) -> bool {
        world
            .get_entity_mut(EntityId::from_raw(zombie_id))
            .is_some_and(|entity| {
                let Entity::Creature(zombie) = entity else {
                    return false;
                };
                zombie
                    .set_path_to(PathRequest::from(destination))
                    .is_ok_and(|path_was_accepted| path_was_accepted)
            })
    }

    fn minestom_zombie_id_tag() -> Tag<i32> {
        Tag::<i32>::integer("spinel_showcase_minestom_zombie_id")
    }

    fn vanilla_zombie_id_tag() -> Tag<i32> {
        Tag::<i32>::integer("spinel_showcase_vanilla_zombie_id")
    }
}
