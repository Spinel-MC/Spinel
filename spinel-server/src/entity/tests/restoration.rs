use crate::entity::{
    Entity, EntityCreature, EntityPosition, ExperienceOrb, GenericEntity, ItemEntity, LivingEntity,
    Player, ProjectileEntity,
};
use spinel_network::types::{Vector3d, Velocity};
use spinel_registry::{EntityType, ItemStack};
use std::net::SocketAddr;
use uuid::Uuid;

#[test]
fn persisted_uuid_constructors_preserve_each_runtime_entity_uuid() {
    let generic_uuid = Uuid::new_v4();
    let living_uuid = Uuid::new_v4();
    let creature_uuid = Uuid::new_v4();
    let experience_orb_uuid = Uuid::new_v4();
    let item_uuid = Uuid::new_v4();
    let projectile_uuid = Uuid::new_v4();
    let player_uuid = Uuid::new_v4();

    assert_eq!(
        GenericEntity::with_uuid(EntityType::ARMOR_STAND, generic_uuid).get_uuid(),
        generic_uuid
    );
    assert_eq!(
        LivingEntity::with_uuid(EntityType::ZOMBIE, living_uuid).get_uuid(),
        living_uuid
    );
    assert_eq!(
        EntityCreature::with_uuid(EntityType::ZOMBIE, creature_uuid).get_uuid(),
        creature_uuid
    );
    assert_eq!(
        ExperienceOrb::with_uuid(7, experience_orb_uuid).get_uuid(),
        experience_orb_uuid
    );
    assert_eq!(
        ItemEntity::with_uuid(ItemStack::air(), item_uuid).get_uuid(),
        item_uuid
    );
    assert_eq!(
        ProjectileEntity::with_uuid(None, EntityType::SNOWBALL, projectile_uuid).get_uuid(),
        projectile_uuid
    );
    assert_eq!(
        Player::new(
            player_uuid,
            "restoration-player".to_owned(),
            773,
            SocketAddr::from(([127, 0, 0, 1], 25565)),
        )
        .get_uuid(),
        player_uuid
    );
}

#[test]
fn entity_restoration_state_mutators_cover_every_runtime_variant() {
    let position = EntityPosition::new(12.0, 64.0, -8.0, 45.0, 15.0);
    let velocity = Velocity(Vector3d {
        x: 0.4,
        y: -0.2,
        z: 0.8,
    });
    let player_uuid = Uuid::new_v4();
    let mut entities = vec![
        Entity::Creature(EntityCreature::with_uuid(
            EntityType::ZOMBIE,
            Uuid::new_v4(),
        )),
        Entity::ExperienceOrb(ExperienceOrb::with_uuid(7, Uuid::new_v4())),
        Entity::Generic(GenericEntity::with_uuid(
            EntityType::ARMOR_STAND,
            Uuid::new_v4(),
        )),
        Entity::Item(ItemEntity::with_uuid(ItemStack::air(), Uuid::new_v4())),
        Entity::Living(LivingEntity::with_uuid(EntityType::ZOMBIE, Uuid::new_v4())),
        Entity::Player(Player::new(
            player_uuid,
            "restoration-player".to_owned(),
            773,
            SocketAddr::from(([127, 0, 0, 1], 25566)),
        )),
        Entity::Projectile(ProjectileEntity::with_uuid(
            None,
            EntityType::SNOWBALL,
            Uuid::new_v4(),
        )),
    ];

    entities.iter_mut().for_each(|entity| {
        entity.set_position(position);
        entity.set_velocity(velocity);

        assert_eq!(entity.get_position(), position);
        assert_eq!(entity.get_velocity(), velocity);
    });
}
