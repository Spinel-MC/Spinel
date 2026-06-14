use crate::entity::{Entity, GenericEntity};
use crate::world::{SetEntityTargetError, World};
use spinel_network::types::Identifier;
use spinel_registry::EntityType;

#[test]
fn world_owns_guardian_and_wither_entity_target_resolution() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let guardian = GenericEntity::new(EntityType::GUARDIAN);
    let wither = GenericEntity::new(EntityType::WITHER);
    let target = GenericEntity::new(EntityType::ZOMBIE);
    let guardian_id = guardian.entity_id();
    let wither_id = wither.entity_id();
    let target_id = target.entity_id();
    world.add_entity(Entity::Generic(guardian));
    world.add_entity(Entity::Generic(wither));
    world.add_entity(Entity::Generic(target));

    world
        .set_guardian_target(guardian_id, Some(target_id))
        .unwrap();
    world
        .set_wither_center_head_target(wither_id, Some(target_id))
        .unwrap();
    world
        .set_wither_left_head_target(wither_id, Some(target_id))
        .unwrap();
    world
        .set_wither_right_head_target(wither_id, Some(target_id))
        .unwrap();

    assert_eq!(
        world.guardian_target(guardian_id).unwrap().entity_id(),
        target_id
    );
    assert_eq!(
        world
            .wither_center_head_target(wither_id)
            .unwrap()
            .entity_id(),
        target_id
    );
    assert_eq!(
        world
            .wither_left_head_target(wither_id)
            .unwrap()
            .entity_id(),
        target_id
    );
    assert_eq!(
        world
            .wither_right_head_target(wither_id)
            .unwrap()
            .entity_id(),
        target_id
    );

    world.set_guardian_target(guardian_id, None).unwrap();
    assert!(world.guardian_target(guardian_id).is_none());
}

#[test]
fn world_target_setters_reject_missing_targets_and_wrong_owner_types() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let zombie = GenericEntity::new(EntityType::ZOMBIE);
    let zombie_id = zombie.entity_id();
    let missing_target = GenericEntity::new(EntityType::PLAYER).entity_id();
    world.add_entity(Entity::Generic(zombie));

    assert_eq!(
        world.set_guardian_target(zombie_id, None),
        Err(SetEntityTargetError::WrongOwnerType {
            owner_id: zombie_id,
            expected_type: EntityType::GUARDIAN,
        })
    );
    assert_eq!(
        world.set_wither_center_head_target(zombie_id, Some(missing_target)),
        Err(SetEntityTargetError::TargetNotFound {
            target_id: missing_target,
        })
    );
}
