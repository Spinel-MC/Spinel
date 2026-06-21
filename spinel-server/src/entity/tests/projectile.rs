use crate::entity::{EntityId, EntityPosition, ProjectileEntity};
use rand::SeedableRng;
use rand::rngs::StdRng;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{Vector3d, Velocity};
use spinel_registry::{EntityType, ItemStack, Material};

#[test]
fn projectile_owner_preserves_shooter_and_disables_base_physics() {
    let shooter = EntityId::from_raw(7);
    let projectile = ProjectileEntity::new(Some(shooter), EntityType::ARROW);

    assert_eq!(projectile.get_shooter(), Some(shooter));
    assert!(!projectile.has_physics());
    assert!(!projectile.was_stuck());
}

#[test]
fn shooter_backed_projectile_spawn_packets_identify_their_shooter() {
    let shooter = EntityId::from_raw(17);
    let velocity = Velocity(Vector3d {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    });
    let protocol_velocity = Velocity(Vector3d {
        x: 0.05,
        y: 0.1,
        z: 0.15,
    });
    [
        EntityType::ARROW,
        EntityType::BREEZE_WIND_CHARGE,
        EntityType::DRAGON_FIREBALL,
        EntityType::FIREBALL,
        EntityType::SMALL_FIREBALL,
        EntityType::SPECTRAL_ARROW,
        EntityType::WIND_CHARGE,
        EntityType::WITHER_SKULL,
    ]
    .into_iter()
    .for_each(|entity_type| {
        let mut projectile = ProjectileEntity::new(Some(shooter), entity_type);
        projectile.set_velocity(velocity);
        assert_eq!(projectile.spawn_packet().data, shooter.get_value());
        assert_eq!(projectile.spawn_packet().velocity, protocol_velocity);
    });

    let snowball = ProjectileEntity::new(Some(shooter), EntityType::SNOWBALL);
    assert_eq!(snowball.spawn_packet().data, 0);
}

#[test]
fn arrow_metadata_owner_matches_minestom_defaults_and_dirty_entries() {
    let mut arrow = ProjectileEntity::new(None, EntityType::ARROW);

    {
        let mut arrow_meta = arrow
            .get_entity_meta_mut()
            .as_arrow()
            .expect("arrow entity must expose ArrowMeta");
        assert!(!arrow_meta.is_critical());
        assert!(!arrow_meta.is_no_clip());
        assert_eq!(arrow_meta.get_piercing_level(), 0);
        assert!(!arrow_meta.is_in_ground());
        assert_eq!(arrow_meta.get_color(), -1);

        arrow_meta.set_critical(true);
        arrow_meta.set_no_clip(true);
        arrow_meta.set_piercing_level(3);
        arrow_meta.set_in_ground(true);
        arrow_meta.set_color(0x112233);
    }

    let arrow_meta = arrow
        .get_entity_meta_mut()
        .as_arrow()
        .expect("arrow entity must expose ArrowMeta");
    assert!(arrow_meta.is_critical());
    assert!(arrow_meta.is_no_clip());
    assert_eq!(arrow_meta.get_piercing_level(), 3);
    assert!(arrow_meta.is_in_ground());
    assert_eq!(arrow_meta.get_color(), 0x112233);
    let dirty_entries = arrow.get_dirty_metadata_packet().unwrap().entries.0;
    assert_eq!(dirty_entries[0].index, 8);
    assert_eq!(dirty_entries[0].value, MetadataValue::Byte(0x03));
    assert_eq!(dirty_entries[1].index, 9);
    assert_eq!(dirty_entries[1].value, MetadataValue::Byte(3));
    assert_eq!(dirty_entries[2].index, 10);
    assert_eq!(dirty_entries[2].value, MetadataValue::Boolean(true));
    assert_eq!(dirty_entries[3].index, 11);
    assert_eq!(dirty_entries[3].value, MetadataValue::VarInt(0x112233));
}

#[test]
fn spectral_arrow_metadata_owner_inherits_abstract_arrow_and_projectile_shooter() {
    let shooter = EntityId::from_raw(32);
    let mut spectral_arrow = ProjectileEntity::new(None, EntityType::SPECTRAL_ARROW);

    {
        let mut spectral_arrow_meta = spectral_arrow
            .get_entity_meta_mut()
            .as_spectral_arrow()
            .expect("spectral arrow entity must expose SpectralArrowMeta");
        spectral_arrow_meta.set_critical(true);
        spectral_arrow_meta.set_shooter(Some(shooter));
        assert!(spectral_arrow_meta.is_critical());
        assert_eq!(spectral_arrow_meta.get_shooter(), Some(shooter));
    }

    assert_eq!(spectral_arrow.spawn_packet().data, shooter.get_value());
}

#[test]
fn trident_and_wither_skull_metadata_owners_match_minestom_definitions() {
    let shooter = EntityId::from_raw(44);
    let mut trident = ProjectileEntity::new(None, EntityType::TRIDENT);
    let mut wither_skull = ProjectileEntity::new(None, EntityType::WITHER_SKULL);

    {
        let mut trident_meta = trident
            .get_entity_meta_mut()
            .as_thrown_trident()
            .expect("trident entity must expose ThrownTridentMeta");
        assert_eq!(trident_meta.get_loyalty_level(), 0);
        assert!(!trident_meta.has_enchantment_glint());
        assert!(!trident_meta.is_no_clip());
        trident_meta.set_loyalty_level(2);
        trident_meta.set_has_enchantment_glint(true);
        trident_meta.set_no_clip(true);
    }
    {
        let mut wither_skull_meta = wither_skull
            .get_entity_meta_mut()
            .as_wither_skull()
            .expect("wither skull entity must expose WitherSkullMeta");
        assert!(!wither_skull_meta.is_invulnerable());
        wither_skull_meta.set_invulnerable(true);
        wither_skull_meta.set_shooter(Some(shooter));
    }

    let trident_meta = trident
        .get_entity_meta_mut()
        .as_thrown_trident()
        .expect("trident entity must expose ThrownTridentMeta");
    assert_eq!(trident_meta.get_loyalty_level(), 2);
    assert!(trident_meta.has_enchantment_glint());
    assert!(trident_meta.is_no_clip());
    assert!(
        wither_skull
            .get_entity_meta_mut()
            .as_wither_skull()
            .expect("wither skull entity must expose WitherSkullMeta")
            .is_invulnerable()
    );
    assert_eq!(wither_skull.spawn_packet().data, shooter.get_value());
}

#[test]
fn fireballs_wind_charges_and_firework_metadata_owners_match_minestom() {
    let shooter = EntityId::from_raw(21);
    let mut fireball = ProjectileEntity::new(None, EntityType::FIREBALL);
    let mut small_fireball = ProjectileEntity::new(None, EntityType::SMALL_FIREBALL);
    let mut dragon_fireball = ProjectileEntity::new(None, EntityType::DRAGON_FIREBALL);
    let mut wind_charge = ProjectileEntity::new(None, EntityType::WIND_CHARGE);
    let mut breeze_wind_charge = ProjectileEntity::new(None, EntityType::BREEZE_WIND_CHARGE);
    let mut firework = ProjectileEntity::new(Some(shooter), EntityType::FIREWORK_ROCKET);
    let fire_charge = ItemStack::of(Material::FIRE_CHARGE);
    let firework_rocket = ItemStack::of(Material::FIREWORK_ROCKET);

    fireball
        .get_entity_meta_mut()
        .as_fireball()
        .expect("fireball entity must expose FireballMeta")
        .set_item(fire_charge.clone());
    fireball
        .get_entity_meta_mut()
        .as_fireball()
        .expect("fireball entity must expose FireballMeta")
        .set_shooter(Some(shooter));
    small_fireball
        .get_entity_meta_mut()
        .as_small_fireball()
        .expect("small fireball entity must expose SmallFireballMeta")
        .set_item(fire_charge.clone());
    small_fireball
        .get_entity_meta_mut()
        .as_small_fireball()
        .expect("small fireball entity must expose SmallFireballMeta")
        .set_shooter(Some(shooter));
    dragon_fireball
        .get_entity_meta_mut()
        .as_dragon_fireball()
        .expect("dragon fireball entity must expose DragonFireballMeta")
        .set_shooter(Some(shooter));
    wind_charge
        .get_entity_meta_mut()
        .as_wind_charge()
        .expect("wind charge entity must expose WindChargeMeta")
        .set_shooter(Some(shooter));
    breeze_wind_charge
        .get_entity_meta_mut()
        .as_breeze_wind_charge()
        .expect("breeze wind charge entity must expose BreezeWindChargeMeta")
        .set_shooter(Some(shooter));
    {
        let mut firework_meta = firework
            .get_entity_meta_mut()
            .as_firework_rocket()
            .expect("firework rocket entity must expose FireworkRocketMeta");
        assert_eq!(firework_meta.get_firework_info(), ItemStack::air());
        assert_eq!(firework_meta.get_shooter_entity_id(), Some(shooter.get_value()));
        assert!(!firework_meta.is_shot_at_angle());
        firework_meta.set_firework_info(firework_rocket.clone());
        firework_meta.set_shot_at_angle(true);
        firework_meta.set_shooter(None);
    }

    assert_eq!(
        fireball
            .get_entity_meta_mut()
            .as_fireball()
            .expect("fireball entity must expose FireballMeta")
            .get_item(),
        fire_charge
    );
    assert_eq!(fireball.spawn_packet().data, shooter.get_value());
    assert_eq!(small_fireball.spawn_packet().data, shooter.get_value());
    assert_eq!(dragon_fireball.spawn_packet().data, shooter.get_value());
    assert_eq!(wind_charge.spawn_packet().data, shooter.get_value());
    assert_eq!(breeze_wind_charge.spawn_packet().data, shooter.get_value());
    let firework_meta = firework
        .get_entity_meta_mut()
        .as_firework_rocket()
        .expect("firework rocket entity must expose FireworkRocketMeta");
    assert_eq!(firework_meta.get_firework_info(), firework_rocket);
    assert_eq!(firework_meta.get_shooter_entity_id(), None);
    assert!(firework_meta.is_shot_at_angle());
}

#[test]
fn item_projectile_metadata_owner_matches_every_minestom_wrapper() {
    let emerald = ItemStack::of(Material::EMERALD);

    let mut snowball = ProjectileEntity::new(None, EntityType::SNOWBALL);
    snowball
        .get_entity_meta_mut()
        .as_snowball()
        .expect("snowball entity must expose SnowballMeta")
        .set_item(emerald.clone());
    assert_eq!(
        snowball
            .get_entity_meta_mut()
            .as_snowball()
            .expect("snowball entity must expose SnowballMeta")
            .get_item(),
        emerald
    );

    let mut egg = ProjectileEntity::new(None, EntityType::EGG);
    egg.get_entity_meta_mut()
        .as_thrown_egg()
        .expect("egg entity must expose ThrownEggMeta")
        .set_item(emerald.clone());
    assert_eq!(
        egg.get_entity_meta_mut()
            .as_thrown_egg()
            .expect("egg entity must expose ThrownEggMeta")
            .get_item(),
        emerald
    );

    let mut ender_pearl = ProjectileEntity::new(None, EntityType::ENDER_PEARL);
    ender_pearl
        .get_entity_meta_mut()
        .as_thrown_ender_pearl()
        .expect("ender pearl entity must expose ThrownEnderPearlMeta")
        .set_item(emerald.clone());
    assert_eq!(
        ender_pearl
            .get_entity_meta_mut()
            .as_thrown_ender_pearl()
            .expect("ender pearl entity must expose ThrownEnderPearlMeta")
            .get_item(),
        emerald
    );

    let mut experience_bottle = ProjectileEntity::new(None, EntityType::EXPERIENCE_BOTTLE);
    experience_bottle
        .get_entity_meta_mut()
        .as_thrown_experience_bottle()
        .expect("experience bottle entity must expose ThrownExperienceBottleMeta")
        .set_item(emerald.clone());
    assert_eq!(
        experience_bottle
            .get_entity_meta_mut()
            .as_thrown_experience_bottle()
            .expect("experience bottle entity must expose ThrownExperienceBottleMeta")
            .get_item(),
        emerald
    );

    let mut splash_potion = ProjectileEntity::new(None, EntityType::SPLASH_POTION);
    splash_potion
        .get_entity_meta_mut()
        .as_splash_potion()
        .expect("splash potion entity must expose SplashPotionMeta")
        .set_item(emerald.clone());
    assert_eq!(
        splash_potion
            .get_entity_meta_mut()
            .as_splash_potion()
            .expect("splash potion entity must expose SplashPotionMeta")
            .get_item(),
        emerald
    );

    let mut lingering_potion = ProjectileEntity::new(None, EntityType::LINGERING_POTION);
    lingering_potion
        .get_entity_meta_mut()
        .as_lingering_potion()
        .expect("lingering potion entity must expose LingeringPotionMeta")
        .set_item(emerald.clone());
    assert_eq!(
        lingering_potion
            .get_entity_meta_mut()
            .as_lingering_potion()
            .expect("lingering potion entity must expose LingeringPotionMeta")
            .get_item(),
        emerald
    );

    let mut eye_of_ender = ProjectileEntity::new(None, EntityType::EYE_OF_ENDER);
    eye_of_ender
        .get_entity_meta_mut()
        .as_eye_of_ender()
        .expect("eye of ender entity must expose EyeOfEnderMeta")
        .set_item(emerald.clone());
    assert_eq!(
        eye_of_ender
            .get_entity_meta_mut()
            .as_eye_of_ender()
            .expect("eye of ender entity must expose EyeOfEnderMeta")
            .get_item(),
        emerald
    );
}

#[test]
fn projectile_shoot_matches_minestom_gravity_compensation_and_velocity_units() {
    let mut projectile = ProjectileEntity::new(None, EntityType::ARROW);
    let mut random = StdRng::seed_from_u64(4);
    projectile.shoot_from_with_rng(
        EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0),
        EntityPosition::new(0.0, 64.0, 10.0, 0.0, 0.0),
        1.0,
        0.0,
        &mut random,
    );

    let velocity = projectile.get_velocity().0;
    assert_eq!(velocity.x, 0.0);
    assert!((velocity.y - 3.9223227027636804).abs() < 0.000001);
    assert!((velocity.z - 19.611613222428958).abs() < 0.000001);
    assert_eq!(projectile.get_position().get_yaw(), 0.0);
    assert!((projectile.get_position().get_pitch() - 11.309933).abs() < 0.00001);
}

#[test]
fn no_gravity_projectile_aims_directly_at_target() {
    let mut projectile = ProjectileEntity::new(None, EntityType::ARROW);
    projectile.set_no_gravity(true);
    let mut random = StdRng::seed_from_u64(8);
    projectile.shoot_from_with_rng(
        EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0),
        EntityPosition::new(0.0, 64.0, 10.0, 0.0, 0.0),
        1.0,
        0.0,
        &mut random,
    );

    assert_eq!(projectile.get_velocity().0.y, 0.0);
    assert_eq!(projectile.get_velocity().0.z, 20.0);
    assert_eq!(projectile.get_position().get_pitch(), 0.0);
}
