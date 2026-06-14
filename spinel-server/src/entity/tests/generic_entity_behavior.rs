use super::super::generic_entity::{EntityAerodynamics, EntityPosition, GenericEntity};
use crate::entity::TimedPotionEffect;
use crate::entity::metadata::definitions;
use crate::entity::{EntityId, EquipmentSlot};
use crate::world::ChunkPosition;
use spinel_core::network::clientbound::play::entity_animation::EntityAnimation;
use spinel_core::network::clientbound::play::update_attributes::EntityAttributeModifier;
use spinel_nbt::{Tag, TagReadable, TagWritable};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{Identifier, Vector3d};
use spinel_registry::{
    Attribute, DataComponentType, EntityBoundingBox, EntityType, ItemStack, Material,
};
use spinel_utils::component::Component;

#[test]
fn generic_entity_owns_minestom_entity_identity_and_type() {
    let entity = GenericEntity::new(EntityType::ZOMBIE);

    assert!(entity.entity_id().value() > 0);
    assert_eq!(entity.entity_type(), EntityType::ZOMBIE);
    assert_eq!(entity.bounding_box(), EntityType::ZOMBIE.bounding_box());
    assert_eq!(entity.identity().uuid(), entity.uuid());
    assert_eq!(entity.pointers().uuid(), entity.uuid());
    assert_eq!(entity.pointers().entity_id(), entity.entity_id());
    assert_eq!(entity.pointers().identity(), entity.identity());
}

#[test]
fn generic_living_entities_use_extracted_vanilla_base_attributes() {
    let zombie = GenericEntity::new(EntityType::ZOMBIE);
    let cow = GenericEntity::new(EntityType::COW);

    assert!(
        (zombie.attribute_value(
            Attribute::MOVEMENT_SPEED.protocol_id(),
            Attribute::MOVEMENT_SPEED.default_value(),
        ) - 0.23000000417232513)
            .abs()
            < f64::EPSILON
    );
    assert_eq!(
        zombie.attribute_value(
            Attribute::FOLLOW_RANGE.protocol_id(),
            Attribute::FOLLOW_RANGE.default_value(),
        ),
        35.0
    );
    assert!(
        (cow.attribute_value(
            Attribute::MOVEMENT_SPEED.protocol_id(),
            Attribute::MOVEMENT_SPEED.default_value(),
        ) - 0.20000000298023224)
            .abs()
            < f64::EPSILON
    );
    assert_eq!(cow.max_health(), 10.0);
    assert_eq!(cow.health(), 10.0);
}

#[test]
fn generic_entity_switch_type_preserves_bounding_box_like_minestom() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let original_bounding_box = entity.bounding_box();
    let original_aerodynamics = entity.aerodynamics();

    entity.switch_entity_type(EntityType::ARROW);

    assert_eq!(entity.entity_type(), EntityType::ARROW);
    assert_eq!(entity.bounding_box(), original_bounding_box);
    assert_ne!(entity.aerodynamics(), original_aerodynamics);
    assert_eq!(
        entity.aerodynamics(),
        EntityAerodynamics::from_entity_type(EntityType::ARROW)
    );
    assert!(entity.has_entity_collision());
    assert!(!entity.prevents_block_placement());

    entity.switch_entity_type(EntityType::TEXT_DISPLAY);

    assert!(!entity.has_entity_collision());
    assert!(entity.prevents_block_placement());
}

#[test]
fn generic_entity_bounding_box_distance_and_position_api_match_minestom_shape() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let mut other_entity = GenericEntity::new(EntityType::ZOMBIE);

    entity.set_bounding_box_dimensions(1.0, 2.0, 3.0);
    entity.set_position(EntityPosition::new(1.0, 2.0, 3.0, 45.0, 15.0));
    entity.refresh_position(EntityPosition::new(4.0, 6.0, 3.0, 90.0, 30.0));
    entity.set_view(180.0, 60.0, 120.0);
    other_entity.set_position(EntityPosition::new(4.0, 9.0, 7.0, 0.0, 0.0));

    assert_eq!(entity.bounding_box(), EntityBoundingBox::new(1.0, 2.0, 3.0));
    assert_eq!(
        entity.previous_position(),
        EntityPosition::new(4.0, 6.0, 3.0, 90.0, 30.0)
    );
    assert_eq!(entity.position().yaw(), 180.0);
    assert_eq!(entity.position().pitch(), 60.0);
    assert_eq!(entity.head_rotation(), 120.0);
    assert_eq!(entity.distance_squared_to_entity(&other_entity), 25.0);
    assert_eq!(entity.distance_to_entity(&other_entity), 5.0);
}

#[test]
fn generic_entity_look_at_and_chunk_api_match_minestom_shape() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let mut target = GenericEntity::new(EntityType::ZOMBIE);

    entity.set_position(EntityPosition::new(-17.0, 64.0, 31.0, 0.0, 0.0));
    target.set_position(EntityPosition::new(-17.0, 64.0, 47.0, 0.0, 0.0));
    entity.look_at_entity(&target);

    assert_eq!(entity.chunk(), ChunkPosition::new(-2, 1));
    assert_eq!(entity.position().yaw(), 0.0);
    assert_eq!(entity.position().pitch(), -0.0);
    assert_eq!(entity.head_rotation(), 0.0);

    entity.look_at_position(EntityPosition::new(-1.0, 64.0, 31.0, 0.0, 0.0));

    assert_eq!(entity.position().yaw(), -90.0);
    assert_eq!(entity.head_rotation(), -90.0);
}

#[test]
fn generic_entity_tag_handler_matches_minestom_entity_surface() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let owner_tag = Tag::<String>::string("owner");

    entity.set_tag(&owner_tag, Some("spinel".to_string()));

    assert_eq!(entity.get_tag(&owner_tag), Some("spinel".to_string()));
}

#[test]
fn generic_entity_passenger_leash_and_status_api_match_minestom_shape() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let passenger_id = EntityId::next();
    let leash_holder_id = EntityId::next();
    let leashed_entity_id = EntityId::next();

    assert_eq!(entity.vehicle(), None);
    assert!(!entity.add_passenger(entity.entity_id()));
    assert!(entity.add_passenger(passenger_id));
    assert!(!entity.add_passenger(passenger_id));
    entity.set_vehicle(entity.entity_id());
    assert_eq!(entity.vehicle(), None);
    entity.set_vehicle(passenger_id);
    assert_eq!(entity.vehicle(), Some(passenger_id));
    entity.clear_vehicle();
    assert_eq!(entity.vehicle(), None);
    assert!(entity.has_passenger());
    assert!(entity.passengers().contains(&passenger_id));

    let passenger_packet = entity.passenger_packet();
    assert_eq!(
        passenger_packet.vehicle_entity_id,
        entity.entity_id().value()
    );
    assert_eq!(
        passenger_packet.passenger_entity_ids.0,
        vec![passenger_id.value()]
    );

    entity.set_leash_holder(Some(leash_holder_id));
    assert_eq!(entity.leash_holder(), Some(leash_holder_id));
    assert!(entity.add_leashed_entity(leashed_entity_id));
    assert!(entity.leashed_entities().contains(&leashed_entity_id));
    assert!(entity.remove_leashed_entity(leashed_entity_id));
    assert!(entity.remove_passenger(passenger_id));
    assert!(!entity.has_passenger());

    let status_packet = entity.status_packet(3);
    let triggered_status_packet = entity.trigger_status(3);
    assert_eq!(status_packet.entity_id, entity.entity_id().value());
    assert_eq!(status_packet.status, 3);
    assert_eq!(triggered_status_packet.entity_id, status_packet.entity_id);
    assert_eq!(triggered_status_packet.status, status_packet.status);
}

#[test]
fn generic_entity_base_metadata_api_matches_minestom_shape() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let custom_name = Component::text("Name").build();

    assert!(!entity.is_on_fire());
    assert!(!entity.is_invisible());
    assert!(!entity.is_glowing());
    assert_eq!(entity.pose(), 0);
    assert_eq!(entity.custom_name(), None);
    assert!(!entity.is_custom_name_visible());
    assert!(!entity.is_silent());
    assert!(!entity.has_no_gravity());

    entity.set_on_fire(true);
    entity.set_invisible(true);
    entity.set_glowing(true);
    entity.set_pose(2);
    entity.set_custom_name(Some(custom_name.clone()));
    entity.set_custom_name_visible(true);
    entity.set_silent(true);
    entity.set_no_gravity(true);

    assert!(entity.is_on_fire());
    assert!(entity.is_invisible());
    assert!(entity.is_glowing());
    assert_eq!(entity.pose(), 2);
    assert_eq!(entity.custom_name(), Some(custom_name));
    assert!(entity.is_custom_name_visible());
    assert!(entity.is_silent());
    assert!(entity.has_no_gravity());
    assert_eq!(entity.eye_height(), 0.2);
    assert!(entity.dirty_metadata_packet().is_some());
}

#[test]
fn generic_entity_data_components_match_minestom_entity_component_surface() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let component =
        DataComponentType::<String>::new(9000, Identifier::vanilla_static("test_component"));

    assert_eq!(entity.component(component.clone()), None);
    entity.set_component(component.clone(), "value".to_string());

    assert_eq!(entity.component(component), Some("value".to_string()));
}

#[test]
fn generic_living_state_damage_fire_and_death_match_minestom_surface() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);

    entity.set_arrow_count(4);
    entity.set_fire_ticks(2);
    entity.set_item_pickup_cooldown(2);
    assert!(entity.is_on_fire());
    assert_eq!(entity.arrow_count(), 4);
    assert_eq!(entity.fire_ticks(), 2);
    assert_eq!(entity.item_pickup_cooldown(), 2);

    assert!(entity.damage("minecraft:generic", 5.0));
    assert_eq!(entity.health(), 15.0);
    assert_eq!(entity.last_damage_source(), Some("minecraft:generic"));
    assert!(!entity.is_dead());

    entity.set_invulnerable(true);
    assert!(!entity.damage("minecraft:generic", 5.0));
    assert!(entity.is_immune_to_damage("minecraft:generic"));
    entity.set_invulnerable(false);
    assert!(entity.damage("minecraft:generic", 100.0));
    assert!(entity.is_dead());

    entity.heal();
    assert_eq!(entity.health(), entity.max_health());
    assert!(entity.is_dead());

    entity.tick();

    assert_eq!(entity.fire_ticks(), 1);
    assert_eq!(entity.item_pickup_cooldown(), 1);
}

#[test]
fn generic_living_attributes_effects_animation_and_bed_api_match_minestom_surface() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let modifier = EntityAttributeModifier::base_attack_speed(-2.0);

    entity.attribute(4, 4.0).add_modifier(modifier.clone());
    assert_eq!(entity.attribute_value(4, 4.0), 2.0);
    assert!(entity.attributes().len() > 1);
    assert_eq!(
        entity.update_attributes_packet().attributes.len(),
        entity
            .attributes()
            .iter()
            .filter(|attribute| attribute.attribute().is_syncable())
            .count()
    );
    assert_eq!(
        entity
            .attribute(4, 4.0)
            .remove_modifier(&modifier.modifier_id)
            .unwrap(),
        modifier
    );
    assert_eq!(entity.attribute_value(4, 4.0), 4.0);

    let effect_packet = entity.add_effect(TimedPotionEffect::new(1, 2, 2, 6, entity.ticks()));
    assert_eq!(effect_packet.effect_id, 1);
    assert!(entity.has_effect(1));
    assert_eq!(entity.effect_level(1), Some(2));
    assert_eq!(entity.active_effects().len(), 1);
    entity.tick();
    assert!(entity.has_effect(1));
    entity.tick();
    assert!(!entity.has_effect(1));

    entity.add_effect(TimedPotionEffect::new(2, 0, 20, 0, entity.ticks()));
    assert_eq!(entity.remove_effect(2).unwrap().effect_id, 2);
    entity.add_effect(TimedPotionEffect::new(3, 0, 20, 0, entity.ticks()));
    assert_eq!(entity.clear_effects().len(), 1);

    assert_eq!(
        entity.swing_main_hand().animation,
        EntityAnimation::SwingMainArm
    );
    assert_eq!(
        entity.swing_off_hand().animation,
        EntityAnimation::SwingOffHand
    );
    entity.enter_bed(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    assert!(entity.bed_position().is_some());
    assert_eq!(entity.leave_bed().animation, EntityAnimation::LeaveBed);
    assert!(entity.bed_position().is_none());
}

#[test]
fn generic_living_motion_team_and_collision_api_match_minestom_surface() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let aerodynamics = EntityAerodynamics::new(0.8, 0.9, 0.04);

    assert_eq!(entity.expanded_bounding_box().width(), 1.6);
    entity.set_flying_with_elytra(true);
    assert!(entity.is_flying_with_elytra());
    entity.set_team(Some("red".to_string()));
    assert_eq!(entity.team(), Some("red"));
    assert!(!entity.living_metadata().entries().is_empty());

    entity.set_aerodynamics(aerodynamics);
    assert_eq!(entity.aerodynamics(), aerodynamics);
    entity.increment_gravity_tick_count();
    assert_eq!(entity.gravity_tick_count(), 1);
    entity.reset_gravity_tick_count();
    assert_eq!(entity.gravity_tick_count(), 0);

    entity.set_synchronization_ticks(5);
    assert_eq!(entity.synchronization_ticks(), 5);
    entity.synchronize_next_tick();
    assert_eq!(entity.synchronization_ticks(), 5);

    assert!(entity.has_entity_collision());
    entity.set_entity_collision(false);
    assert!(!entity.has_entity_collision());
    assert!(entity.prevents_block_placement());
    entity.set_prevents_block_placement(false);
    assert!(!entity.prevents_block_placement());

    assert_eq!(
        entity.relative_start(),
        Vector3d {
            x: -0.3,
            y: 0.0,
            z: -0.3
        }
    );
    assert_eq!(
        entity.relative_end(),
        Vector3d {
            x: 0.3,
            y: 1.95,
            z: 0.3
        }
    );
    assert!(entity.intersects_box_at(
        Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        EntityBoundingBox::new(0.6, 1.95, 0.6),
    ));
    assert!(!entity.intersects_box_at(
        Vector3d {
            x: 4.0,
            y: 0.0,
            z: 0.0
        },
        EntityBoundingBox::new(0.6, 1.95, 0.6),
    ));
    assert!(entity.intersects_box_swept(
        Vector3d {
            x: -1.0,
            y: 0.0,
            z: 0.0
        },
        Vector3d {
            x: 2.0,
            y: 0.0,
            z: 0.0
        },
        Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        EntityBoundingBox::new(0.6, 1.95, 0.6),
    ));

    entity.take_knockback(0.0, 0.5, -0.25);
    assert_eq!(entity.velocity().0.x, 0.0);
    assert_eq!(entity.velocity().0.z, 0.0);

    entity.schedule_remove_after_ticks(1);
    entity.tick();
    assert!(entity.is_removed());

    let mut duration_entity = GenericEntity::new(EntityType::ZOMBIE);
    duration_entity.schedule_remove_after_duration(std::time::Duration::from_millis(51));
    duration_entity.tick();
    assert!(!duration_entity.is_removed());
    duration_entity.tick();
    assert!(duration_entity.is_removed());
}

#[test]
fn generic_entity_knockback_matches_minestom_base_and_living_rules() {
    let mut living_entity = GenericEntity::new(EntityType::ZOMBIE);
    living_entity.set_on_ground(true);
    living_entity
        .attribute(Attribute::KNOCKBACK_RESISTANCE.protocol_id(), 0.0)
        .set_base_value(0.5);

    living_entity.take_knockback(0.4, 1.0, 0.0);

    assert!((living_entity.velocity().0.x + 4.0).abs() < 0.000_001);
    assert!((living_entity.velocity().0.y - 4.0).abs() < 0.000_001);
    assert_eq!(living_entity.velocity().0.z, 0.0);

    let mut base_entity = GenericEntity::new(EntityType::ITEM);
    base_entity.set_on_ground(true);

    base_entity.take_knockback(0.4, 1.0, 0.0);

    assert!((base_entity.velocity().0.x + 8.0).abs() < 0.000_001);
    assert!((base_entity.velocity().0.y - 8.0).abs() < 0.000_001);
    assert_eq!(base_entity.velocity().0.z, 0.0);
}

#[test]
fn generic_living_health_heal_and_kill_state_match_minestom() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_on_ground(true);
    entity.take_knockback(0.4, 1.0, 0.0);
    entity.set_health(0.0);

    assert!(entity.is_dead());
    assert_eq!(entity.pose(), 6);
    assert_eq!(
        entity.velocity().0,
        Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    );

    let mut healed_entity = GenericEntity::new(EntityType::ZOMBIE);
    healed_entity.set_health(4.0);
    healed_entity.heal();

    assert_eq!(healed_entity.health(), healed_entity.max_health());
    assert_eq!(
        healed_entity
            .living_metadata()
            .value(&definitions::living_entity::health()),
        MetadataValue::Float(healed_entity.max_health())
    );
}

#[test]
fn removed_generic_entity_does_not_tick() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);

    assert!(!entity.is_active());
    assert_eq!(entity.alive_ticks(), 0);

    entity.tick();

    assert_eq!(entity.ticks(), 1);
    assert_eq!(entity.alive_ticks(), 1);

    entity.update(20);

    assert_eq!(entity.ticks(), 2);
    assert_eq!(entity.alive_ticks(), 2);

    entity.remove();
    entity.tick();

    assert_eq!(entity.ticks(), 2);
    assert_eq!(entity.alive_ticks(), 2);
}

#[test]
fn generic_entity_builds_spawn_and_metadata_packets_from_owned_state() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(1.0, 2.0, 3.0, 45.0, 90.0));
    entity
        .metadata_mut()
        .set(&definitions::air_ticks(), MetadataValue::VarInt(10));

    let spawn_packet = entity.spawn_packet();
    let metadata_packet = entity.dirty_metadata_packet().unwrap();

    assert_eq!(spawn_packet.entity_id, entity.entity_id().value());
    assert_eq!(spawn_packet.entity_type, EntityType::ZOMBIE.id());
    assert_eq!(spawn_packet.x, 1.0);
    assert_eq!(metadata_packet.entity_id, entity.entity_id().value());
    assert_eq!(metadata_packet.entries.0.len(), 1);
    assert!(entity.dirty_metadata_packet().is_none());
}

#[test]
fn generic_entity_metadata_defaults_are_not_redundantly_sent() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);

    assert_eq!(
        entity.metadata().value(&definitions::air_ticks()),
        definitions::air_ticks().default_value().clone()
    );
    assert!(entity.metadata_packet().entries.0.is_empty());
    assert!(entity.dirty_metadata_packet().is_none());
}

#[test]
fn generic_entity_builds_minestom_equipment_packet_from_owned_equipment() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_equipment(
        EquipmentSlot::MainHand,
        ItemStack::of(Material::DIAMOND_SWORD),
    );
    entity.set_equipment(
        EquipmentSlot::Helmet,
        ItemStack::of(Material::DIAMOND_HELMET),
    );

    let equipment_packet = entity.equipment_packet();

    assert_eq!(equipment_packet.entity_id, entity.entity_id().value());
    assert_eq!(equipment_packet.equipment.0.len(), 7);
    assert_eq!(
        equipment_packet.equipment.0[0]
            .item
            .to_item_stack()
            .material(),
        &Material::DIAMOND_SWORD
    );
    assert_eq!(
        equipment_packet.equipment.0[5]
            .item
            .to_item_stack()
            .material(),
        &Material::DIAMOND_HELMET
    );
}

#[test]
fn generic_entity_builds_velocity_head_look_and_teleport_packets_from_owned_state() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(1.0, 2.0, 3.0, 45.0, 90.0).with_head_yaw(30.0));
    entity.set_velocity(spinel_network::types::Velocity(
        spinel_network::types::Vector3d {
            x: 0.25,
            y: 0.0,
            z: -0.25,
        },
    ));

    let velocity_packet = entity.velocity_packet();
    let head_look_packet = entity.head_look_packet();
    let teleport_packet = entity.teleport_packet();

    assert_eq!(velocity_packet.entity_id, entity.entity_id().value());
    assert_eq!(velocity_packet.velocity.0.x, 0.0125);
    assert_eq!(velocity_packet.velocity.0.z, -0.0125);
    assert_eq!(head_look_packet.entity_id, entity.entity_id().value());
    assert_eq!(head_look_packet.head_yaw.0, 30.0);
    assert_eq!(teleport_packet.entity_id, entity.entity_id().value());
    assert_eq!(teleport_packet.position, entity.position().as_vector());
    assert_eq!(teleport_packet.delta.x, 0.0125);
    assert_eq!(teleport_packet.delta.z, -0.0125);
}

#[test]
fn generic_entity_builds_minestom_relative_movement_packets_from_previous_position() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let previous_position = EntityPosition::new(1.0, 2.0, 3.0, 0.0, 0.0);
    entity.set_position(EntityPosition::new(1.5, 2.0, 2.5, 90.0, 45.0));

    let position_packet = entity.position_delta_packet(previous_position, true);
    let position_and_rotation_packet =
        entity.position_and_rotation_delta_packet(previous_position, false);
    let rotation_packet = entity.rotation_packet(true);

    assert_eq!(position_packet.delta_x, 2048);
    assert_eq!(position_packet.delta_z, -2048);
    assert!(position_packet.on_ground);
    assert_eq!(position_and_rotation_packet.delta_x, 2048);
    assert_eq!(position_and_rotation_packet.yaw.0, 90.0);
    assert!(!position_and_rotation_packet.on_ground);
    assert_eq!(rotation_packet.pitch.0, 45.0);
    assert!(rotation_packet.on_ground);
}

#[test]
fn generic_entity_position_clamps_to_minestom_coordinate_limit() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);

    entity.set_position(EntityPosition::new(
        3_000_000_000.0,
        -3_000_000_000.0,
        12.0,
        0.0,
        0.0,
    ));

    assert_eq!(entity.position().x(), 2_000_000_000.0);
    assert_eq!(entity.position().y(), -2_000_000_000.0);
    assert_eq!(entity.position().z(), 12.0);
}
