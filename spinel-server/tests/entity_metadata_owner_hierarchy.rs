use spinel_registry::EntityType;
use spinel_server::entity::GenericEntity;

#[test]
fn bee_meta_exposes_inherited_animal_mob_and_living_behavior() {
    let mut bee = GenericEntity::new(EntityType::BEE);
    let mut bee_meta = bee
        .entity_meta_mut()
        .bee()
        .expect("bee entity must expose BeeMeta");

    bee_meta.set_angry(true);
    bee_meta.set_baby(true);
    bee_meta.set_no_ai(true);
    bee_meta.set_hand_active(true);

    assert!(bee_meta.is_angry());
    assert!(bee_meta.is_baby());
    assert!(bee_meta.is_no_ai());
    assert!(bee_meta.is_hand_active());
}

#[test]
fn monster_and_flying_meta_expose_their_inherited_mob_behavior() {
    let mut guardian = GenericEntity::new(EntityType::GUARDIAN);
    let mut guardian_meta = guardian
        .entity_meta_mut()
        .guardian()
        .expect("guardian entity must expose GuardianMeta");
    guardian_meta.set_retracting_spikes(true);
    guardian_meta.set_aggressive(true);
    guardian_meta.set_health(12.0);

    assert!(guardian_meta.is_retracting_spikes());
    assert!(guardian_meta.is_aggressive());
    assert_eq!(guardian_meta.health(), 12.0);

    let mut ghast = GenericEntity::new(EntityType::GHAST);
    let mut ghast_meta = ghast
        .entity_meta_mut()
        .ghast()
        .expect("ghast entity must expose GhastMeta");
    ghast_meta.set_attacking(true);
    ghast_meta.set_left_handed(true);

    assert!(ghast_meta.is_attacking());
    assert!(ghast_meta.is_left_handed());
}

#[test]
fn unrelated_entities_cannot_obtain_entity_specific_metadata() {
    let mut zombie = GenericEntity::new(EntityType::ZOMBIE);

    assert!(zombie.entity_meta_mut().bee().is_none());
    assert!(zombie.entity_meta_mut().armadillo().is_none());
    assert!(zombie.entity_meta_mut().guardian().is_none());
    assert!(zombie.entity_meta_mut().ghast().is_none());
    assert!(zombie.entity_meta_mut().vex().is_none());
}
