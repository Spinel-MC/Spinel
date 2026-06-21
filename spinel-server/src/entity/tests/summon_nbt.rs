use crate::entity::{EntityPosition, GenericEntity};
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
    assert!(entity.is_on_fire());
    assert!(entity.is_invisible());
    assert!(entity.is_glowing());
    assert!(entity.is_silent());
    assert!(entity.has_no_gravity());
}

#[test]
fn summon_nbt_applies_living_and_type_specific_state_only_to_matching_entities() {
    let living_nbt = parse_snbt_compound(
        "{Invulnerable:1b,Health:7.5f,Small:1b,ShowArms:1b,NoBasePlate:1b,Marker:1b}",
    )
    .unwrap();
    let mut armor_stand = GenericEntity::new(EntityType::ARMOR_STAND);
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
    let mut slime = GenericEntity::new(EntityType::SLIME);
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
    assert_eq!(item.get_health(), 1.0);
    assert!(item.get_entity_meta_mut().as_slime().is_none());
}
