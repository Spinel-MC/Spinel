use crate::entity::{EntityPosition, GenericEntity, LivingEntity};
use spinel_nbt::parse_snbt_compound;
use spinel_registry::EntityType;

#[test]
fn summon_nbt_applies_position_rotation_name_and_base_flags() {
    let mut entity = GenericEntity::new(EntityType::ARMOR_STAND);
    entity.set_position(EntityPosition::new(0.0, 64.0, 0.0, 0.0, 0.0));
    let nbt = parse_snbt_compound(
        r#"{Pos:[1.5d,70.0d,-2.5d],Rotation:[90.0f,15.0f],CustomName:'{"text":"Guide"}',CustomNameVisible:1b,OnGround:1b,HasVisualFire:1b,Invisible:1b,Glowing:1b,Silent:1b,NoGravity:1b}"#,
    )
    .unwrap();

    entity.apply_summon_nbt(&nbt);

    assert_eq!(entity.get_position().get_x(), 1.5);
    assert_eq!(entity.get_position().get_y(), 70.0);
    assert_eq!(entity.get_position().get_z(), -2.5);
    assert_eq!(entity.get_position().get_yaw(), 90.0);
    assert_eq!(entity.get_position().get_pitch(), 15.0);
    assert_eq!(entity.get_custom_name().unwrap().to_plain_string(), "Guide");
    assert!(entity.is_custom_name_visible());
    assert!(entity.is_on_ground());
    assert!(entity.has_visual_fire());
    assert!(entity.is_on_fire());
    assert!(entity.is_invisible());
    assert!(entity.is_glowing());
    assert!(entity.is_silent());
    assert!(entity.has_no_gravity());
}

#[test]
fn summon_nbt_preserves_custom_name_shadow_color() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let nbt = parse_snbt_compound(r#"{CustomName:{color:"gold",shadow_color:-43691,text:"mud"}}"#)
        .unwrap();

    entity.apply_summon_nbt(&nbt);

    let custom_name = entity.get_custom_name().unwrap();
    let custom_name_value = serde_json::to_value(custom_name.clone()).unwrap();
    let custom_name_nbt_value =
        spinel_nbt::nbt_to_json(spinel_nbt::Nbt::Compound(custom_name.to_nbt_compound()));
    assert_eq!(custom_name.to_plain_string(), "mud");
    assert_eq!(custom_name_value["shadow_color"], serde_json::json!(-43691));
    assert_eq!(
        custom_name_nbt_value["shadow_color"],
        serde_json::json!(-43691)
    );
}

#[test]
fn summon_nbt_visual_fire_survives_ordinary_fire_tick_clearing() {
    let mut entity = LivingEntity::new(EntityType::ZOMBIE);
    let nbt = parse_snbt_compound("{HasVisualFire:1b}").unwrap();

    entity.apply_summon_nbt(&nbt);
    entity.set_fire_ticks(1);
    entity.tick_living_state();

    assert_eq!(entity.get_fire_ticks(), 0);
    assert!(entity.get_entity().has_visual_fire());
    assert!(entity.is_on_fire());
}

#[test]
fn ordinary_fire_clears_when_visual_fire_is_absent() {
    let mut entity = LivingEntity::new(EntityType::ZOMBIE);

    entity.set_fire_ticks(1);
    entity.tick_living_state();

    assert_eq!(entity.get_fire_ticks(), 0);
    assert!(!entity.get_entity().has_visual_fire());
    assert!(!entity.is_on_fire());
}

#[test]
fn summon_nbt_applies_living_and_type_specific_state_only_to_matching_entities() {
    let living_nbt = parse_snbt_compound(
        "{Invulnerable:1b,Health:7.5f,Small:1b,ShowArms:1b,NoBasePlate:1b,Marker:1b}",
    )
    .unwrap();
    let mut armor_stand = LivingEntity::new(EntityType::ARMOR_STAND);
    armor_stand.apply_summon_nbt(&living_nbt);

    assert!(armor_stand.is_invulnerable());
    assert_eq!(armor_stand.get_health(), 7.5);
    {
        let armor_stand_meta = armor_stand
            .get_entity_meta_mut()
            .as_armor_stand()
            .expect("armor stand entity must expose ArmorStandMeta");
        assert!(armor_stand_meta.is_small());
        assert!(armor_stand_meta.has_arms());
        assert!(armor_stand_meta.has_no_base_plate());
        assert!(armor_stand_meta.is_marker());
    }

    let slime_nbt = parse_snbt_compound("{Size:4,Health:12.0f}").unwrap();
    let mut slime = LivingEntity::new(EntityType::SLIME);
    slime.apply_summon_nbt(&slime_nbt);
    assert_eq!(
        slime
            .get_entity_meta_mut()
            .as_slime()
            .expect("slime entity must expose SlimeMeta")
            .get_size(),
        4
    );
    assert_eq!(slime.get_health(), 12.0);

    let mut item = GenericEntity::new(EntityType::ITEM);
    item.apply_summon_nbt(&slime_nbt);
    assert_eq!(item.get_entity_type(), EntityType::ITEM);
}
