use crate::entity::{Entity, EntityPosition, EquipmentSlot};
use crate::world::{ChunkPosition, World};
use spinel_nbt::parse_snbt_compound;
use spinel_network::types::Identifier;
use spinel_registry::{EntityType, Material};

#[test]
fn world_spawn_entity_delegates_summon_nbt_to_the_entity_owner() {
    let mut world = World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("summon_nbt"),
    );
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let nbt = parse_snbt_compound(
        r#"{Pos:[4.5d,72.0d,6.5d],Rotation:[45.0f,10.0f],CustomName:'{"text":"Spawned"}',Glowing:1b,Small:1b}"#,
    )
    .unwrap();

    let entity_id = world
        .spawn_entity(
            EntityType::ARMOR_STAND,
            EntityPosition::new(1.0, 65.0, 1.0, 0.0, 0.0),
            Some(&nbt),
        )
        .unwrap();
    let Entity::Living(entity) = world.get_entity_mut(entity_id).unwrap() else {
        panic!("spawned entity must preserve the living owner");
    };

    assert_eq!(
        entity.get_position(),
        EntityPosition::new(4.5, 72.0, 6.5, 45.0, 10.0)
    );
    assert_eq!(
        entity.get_custom_name().unwrap().to_plain_string(),
        "Spawned"
    );
    assert!(entity.is_glowing());
    assert!(entity
        .get_entity_meta_mut()
        .as_armor_stand()
        .expect("armor stand entity must expose ArmorStandMeta")
        .is_small());
}

#[test]
fn world_spawn_entity_uses_creature_owner_for_command_spawned_mobs() {
    let mut world = World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("summon_mob"),
    );
    let nbt = parse_snbt_compound(
        r#"{CustomName:{color:"dark_purple",text:"Spawned Zombie"},Glowing:1b,equipment:{head:{id:"minecraft:tnt",count:1}}}"#,
    )
    .unwrap();

    let entity_id = world
        .spawn_entity(
            EntityType::ZOMBIE,
            EntityPosition::new(1.0, 65.0, 1.0, 0.0, 0.0),
            Some(&nbt),
        )
        .unwrap();
    let Entity::Creature(entity) = world.get_entity(entity_id).unwrap() else {
        panic!("command-spawned mob must use the creature owner");
    };

    assert_eq!(
        entity.get_custom_name().unwrap().to_plain_string(),
        "Spawned Zombie"
    );
    assert!(entity.is_glowing());
    let helmet = entity.get_equipment(EquipmentSlot::Helmet);
    assert_eq!(helmet.material(), &Material::TNT);
    assert_eq!(helmet.amount(), 1);
}
