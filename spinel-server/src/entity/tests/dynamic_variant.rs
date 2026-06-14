use crate::entity::GenericEntity;
use spinel_registry::data_components::vanilla_components::{
    CAT_VARIANT, CHICKEN_VARIANT, COW_VARIANT, FROG_VARIANT, PIG_VARIANT, WOLF_SOUND_VARIANT,
    WOLF_VARIANT, ZOMBIE_NAUTILUS_VARIANT,
};
use spinel_registry::{
    EntityType, Identifier, Registries, RegistryKey, cat_variant, chicken_variant, cow_variant,
    frog_variant, pig_variant, wolf_sound_variant, wolf_variant, zombie_nautilus_variant,
};

#[test]
fn dynamic_entity_variants_resolve_through_active_registries() {
    let mut registries = Registries::empty();
    let cat_variant = registries
        .cat_variant_mut()
        .register(
            Identifier::new("example", "cat"),
            cat_variant::CatVariant::default(),
        )
        .unwrap();
    let chicken_variant = registries
        .chicken_variant_mut()
        .register(
            Identifier::new("example", "chicken"),
            chicken_variant::ChickenVariant::default(),
        )
        .unwrap();
    let cow_variant = registries
        .cow_variant_mut()
        .register(
            Identifier::new("example", "cow"),
            cow_variant::CowVariant::default(),
        )
        .unwrap();
    let frog_variant = registries
        .frog_variant_mut()
        .register(
            Identifier::new("example", "frog"),
            frog_variant::FrogVariant::default(),
        )
        .unwrap();
    let pig_variant = registries
        .pig_variant_mut()
        .register(
            Identifier::new("example", "pig"),
            pig_variant::PigVariant::default(),
        )
        .unwrap();
    let wolf_variant = registries
        .wolf_variant_mut()
        .register(
            Identifier::new("example", "wolf"),
            wolf_variant::WolfVariant::default(),
        )
        .unwrap();
    let wolf_sound_variant = registries
        .wolf_sound_variant_mut()
        .register(
            Identifier::new("example", "wolf_sound"),
            wolf_sound_variant::WolfSoundVariant::default(),
        )
        .unwrap();
    let zombie_nautilus_variant = registries
        .zombie_nautilus_variant_mut()
        .register(
            Identifier::new("example", "zombie_nautilus"),
            zombie_nautilus_variant::ZombieNautilusVariant::default(),
        )
        .unwrap();

    let mut cat = GenericEntity::new(EntityType::CAT);
    let mut chicken = GenericEntity::new(EntityType::CHICKEN);
    let mut cow = GenericEntity::new(EntityType::COW);
    let mut frog = GenericEntity::new(EntityType::FROG);
    let mut pig = GenericEntity::new(EntityType::PIG);
    let mut wolf = GenericEntity::new(EntityType::WOLF);
    let mut zombie_nautilus = GenericEntity::new(EntityType::ZOMBIE_NAUTILUS);

    cat.set_component_with_registries(&registries, CAT_VARIANT, cat_variant.clone())
        .unwrap();
    chicken
        .set_component_with_registries(&registries, CHICKEN_VARIANT, chicken_variant.clone())
        .unwrap();
    cow.set_component_with_registries(&registries, COW_VARIANT, cow_variant.clone())
        .unwrap();
    frog.set_component_with_registries(&registries, FROG_VARIANT, frog_variant.clone())
        .unwrap();
    pig.set_component_with_registries(&registries, PIG_VARIANT, pig_variant.clone())
        .unwrap();
    wolf.set_component_with_registries(&registries, WOLF_VARIANT, wolf_variant.clone())
        .unwrap();
    wolf.set_component_with_registries(&registries, WOLF_SOUND_VARIANT, wolf_sound_variant.clone())
        .unwrap();
    zombie_nautilus
        .set_component_with_registries(
            &registries,
            ZOMBIE_NAUTILUS_VARIANT,
            zombie_nautilus_variant.clone(),
        )
        .unwrap();

    assert_eq!(cat.cat_variant(&registries), Some(cat_variant.clone()));
    assert_eq!(
        chicken.chicken_variant(&registries),
        Some(chicken_variant.clone())
    );
    assert_eq!(cow.cow_variant(&registries), Some(cow_variant.clone()));
    assert_eq!(frog.frog_variant(&registries), Some(frog_variant.clone()));
    assert_eq!(pig.pig_variant(&registries), Some(pig_variant.clone()));
    assert_eq!(wolf.wolf_variant(&registries), Some(wolf_variant.clone()));
    assert_eq!(
        wolf.wolf_sound_variant(&registries),
        Some(wolf_sound_variant.clone())
    );
    assert_eq!(
        zombie_nautilus.zombie_nautilus_variant(&registries),
        Some(zombie_nautilus_variant.clone())
    );

    assert_eq!(
        cat.component_with_registries(&registries, CAT_VARIANT),
        Some(cat_variant)
    );
    assert_eq!(
        chicken.component_with_registries(&registries, CHICKEN_VARIANT),
        Some(chicken_variant)
    );
    assert_eq!(
        cow.component_with_registries(&registries, COW_VARIANT),
        Some(cow_variant)
    );
    assert_eq!(
        frog.component_with_registries(&registries, FROG_VARIANT),
        Some(frog_variant)
    );
    assert_eq!(
        pig.component_with_registries(&registries, PIG_VARIANT),
        Some(pig_variant)
    );
    assert_eq!(
        wolf.component_with_registries(&registries, WOLF_VARIANT),
        Some(wolf_variant)
    );
    assert_eq!(
        wolf.component_with_registries(&registries, WOLF_SOUND_VARIANT),
        Some(wolf_sound_variant)
    );
    assert_eq!(
        zombie_nautilus.component_with_registries(&registries, ZOMBIE_NAUTILUS_VARIANT),
        Some(zombie_nautilus_variant)
    );
}

#[test]
fn dynamic_entity_variant_setters_reject_unregistered_keys_without_mutation() {
    let mut registries = Registries::empty();
    let registered_variant = registries
        .pig_variant_mut()
        .register(
            Identifier::new("example", "registered"),
            pig_variant::PigVariant::default(),
        )
        .unwrap();
    let unregistered_variant =
        RegistryKey::<pig_variant::PigVariant>::new(Identifier::new("example", "missing"));
    let mut pig = GenericEntity::new(EntityType::PIG);

    pig.set_pig_variant(&registries, registered_variant.clone())
        .unwrap();
    let error = pig
        .set_pig_variant(&registries, unregistered_variant.clone())
        .unwrap_err();

    assert_eq!(error.registry(), registries.pig_variant().key());
    assert_eq!(error.variant(), unregistered_variant.key());
    assert_eq!(pig.pig_variant(&registries), Some(registered_variant));
}
