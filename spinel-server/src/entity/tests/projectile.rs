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

    assert_eq!(projectile.shooter(), Some(shooter));
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
        assert_eq!(projectile.spawn_packet().data, shooter.value());
        assert_eq!(projectile.spawn_packet().velocity, protocol_velocity);
    });

    let snowball = ProjectileEntity::new(Some(shooter), EntityType::SNOWBALL);
    assert_eq!(snowball.spawn_packet().data, 0);
}

#[test]
fn abstract_arrow_metadata_matches_minestom_defaults_and_dirty_entries() {
    let mut arrow = ProjectileEntity::new(None, EntityType::ARROW);

    assert!(!arrow.is_critical());
    assert!(!arrow.has_no_clip());
    assert_eq!(arrow.piercing_level(), 0);
    assert!(!arrow.is_in_ground());
    assert_eq!(arrow.color(), -1);

    arrow.set_critical(true);
    arrow.set_no_clip(true);
    arrow.set_piercing_level(3);
    arrow.set_in_ground(true);
    arrow.set_color(0x112233);

    assert!(arrow.is_critical());
    assert!(arrow.has_no_clip());
    assert_eq!(arrow.piercing_level(), 3);
    assert!(arrow.is_in_ground());
    assert_eq!(arrow.color(), 0x112233);
    let dirty_entries = arrow.dirty_metadata_packet().unwrap().entries.0;
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
fn trident_and_wither_skull_metadata_match_minestom_definitions() {
    let mut trident = ProjectileEntity::new(None, EntityType::TRIDENT);
    let mut wither_skull = ProjectileEntity::new(None, EntityType::WITHER_SKULL);

    assert_eq!(trident.loyalty_level(), 0);
    assert!(!trident.has_enchantment_glint());
    assert!(!wither_skull.is_invulnerable_wither_skull());

    trident.set_loyalty_level(2);
    trident.set_enchantment_glint(true);
    wither_skull.set_invulnerable_wither_skull(true);

    assert_eq!(trident.loyalty_level(), 2);
    assert!(trident.has_enchantment_glint());
    assert!(wither_skull.is_invulnerable_wither_skull());
}

#[test]
fn projectile_item_and_firework_metadata_match_minestom_definitions() {
    let shooter = EntityId::from_raw(21);
    let mut snowball = ProjectileEntity::new(None, EntityType::SNOWBALL);
    let mut fireball = ProjectileEntity::new(None, EntityType::FIREBALL);
    let mut firework = ProjectileEntity::new(Some(shooter), EntityType::FIREWORK_ROCKET);
    let diamond = ItemStack::of(Material::DIAMOND);
    let fire_charge = ItemStack::of(Material::FIRE_CHARGE);
    let firework_rocket = ItemStack::of(Material::FIREWORK_ROCKET);

    assert_eq!(snowball.projectile_item(), ItemStack::air());
    assert_eq!(fireball.projectile_item(), ItemStack::air());
    assert_eq!(firework.firework_info(), ItemStack::air());
    assert_eq!(firework.firework_shooter_entity_id(), Some(shooter.value()));
    assert!(!firework.is_shot_at_angle());

    snowball.set_projectile_item(diamond.clone());
    fireball.set_projectile_item(fire_charge.clone());
    firework.set_firework_info(firework_rocket.clone());
    firework.set_shot_at_angle(true);
    firework.set_shooter(None);

    assert_eq!(snowball.projectile_item(), diamond);
    assert_eq!(fireball.projectile_item(), fire_charge);
    assert_eq!(firework.firework_info(), firework_rocket);
    assert_eq!(firework.firework_shooter_entity_id(), None);
    assert!(firework.is_shot_at_angle());
}

#[test]
fn item_projectile_metadata_slot_matches_every_minestom_wrapper() {
    let emerald = ItemStack::of(Material::EMERALD);
    [
        EntityType::EGG,
        EntityType::ENDER_PEARL,
        EntityType::EXPERIENCE_BOTTLE,
        EntityType::EYE_OF_ENDER,
        EntityType::FIREBALL,
        EntityType::LINGERING_POTION,
        EntityType::SMALL_FIREBALL,
        EntityType::SNOWBALL,
        EntityType::SPLASH_POTION,
    ]
    .into_iter()
    .for_each(|entity_type| {
        let mut projectile = ProjectileEntity::new(None, entity_type);
        projectile.set_projectile_item(emerald.clone());

        assert_eq!(projectile.projectile_item(), emerald);
    });
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

    let velocity = projectile.velocity().0;
    assert_eq!(velocity.x, 0.0);
    assert!((velocity.y - 3.9223227027636804).abs() < 0.000001);
    assert!((velocity.z - 19.611613222428958).abs() < 0.000001);
    assert_eq!(projectile.position().yaw(), 0.0);
    assert!((projectile.position().pitch() - 11.309933).abs() < 0.00001);
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

    assert_eq!(projectile.velocity().0.y, 0.0);
    assert_eq!(projectile.velocity().0.z, 20.0);
    assert_eq!(projectile.position().pitch(), 0.0);
}
