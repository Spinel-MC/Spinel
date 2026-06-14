use crate::entity::metadata::{
    ArmadilloState, AxolotlVariant, CopperGolemState, CopperGolemWeatherState, CreeperState,
    FoxVariant, HorseColor, HorseMarking, HorseVariant, LlamaVariant, MooshroomVariant, PandaGene,
    ParrotColor, PufferfishState, RabbitVariant, SalmonSize, SnifferState, SpellcasterIllagerSpell,
    TropicalFishPattern, TropicalFishVariant, VillagerData, VillagerLevel,
};
use crate::entity::{EntityId, EntityPosition, GenericEntity, PlayerHand};
use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::entity_animation::EntityAnimation;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::set_equipment::SetEquipmentPacket;
use spinel_core::network::clientbound::play::update_attributes::{
    EntityAttributeModifier, UpdateAttributesPacket,
};
use spinel_network::ConnectionState;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{
    MainHand, Particle, Position, ResolvableProfile, Slot, TeleportFlags, Vector3d, Vector3f,
    Velocity,
};
use spinel_registry::{EntityType, ItemStack, VillagerProfession, VillagerType};
use spinel_utils::color::DyeColor;
use spinel_utils::component::events::HoverEvent;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicI32, Ordering};

#[test]
fn generic_entity_teleport_overloads_update_position_velocity_chunks_and_flags() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let position = EntityPosition::new(1.0, 64.0, 2.0, 90.0, 45.0);
    let velocity = Velocity(Vector3d {
        x: 0.1,
        y: 0.2,
        z: 0.3,
    });

    let plain_teleport = entity.teleport(position);
    assert_eq!(plain_teleport.position(), position);
    assert_eq!(entity.position(), position);

    let velocity_teleport = entity.teleport_with_velocity(position, velocity);
    assert_eq!(velocity_teleport.velocity(), velocity);
    assert_eq!(entity.velocity(), velocity);

    let chunk_teleport = entity.teleport_with_chunks_and_flags(
        position,
        Some(vec![1, 2, 3]),
        TeleportFlags::absolute(),
    );
    assert_eq!(chunk_teleport.chunks(), Some([1, 2, 3].as_slice()));
    assert_eq!(chunk_teleport.flags().bitmask(), TeleportFlags::DELTA_COORD);

    let full_teleport = entity.teleport_with_velocity_chunks_and_flags(
        EntityPosition::new(4.0, 70.0, 5.0, 180.0, 0.0),
        velocity,
        Some(vec![4]),
        TeleportFlags::absolute(),
    );
    assert_eq!(full_teleport.position().x(), 4.0);
    assert_eq!(full_teleport.chunks(), Some([4].as_slice()));
}

#[test]
fn generic_entity_teleport_resolves_minestom_relative_position_and_velocity_flags() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(10.0, 20.0, 30.0, 40.0, 50.0));
    entity.set_velocity(Velocity(Vector3d {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    }));

    let preserved_velocity = entity.teleport(EntityPosition::new(4.0, 5.0, 6.0, 7.0, 8.0));

    assert_eq!(
        preserved_velocity.velocity(),
        Velocity(Vector3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        })
    );
    assert_eq!(
        preserved_velocity.flags().bitmask(),
        TeleportFlags::DELTA_COORD
    );

    let flags = TeleportFlags::from_bitmask(
        TeleportFlags::X
            | TeleportFlags::Z
            | TeleportFlags::Y_ROTATION
            | TeleportFlags::DELTA_X
            | TeleportFlags::DELTA_Z
            | TeleportFlags::ROTATE_DELTA,
    );
    let teleport = entity.teleport_with_velocity_chunks_and_flags(
        EntityPosition::new(2.0, 70.0, -3.0, 15.0, 25.0),
        Velocity(Vector3d {
            x: 0.5,
            y: 8.0,
            z: -0.5,
        }),
        None,
        flags,
    );

    assert_eq!(
        teleport.position(),
        EntityPosition::new(6.0, 70.0, 3.0, 22.0, 25.0)
    );
    assert_eq!(
        teleport.velocity(),
        Velocity(Vector3d {
            x: 1.5,
            y: 8.0,
            z: 2.5,
        })
    );
    assert_eq!(
        teleport.teleport_position(),
        EntityPosition::new(2.0, 70.0, -3.0, 15.0, 25.0)
    );
    assert_eq!(
        teleport.teleport_velocity(),
        Velocity(Vector3d {
            x: 0.5,
            y: 8.0,
            z: -0.5,
        })
    );
}

#[test]
fn generic_entity_refresh_position_packet_control_overloads_update_owned_position() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);

    entity.refresh_position_with_packet_controls(
        EntityPosition::new(1.0, 2.0, 3.0, 4.0, 5.0),
        true,
        false,
    );
    assert_eq!(entity.position().x(), 1.0);

    entity.refresh_position_ignoring_view(EntityPosition::new(6.0, 7.0, 8.0, 9.0, 10.0), true);
    assert_eq!(entity.position().x(), 6.0);
    assert_eq!(entity.previous_position().x(), 1.0);
}

#[test]
fn generic_entity_from_client_swing_overloads_preserve_hand_animation() {
    let entity = GenericEntity::new(EntityType::ZOMBIE);

    assert_eq!(
        entity.swing_main_hand_from_client(true).animation,
        EntityAnimation::SwingMainArm
    );
    assert_eq!(
        entity.swing_off_hand_from_client(false).animation,
        EntityAnimation::SwingOffHand
    );
}

#[test]
fn generic_entity_hover_event_uses_type_uuid_and_custom_name() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_custom_name(Some(spinel_utils::component::text::TextComponent::literal(
        "Zombie",
    )));

    let HoverEvent::ShowEntity(hover_entity) = entity.as_hover_event() else {
        panic!("expected show entity hover event");
    };

    assert_eq!(hover_entity.entity_type, "minecraft:zombie");
    assert_eq!(hover_entity.id, entity.uuid().to_string());
    assert!(hover_entity.name.is_some());
}

#[test]
fn generic_entity_base_metadata_api_matches_minestom_entity_meta_defaults() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);

    assert!(!entity.is_on_fire());
    assert!(!entity.is_sneaking());
    assert!(!entity.is_sprinting());
    assert!(!entity.is_swimming());
    assert!(!entity.is_invisible());
    assert!(!entity.is_glowing());
    assert!(!entity.is_flying_with_elytra());
    assert_eq!(entity.air_ticks(), 300);
    assert_eq!(entity.ticks_frozen(), 0);

    entity.set_on_fire(true);
    entity.set_sneaking(true);
    entity.set_sprinting(true);
    entity.set_swimming(true);
    entity.set_invisible(true);
    entity.set_glowing(true);
    entity.set_flying_with_elytra(true);
    entity.set_air_ticks(42);
    entity.set_ticks_frozen(7);

    assert!(entity.is_on_fire());
    assert!(entity.is_sneaking());
    assert!(entity.is_sprinting());
    assert!(entity.is_swimming());
    assert!(entity.is_invisible());
    assert!(entity.is_glowing());
    assert!(entity.is_flying_with_elytra());
    assert_eq!(entity.air_ticks(), 42);
    assert_eq!(entity.ticks_frozen(), 7);
    assert!(
        entity
            .metadata()
            .entries()
            .iter()
            .any(|entry| entry.index == 1 && entry.value == MetadataValue::VarInt(42))
    );
    assert!(
        entity
            .metadata()
            .entries()
            .iter()
            .any(|entry| entry.index == 7 && entry.value == MetadataValue::VarInt(7))
    );
}

#[test]
fn generic_entity_living_mob_and_ageable_metadata_api_matches_minestom_meta_surface() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let original_bounding_box = entity.bounding_box();
    let effect_particle = Particle::effect();
    let bed_position = Position { x: 1, y: 64, z: 2 };

    assert!(!entity.is_hand_active());
    assert_eq!(entity.active_hand(), PlayerHand::Main);
    assert!(!entity.is_in_riptide_spin_attack());
    assert!(entity.effect_particles().is_empty());
    assert!(!entity.is_potion_effect_ambient());
    assert_eq!(entity.arrow_count(), 0);
    assert_eq!(entity.bee_stinger_count(), 0);
    assert_eq!(entity.bed_in_which_sleeping_position(), None);
    assert!(!entity.is_no_ai());
    assert!(!entity.is_left_handed());
    assert!(!entity.is_aggressive());
    assert!(!entity.is_baby());

    entity.set_hand_active(true);
    entity.set_active_hand(PlayerHand::Off);
    entity.set_in_riptide_spin_attack(true);
    entity.set_effect_particles(vec![effect_particle.clone()]);
    entity.set_potion_effect_ambient(true);
    entity.set_arrow_count(3);
    entity.set_bee_stinger_count(4);
    entity.set_bed_in_which_sleeping_position(Some(bed_position));
    entity.set_no_ai(true);
    entity.set_left_handed(true);
    entity.set_aggressive(true);
    entity.set_health(13.5);
    entity.set_baby(true);

    assert!(entity.is_hand_active());
    assert_eq!(entity.active_hand(), PlayerHand::Off);
    assert!(entity.is_in_riptide_spin_attack());
    assert_eq!(entity.effect_particles(), vec![effect_particle]);
    assert!(entity.is_potion_effect_ambient());
    assert_eq!(entity.arrow_count(), 3);
    assert_eq!(entity.bee_stinger_count(), 4);
    assert_eq!(entity.bed_in_which_sleeping_position(), Some(bed_position));
    assert!(entity.is_no_ai());
    assert!(entity.is_left_handed());
    assert!(entity.is_aggressive());
    assert!(entity.is_baby());
    assert_eq!(
        entity.bounding_box().width(),
        original_bounding_box.width() / 2.0
    );
    assert_eq!(
        entity.bounding_box().height(),
        original_bounding_box.height() / 2.0
    );
    assert!(
        entity
            .metadata()
            .entries()
            .iter()
            .any(|entry| entry.index == 9 && entry.value == MetadataValue::Float(13.5))
    );
    assert!(
        entity
            .metadata()
            .entries()
            .iter()
            .any(|entry| entry.index == 12 && entry.value == MetadataValue::VarInt(3))
    );

    entity.set_baby(false);

    assert!(!entity.is_baby());
    assert_eq!(entity.bounding_box(), original_bounding_box);
}

#[test]
fn generic_entity_vehicle_avatar_player_and_mannequin_metadata_api_matches_minestom_meta_surface() {
    let mut entity = GenericEntity::new(EntityType::PLAYER);
    let mut mannequin = GenericEntity::new(EntityType::MANNEQUIN);
    let description = spinel_utils::component::text::TextComponent::literal("description");
    let profile = ResolvableProfile::default();

    assert_eq!(entity.shaking_ticks(), 0);
    assert_eq!(entity.shaking_direction(), 1);
    assert_eq!(entity.shaking_multiplier(), 0.0);
    assert_eq!(entity.main_hand(), MainHand::Right);
    assert_eq!(entity.displayed_skin_parts(), 0);
    assert_eq!(entity.additional_hearts(), 0.0);
    assert_eq!(entity.score(), 0);
    assert_eq!(entity.left_shoulder_entity_data(), None);
    assert_eq!(entity.right_shoulder_entity_data(), None);
    assert_eq!(mannequin.profile(), profile);
    assert!(!mannequin.is_immovable());

    entity.set_shaking_ticks(5);
    entity.set_shaking_direction(-1);
    entity.set_shaking_multiplier(0.75);
    entity.set_main_hand(MainHand::Left);
    entity.set_cape_enabled(true);
    entity.set_jacket_enabled(true);
    entity.set_left_sleeve_enabled(true);
    entity.set_right_sleeve_enabled(true);
    entity.set_left_leg_enabled(true);
    entity.set_right_leg_enabled(true);
    entity.set_hat_enabled(true);
    entity.set_displayed_skin_parts(0x7f);
    entity.set_additional_hearts(2.5);
    entity.set_score(12);
    entity.set_left_shoulder_entity_data(Some(1));
    entity.set_right_shoulder_entity_data(Some(2));
    mannequin.set_profile(profile.clone());
    mannequin.set_immovable(true);
    mannequin.set_description(Some(description.clone()));

    assert_eq!(entity.shaking_ticks(), 5);
    assert_eq!(entity.shaking_direction(), -1);
    assert_eq!(entity.shaking_multiplier(), 0.75);
    assert_eq!(entity.main_hand(), MainHand::Left);
    assert!(entity.is_cape_enabled());
    assert!(entity.is_jacket_enabled());
    assert!(entity.is_left_sleeve_enabled());
    assert!(entity.is_right_sleeve_enabled());
    assert!(entity.is_left_leg_enabled());
    assert!(entity.is_right_leg_enabled());
    assert!(entity.is_hat_enabled());
    assert_eq!(entity.displayed_skin_parts(), 0x7f);
    assert_eq!(entity.additional_hearts(), 2.5);
    assert_eq!(entity.score(), 12);
    assert_eq!(entity.left_shoulder_entity_data(), Some(1));
    assert_eq!(entity.right_shoulder_entity_data(), Some(2));
    assert_eq!(mannequin.profile(), profile);
    assert!(mannequin.is_immovable());
    assert_eq!(mannequin.description(), Some(description));
}

#[test]
fn generic_entity_display_metadata_api_matches_minestom_display_meta_surface() {
    let mut entity = GenericEntity::new(EntityType::TEXT_DISPLAY);
    let mut block_display = GenericEntity::new(EntityType::BLOCK_DISPLAY);
    let mut item_display = GenericEntity::new(EntityType::ITEM_DISPLAY);
    let mut text_display = GenericEntity::new(EntityType::TEXT_DISPLAY);
    let mut interaction = GenericEntity::new(EntityType::INTERACTION);
    let translation = Vector3f {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let scale = Vector3f {
        x: 2.0,
        y: 3.0,
        z: 4.0,
    };
    let rotation = spinel_network::types::Quaternionf {
        x: 0.0,
        y: 1.0,
        z: 0.0,
        w: 0.0,
    };
    let displayed_item = Slot::from_item_stack(&ItemStack::air());
    let text = spinel_utils::component::text::TextComponent::literal("display");

    assert_eq!(entity.transformation_interpolation_start_delta(), 0);
    assert_eq!(entity.transformation_interpolation_duration(), 0);
    assert_eq!(entity.position_rotation_interpolation_duration(), 0);
    assert_eq!(
        entity.display_scale(),
        Vector3f {
            x: 1.0,
            y: 1.0,
            z: 1.0
        }
    );
    assert_eq!(entity.brightness_override(), -1);
    assert_eq!(entity.block_light(), 0);
    assert_eq!(entity.sky_light(), 0);
    assert_eq!(entity.display_view_range(), 1.0);
    assert_eq!(entity.shadow_strength(), 1.0);
    assert_eq!(entity.line_width(), 200);
    assert_eq!(entity.background_color(), 0x40000000);
    assert_eq!(entity.text_opacity(), -1);
    assert_eq!(interaction.interaction_width(), 1.0);
    assert_eq!(interaction.interaction_height(), 1.0);
    assert!(!interaction.has_interaction_response());

    entity.set_transformation_interpolation_start_delta(1);
    entity.set_transformation_interpolation_duration(2);
    entity.set_position_rotation_interpolation_duration(3);
    entity.set_display_translation(translation);
    entity.set_display_scale(scale);
    entity.set_left_rotation(rotation);
    entity.set_right_rotation(rotation);
    entity.set_billboard_render_constraints(3);
    entity.set_brightness(4, 5);
    entity.set_display_view_range(6.0);
    entity.set_shadow_radius(7.0);
    entity.set_shadow_strength(8.0);
    entity.set_display_width(9.0);
    entity.set_display_height(10.0);
    entity.set_glow_color_override(11);
    block_display.set_displayed_block_state(12);
    item_display.set_displayed_item(displayed_item.clone());
    item_display.set_display_context(6);
    text_display.set_display_text(text.clone());
    text_display.set_line_width(13);
    text_display.set_background_color(14);
    text_display.set_text_opacity(15);
    text_display.set_text_shadow(true);
    text_display.set_text_see_through(true);
    text_display.set_uses_default_text_background(true);
    text_display.set_text_alignment(2);
    interaction.set_interaction_width(1.5);
    interaction.set_interaction_height(2.5);
    interaction.set_interaction_response(true);

    assert_eq!(entity.transformation_interpolation_start_delta(), 1);
    assert_eq!(entity.transformation_interpolation_duration(), 2);
    assert_eq!(entity.position_rotation_interpolation_duration(), 3);
    assert_eq!(entity.display_translation(), translation);
    assert_eq!(entity.display_scale(), scale);
    assert_eq!(entity.left_rotation(), rotation);
    assert_eq!(entity.right_rotation(), rotation);
    assert_eq!(entity.billboard_render_constraints(), 3);
    assert_eq!(entity.block_light(), 4);
    assert_eq!(entity.sky_light(), 5);
    assert_eq!(entity.display_view_range(), 6.0);
    assert_eq!(entity.shadow_radius(), 7.0);
    assert_eq!(entity.shadow_strength(), 8.0);
    assert_eq!(entity.display_width(), 9.0);
    assert_eq!(entity.display_height(), 10.0);
    assert_eq!(entity.glow_color_override(), 11);
    assert_eq!(block_display.displayed_block_state(), 12);
    assert_eq!(item_display.displayed_item(), displayed_item);
    assert_eq!(item_display.display_context(), 6);
    assert_eq!(text_display.display_text(), text);
    assert_eq!(text_display.line_width(), 13);
    assert_eq!(text_display.background_color(), 14);
    assert_eq!(text_display.text_opacity(), 15);
    assert!(text_display.has_text_shadow());
    assert!(text_display.is_text_see_through());
    assert!(text_display.uses_default_text_background());
    assert_eq!(text_display.text_alignment(), 2);
    assert_eq!(interaction.interaction_width(), 1.5);
    assert_eq!(interaction.interaction_height(), 2.5);
    assert!(interaction.has_interaction_response());
}

#[test]
fn generic_entity_object_metadata_api_matches_minestom_other_meta_surface() {
    let mut area_effect_cloud = GenericEntity::new(EntityType::AREA_EFFECT_CLOUD);
    let mut fishing_hook = GenericEntity::new(EntityType::FISHING_BOBBER);
    let owner_entity_id = EntityId::next();
    let mut boat = GenericEntity::new(EntityType::OAK_BOAT);
    let mut minecart = GenericEntity::new(EntityType::MINECART);
    let mut furnace_minecart = GenericEntity::new(EntityType::FURNACE_MINECART);
    let mut command_block_minecart = GenericEntity::new(EntityType::COMMAND_BLOCK_MINECART);
    let mut end_crystal = GenericEntity::new(EntityType::END_CRYSTAL);
    let falling_block_type = EntityType::from_key("minecraft:falling_block").unwrap();
    let mut falling_block = GenericEntity::new(falling_block_type);
    let mut item_frame = GenericEntity::new(EntityType::ITEM_FRAME);
    let mut painting = GenericEntity::new(EntityType::PAINTING);
    let mut primed_tnt = GenericEntity::new(EntityType::TNT);
    let mut ominous_item_spawner = GenericEntity::new(EntityType::OMINOUS_ITEM_SPAWNER);
    let particle = Particle::effect();
    let position = Position { x: 1, y: 64, z: 2 };
    let slot = Slot::from_item_stack(&ItemStack::air());
    let output = spinel_utils::component::text::TextComponent::literal("output");

    assert_eq!(area_effect_cloud.area_effect_cloud_radius(), 0.5);
    assert!(!area_effect_cloud.is_area_effect_cloud_waiting());
    area_effect_cloud.set_area_effect_cloud_radius(3.5);
    area_effect_cloud.set_area_effect_cloud_waiting(true);
    area_effect_cloud.set_area_effect_cloud_particle(particle.clone());

    fishing_hook.set_hooked_entity_id(7);
    fishing_hook.set_fishing_hook_catchable(true);
    fishing_hook.set_fishing_hook_owner_entity_id(Some(owner_entity_id));
    boat.set_left_paddle_turning(true);
    boat.set_right_paddle_turning(true);
    boat.set_splash_timer(9);
    minecart.set_custom_minecart_block_state(10);
    minecart.set_custom_minecart_block_y_position(11);
    furnace_minecart.set_furnace_minecart_fuel(true);
    command_block_minecart.set_command_block_minecart_command("say hi".to_string());
    command_block_minecart.set_command_block_minecart_last_output(output.clone());
    end_crystal.set_end_crystal_beam_target(Some(position));
    end_crystal.set_end_crystal_showing_bottom(false);
    falling_block.set_falling_block_state(15);
    item_frame.set_hanging_direction(2);
    item_frame.set_item_frame_item(slot.clone());
    item_frame.set_item_frame_rotation(3);
    painting.set_painting_variant(4);
    primed_tnt.set_primed_tnt_fuse_time(5);
    primed_tnt.set_primed_tnt_block_state(6);
    ominous_item_spawner.set_ominous_item_spawner_item(slot.clone());

    assert_eq!(area_effect_cloud.area_effect_cloud_radius(), 3.5);
    assert!(area_effect_cloud.is_area_effect_cloud_waiting());
    assert_eq!(area_effect_cloud.area_effect_cloud_particle(), particle);
    assert_eq!(fishing_hook.hooked_entity_id(), 7);
    assert!(fishing_hook.is_fishing_hook_catchable());
    assert_eq!(
        fishing_hook.fishing_hook_owner_entity_id(),
        Some(owner_entity_id)
    );
    assert_eq!(fishing_hook.spawn_packet().data, owner_entity_id.value());
    assert!(boat.is_left_paddle_turning());
    assert!(boat.is_right_paddle_turning());
    assert_eq!(boat.splash_timer(), 9);
    assert_eq!(minecart.custom_minecart_block_state(), 10);
    assert_eq!(minecart.custom_minecart_block_y_position(), 11);
    assert!(furnace_minecart.has_furnace_minecart_fuel());
    assert_eq!(
        command_block_minecart.command_block_minecart_command(),
        "say hi"
    );
    assert_eq!(
        command_block_minecart.command_block_minecart_last_output(),
        output
    );
    assert_eq!(end_crystal.end_crystal_beam_target(), Some(position));
    assert!(!end_crystal.is_end_crystal_showing_bottom());
    assert_eq!(falling_block.falling_block_state(), 15);
    assert_eq!(falling_block.spawn_packet().data, 15);
    assert_eq!(item_frame.hanging_direction(), 2);
    assert_eq!(item_frame.spawn_packet().data, 2);
    assert_eq!(item_frame.item_frame_item(), slot);
    assert_eq!(item_frame.item_frame_rotation(), 3);
    assert_eq!(painting.painting_variant(), 4);
    assert_eq!(primed_tnt.primed_tnt_fuse_time(), 5);
    assert_eq!(primed_tnt.primed_tnt_block_state(), 6);
    assert_eq!(ominous_item_spawner.ominous_item_spawner_item(), slot);
}

#[test]
fn generic_entity_object_data_provider_velocity_rules_match_minestom() {
    let llama_spit_type = EntityType::from_key("minecraft:llama_spit").unwrap();
    let shulker_bullet_type = EntityType::from_key("minecraft:shulker_bullet").unwrap();
    let mut zombie = GenericEntity::new(EntityType::ZOMBIE);
    let mut llama_spit = GenericEntity::new(llama_spit_type);
    let mut shulker_bullet = GenericEntity::new(shulker_bullet_type);
    let velocity = Velocity(Vector3d {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    });

    zombie.set_velocity(velocity);
    llama_spit.set_velocity(velocity);
    shulker_bullet.set_velocity(velocity);

    assert_eq!(
        zombie.spawn_packet().velocity,
        Velocity(Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0
        })
    );
    let protocol_velocity = Velocity(Vector3d {
        x: 0.05,
        y: 0.1,
        z: 0.15,
    });
    assert_eq!(llama_spit.spawn_packet().velocity, protocol_velocity);
    assert_eq!(shulker_bullet.spawn_packet().velocity, protocol_velocity);
    assert_eq!(zombie.velocity_packet().velocity, protocol_velocity);
}

#[test]
fn generic_entity_armor_stand_metadata_api_matches_minestom_meta_surface() {
    let mut armor_stand = GenericEntity::new(EntityType::ARMOR_STAND);
    let rotation = Vector3f {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    assert!(!armor_stand.is_armor_stand_small());
    assert!(!armor_stand.has_armor_stand_arms());
    assert!(!armor_stand.has_armor_stand_no_base_plate());
    assert!(!armor_stand.is_armor_stand_marker());
    assert_eq!(
        armor_stand.armor_stand_left_arm_rotation(),
        Vector3f {
            x: -10.0,
            y: 0.0,
            z: -10.0
        }
    );

    armor_stand.set_armor_stand_small(true);
    armor_stand.set_armor_stand_arms(true);
    armor_stand.set_armor_stand_no_base_plate(true);
    armor_stand.set_armor_stand_marker(true);
    armor_stand.set_armor_stand_head_rotation(rotation);
    armor_stand.set_armor_stand_body_rotation(rotation);
    armor_stand.set_armor_stand_left_arm_rotation(rotation);
    armor_stand.set_armor_stand_right_arm_rotation(rotation);
    armor_stand.set_armor_stand_left_leg_rotation(rotation);
    armor_stand.set_armor_stand_right_leg_rotation(rotation);

    assert!(armor_stand.is_armor_stand_small());
    assert!(armor_stand.has_armor_stand_arms());
    assert!(armor_stand.has_armor_stand_no_base_plate());
    assert!(armor_stand.is_armor_stand_marker());
    assert_eq!(armor_stand.armor_stand_head_rotation(), rotation);
    assert_eq!(armor_stand.armor_stand_body_rotation(), rotation);
    assert_eq!(armor_stand.armor_stand_left_arm_rotation(), rotation);
    assert_eq!(armor_stand.armor_stand_right_arm_rotation(), rotation);
    assert_eq!(armor_stand.armor_stand_left_leg_rotation(), rotation);
    assert_eq!(armor_stand.armor_stand_right_leg_rotation(), rotation);
}

#[test]
fn generic_entity_slime_metadata_api_matches_minestom_meta_surface() {
    let mut slime = GenericEntity::new(EntityType::SLIME);

    assert_eq!(slime.slime_size(), 1);

    slime.set_slime_size(4);

    assert_eq!(slime.slime_size(), 4);
    assert_eq!(
        slime.bounding_box().width(),
        f64::from(0.51000005_f32 * 4.0)
    );
    assert_eq!(
        slime.bounding_box().height(),
        f64::from(0.51000005_f32 * 4.0)
    );
    assert_eq!(
        slime.bounding_box().depth(),
        f64::from(0.51000005_f32 * 4.0)
    );
}

#[test]
fn generic_entity_ender_dragon_metadata_api_matches_minestom_meta_surface() {
    let mut ender_dragon = GenericEntity::new(EntityType::ENDER_DRAGON);

    assert_eq!(ender_dragon.ender_dragon_phase(), 10);

    ender_dragon.set_ender_dragon_phase(7);

    assert_eq!(ender_dragon.ender_dragon_phase(), 7);
}

#[test]
fn generic_entity_bat_and_bee_metadata_api_matches_minestom_meta_surface() {
    let mut bat = GenericEntity::new(EntityType::BAT);
    let mut bee = GenericEntity::new(EntityType::BEE);

    assert!(!bat.is_hanging_bat());
    assert!(!bee.is_angry_bee());
    assert!(!bee.has_bee_stung());
    assert!(!bee.bee_has_nectar());
    assert_eq!(bee.bee_anger_ticks(), -1);

    bat.set_hanging_bat(true);
    bee.set_angry_bee(true);
    bee.set_bee_has_stung(true);
    bee.set_bee_has_nectar(true);
    bee.set_bee_anger_ticks(45);

    assert!(bat.is_hanging_bat());
    assert!(bee.is_angry_bee());
    assert!(bee.has_bee_stung());
    assert!(bee.bee_has_nectar());
    assert_eq!(bee.bee_anger_ticks(), 45);
}

#[test]
fn generic_entity_allay_armadillo_and_sniffer_metadata_match_minestom() {
    let mut allay = GenericEntity::new(EntityType::ALLAY);
    let mut armadillo = GenericEntity::new(EntityType::ARMADILLO);
    let mut sniffer = GenericEntity::new(EntityType::SNIFFER);

    assert!(!allay.is_dancing_allay());
    assert!(allay.allay_can_duplicate());
    assert_eq!(armadillo.armadillo_state(), ArmadilloState::Idle);
    assert_eq!(sniffer.sniffer_state(), SnifferState::Idling);
    assert_eq!(sniffer.sniffer_drop_seed_at_tick(), 0);

    allay.set_dancing_allay(true);
    allay.set_allay_can_duplicate(false);
    armadillo.set_armadillo_state(ArmadilloState::Scared);
    sniffer.set_sniffer_state(SnifferState::Digging);
    sniffer.set_sniffer_drop_seed_at_tick(42);

    assert!(allay.is_dancing_allay());
    assert!(!allay.allay_can_duplicate());
    assert_eq!(armadillo.armadillo_state(), ArmadilloState::Scared);
    assert_eq!(sniffer.sniffer_state(), SnifferState::Digging);
    assert_eq!(sniffer.sniffer_drop_seed_at_tick(), 42);
}

#[test]
fn generic_entity_blaze_creeper_and_golem_metadata_match_minestom() {
    let mut blaze = GenericEntity::new(EntityType::BLAZE);
    let mut creeper = GenericEntity::new(EntityType::CREEPER);
    let mut iron_golem = GenericEntity::new(EntityType::IRON_GOLEM);
    let mut snow_golem = GenericEntity::new(EntityType::SNOW_GOLEM);

    assert!(!blaze.is_blaze_on_fire());
    assert_eq!(creeper.creeper_state(), CreeperState::Idle);
    assert!(!creeper.is_charged_creeper());
    assert!(!creeper.is_ignited_creeper());
    assert!(!iron_golem.is_player_created_iron_golem());
    assert!(snow_golem.snow_golem_has_pumpkin_hat());

    blaze.set_blaze_on_fire(true);
    creeper.set_creeper_state(CreeperState::Fuse);
    creeper.set_charged_creeper(true);
    creeper.set_ignited_creeper(true);
    iron_golem.set_player_created_iron_golem(true);
    snow_golem.set_snow_golem_has_pumpkin_hat(false);

    assert!(blaze.is_blaze_on_fire());
    assert_eq!(creeper.creeper_state(), CreeperState::Fuse);
    assert!(creeper.is_charged_creeper());
    assert!(creeper.is_ignited_creeper());
    assert!(iron_golem.is_player_created_iron_golem());
    assert!(!snow_golem.snow_golem_has_pumpkin_hat());
}

#[test]
fn generic_entity_copper_golem_shulker_and_spellcaster_metadata_match_minestom() {
    let mut copper_golem = GenericEntity::new(EntityType::COPPER_GOLEM);
    let mut shulker = GenericEntity::new(EntityType::SHULKER);
    let mut evoker = GenericEntity::new(EntityType::EVOKER);

    assert_eq!(
        copper_golem.copper_golem_weather_state(),
        CopperGolemWeatherState::Unaffected
    );
    assert_eq!(copper_golem.copper_golem_state(), CopperGolemState::Idle);
    assert_eq!(
        shulker.shulker_attach_face(),
        spinel_registry::BlockFaceDirection::Down
    );
    assert_eq!(shulker.shulker_shield_height(), 0);
    assert_eq!(shulker.shulker_color(), DyeColor::Purple);
    assert_eq!(
        evoker.spellcaster_illager_spell(),
        SpellcasterIllagerSpell::None
    );

    copper_golem.set_copper_golem_weather_state(CopperGolemWeatherState::Oxidized);
    copper_golem.set_copper_golem_state(CopperGolemState::DroppingNoItem);
    shulker.set_shulker_attach_face(spinel_registry::BlockFaceDirection::North);
    shulker.set_shulker_shield_height(100);
    shulker.set_component(
        spinel_registry::data_components::vanilla_components::SHULKER_COLOR,
        DyeColor::Blue,
    );
    evoker.set_spellcaster_illager_spell(SpellcasterIllagerSpell::SummonVex);

    assert_eq!(
        copper_golem.copper_golem_weather_state(),
        CopperGolemWeatherState::Oxidized
    );
    assert_eq!(
        copper_golem.copper_golem_state(),
        CopperGolemState::DroppingNoItem
    );
    assert_eq!(
        shulker.shulker_attach_face(),
        spinel_registry::BlockFaceDirection::North
    );
    assert_eq!(shulker.shulker_shield_height(), 100);
    assert_eq!(shulker.shulker_color(), DyeColor::Blue);
    assert_eq!(
        shulker.component(spinel_registry::data_components::vanilla_components::SHULKER_COLOR),
        Some(DyeColor::Blue)
    );
    assert_eq!(
        evoker.spellcaster_illager_spell(),
        SpellcasterIllagerSpell::SummonVex
    );
}

#[test]
fn generic_entity_bogged_creaking_spider_and_vex_metadata_match_minestom() {
    let mut bogged = GenericEntity::new(EntityType::BOGGED);
    let mut creaking = GenericEntity::new(EntityType::CREAKING);
    let mut spider = GenericEntity::new(EntityType::SPIDER);
    let mut vex = GenericEntity::new(EntityType::VEX);
    let home_position = Position { x: 8, y: 9, z: 10 };

    assert!(!bogged.is_sheared_bogged());
    assert!(creaking.creaking_can_move());
    assert!(!creaking.is_active_creaking());
    assert!(!creaking.is_tearing_down_creaking());
    assert_eq!(creaking.creaking_home_position(), None);
    assert!(!spider.is_climbing_spider());
    assert!(!vex.is_attacking_vex());

    bogged.set_sheared_bogged(true);
    creaking.set_creaking_can_move(false);
    creaking.set_active_creaking(true);
    creaking.set_tearing_down_creaking(true);
    creaking.set_creaking_home_position(Some(home_position));
    spider.set_climbing_spider(true);
    vex.set_attacking_vex(true);

    assert!(bogged.is_sheared_bogged());
    assert!(!creaking.creaking_can_move());
    assert!(creaking.is_active_creaking());
    assert!(creaking.is_tearing_down_creaking());
    assert_eq!(creaking.creaking_home_position(), Some(home_position));
    assert!(spider.is_climbing_spider());
    assert!(vex.is_attacking_vex());
}

#[test]
fn generic_entity_guardian_raider_wither_warden_and_zombie_metadata_match_minestom() {
    let mut guardian = GenericEntity::new(EntityType::GUARDIAN);
    let mut raider = GenericEntity::new(EntityType::RAVAGER);
    let mut pillager = GenericEntity::new(EntityType::PILLAGER);
    let mut witch = GenericEntity::new(EntityType::WITCH);
    let mut warden = GenericEntity::new(EntityType::WARDEN);
    let mut wither = GenericEntity::new(EntityType::WITHER);
    let mut zoglin = GenericEntity::new(EntityType::ZOGLIN);
    let mut zombie = GenericEntity::new(EntityType::ZOMBIE);

    assert!(!guardian.guardian_is_retracting_spikes());
    assert_eq!(guardian.guardian_target_entity_id(), 0);
    assert!(!raider.raider_is_celebrating());
    assert!(!pillager.pillager_is_charging_crossbow());
    assert!(!witch.witch_is_drinking_potion());
    assert_eq!(warden.warden_anger_level(), 0);
    assert_eq!(wither.wither_center_head_entity_id(), 0);
    assert_eq!(wither.wither_left_head_entity_id(), 0);
    assert_eq!(wither.wither_right_head_entity_id(), 0);
    assert_eq!(wither.wither_invulnerable_time(), 0);
    assert!(!zoglin.is_baby_zoglin());
    assert!(!zombie.is_baby_zombie());
    assert!(!zombie.zombie_is_becoming_drowned());

    guardian.set_guardian_retracting_spikes(true);
    guardian.set_guardian_target_entity_id(15);
    raider.set_raider_celebrating(true);
    pillager.set_pillager_charging_crossbow(true);
    witch.set_witch_drinking_potion(true);
    warden.set_warden_anger_level(80);
    wither.set_wither_center_head_entity_id(1);
    wither.set_wither_left_head_entity_id(2);
    wither.set_wither_right_head_entity_id(3);
    wither.set_wither_invulnerable_time(20);
    zoglin.set_baby_zoglin(true);
    zombie.set_baby_zombie(true);
    zombie.set_zombie_becoming_drowned(true);

    assert!(guardian.guardian_is_retracting_spikes());
    assert_eq!(guardian.guardian_target_entity_id(), 15);
    assert!(raider.raider_is_celebrating());
    assert!(pillager.pillager_is_charging_crossbow());
    assert!(witch.witch_is_drinking_potion());
    assert_eq!(warden.warden_anger_level(), 80);
    assert_eq!(wither.wither_center_head_entity_id(), 1);
    assert_eq!(wither.wither_left_head_entity_id(), 2);
    assert_eq!(wither.wither_right_head_entity_id(), 3);
    assert_eq!(wither.wither_invulnerable_time(), 20);
    assert!(zoglin.is_baby_zoglin());
    assert!(zombie.is_baby_zombie());
    assert!(zombie.zombie_is_becoming_drowned());
}

#[test]
fn generic_entity_piglin_enderman_ghast_and_phantom_metadata_match_minestom() {
    let mut piglin = GenericEntity::new(EntityType::PIGLIN);
    let mut enderman = GenericEntity::new(EntityType::ENDERMAN);
    let mut ghast = GenericEntity::new(EntityType::GHAST);
    let mut phantom = GenericEntity::new(EntityType::PHANTOM);
    let stone = spinel_registry::vanilla_world_blocks::Block::STONE.default_state();
    let air = spinel_registry::vanilla_world_blocks::Block::AIR.default_state();

    assert!(!piglin.piglin_is_immune_to_zombification());
    assert!(!piglin.is_baby_piglin());
    assert!(!piglin.piglin_is_charging_crossbow());
    assert!(!piglin.is_dancing_piglin());
    assert_eq!(enderman.enderman_carried_block(), None);
    assert!(!enderman.is_screaming_enderman());
    assert!(!enderman.is_staring_enderman());
    assert!(!ghast.is_attacking_ghast());
    assert_eq!(phantom.phantom_size(), 0);

    piglin.set_piglin_immune_to_zombification(true);
    piglin.set_baby_piglin(true);
    piglin.set_piglin_charging_crossbow(true);
    piglin.set_dancing_piglin(true);
    enderman.set_enderman_carried_block(Some(stone));
    enderman.set_screaming_enderman(true);
    enderman.set_staring_enderman(true);
    ghast.set_attacking_ghast(true);
    phantom.set_phantom_size(4);

    assert!(piglin.piglin_is_immune_to_zombification());
    assert!(piglin.is_baby_piglin());
    assert!(piglin.piglin_is_charging_crossbow());
    assert!(piglin.is_dancing_piglin());
    assert_eq!(enderman.enderman_carried_block(), Some(stone));
    assert!(enderman.is_screaming_enderman());
    assert!(enderman.is_staring_enderman());
    assert!(ghast.is_attacking_ghast());
    assert_eq!(phantom.phantom_size(), 4);

    enderman.set_enderman_carried_block(Some(air));
    assert_eq!(enderman.enderman_carried_block(), None);
}

#[test]
fn generic_entity_villager_scalar_metadata_matches_minestom() {
    let mut villager = GenericEntity::new(EntityType::VILLAGER);
    let mut zombie_villager = GenericEntity::new(EntityType::ZOMBIE_VILLAGER);
    let librarian = VillagerData::DEFAULT
        .with_type(VillagerType::PLAINS)
        .with_profession(VillagerProfession::LIBRARIAN)
        .with_level(VillagerLevel::Master);

    assert_eq!(villager.villager_head_shake_timer(), 0);
    assert_eq!(villager.villager_data(), VillagerData::DEFAULT);
    assert_eq!(
        villager.component(spinel_registry::data_components::vanilla_components::VILLAGER_VARIANT),
        Some(VillagerType::DESERT)
    );
    assert!(!zombie_villager.zombie_villager_is_converting());
    assert_eq!(
        zombie_villager.zombie_villager_data(),
        VillagerData::DEFAULT
    );

    villager.set_villager_head_shake_timer(12);
    villager.set_villager_data(librarian.clone());
    villager.set_component(
        spinel_registry::data_components::vanilla_components::VILLAGER_VARIANT,
        VillagerType::TAIGA,
    );
    zombie_villager.set_zombie_villager_converting(true);
    zombie_villager.set_zombie_villager_data(librarian.clone());
    zombie_villager.set_component(
        spinel_registry::data_components::vanilla_components::VILLAGER_VARIANT,
        VillagerType::SWAMP,
    );

    assert_eq!(villager.villager_head_shake_timer(), 12);
    assert_eq!(
        villager.villager_data(),
        librarian.with_type(VillagerType::TAIGA)
    );
    assert!(zombie_villager.zombie_villager_is_converting());
    assert_eq!(
        zombie_villager.zombie_villager_data(),
        librarian.with_type(VillagerType::SWAMP)
    );
}

#[test]
fn generic_entity_dolphin_and_goat_metadata_api_matches_minestom_meta_surface() {
    let mut dolphin = GenericEntity::new(EntityType::DOLPHIN);
    let mut goat = GenericEntity::new(EntityType::GOAT);
    let treasure_position = spinel_network::types::Position { x: 4, y: 5, z: 6 };

    assert_eq!(
        dolphin.dolphin_treasure_position(),
        spinel_network::types::Position { x: 0, y: 0, z: 0 }
    );
    assert!(!dolphin.dolphin_has_fish());
    assert_eq!(dolphin.dolphin_moisture_level(), 2400);
    assert!(!goat.is_screaming_goat());
    assert!(goat.goat_has_left_horn());
    assert!(goat.goat_has_right_horn());

    dolphin.set_dolphin_treasure_position(treasure_position);
    dolphin.set_dolphin_has_fish(true);
    dolphin.set_dolphin_moisture_level(1200);
    goat.set_screaming_goat(true);
    goat.set_goat_has_left_horn(false);
    goat.set_goat_has_right_horn(false);

    assert_eq!(dolphin.dolphin_treasure_position(), treasure_position);
    assert!(dolphin.dolphin_has_fish());
    assert_eq!(dolphin.dolphin_moisture_level(), 1200);
    assert!(goat.is_screaming_goat());
    assert!(!goat.goat_has_left_horn());
    assert!(!goat.goat_has_right_horn());
}

#[test]
fn generic_entity_axolotl_metadata_api_matches_minestom_meta_surface() {
    let mut axolotl = GenericEntity::new(EntityType::AXOLOTL);

    assert_eq!(axolotl.axolotl_variant(), AxolotlVariant::Lucy);
    assert!(!axolotl.is_playing_dead_axolotl());
    assert!(!axolotl.is_from_bucket_axolotl());

    axolotl.set_axolotl_variant(AxolotlVariant::Blue);
    axolotl.set_playing_dead_axolotl(true);
    axolotl.set_from_bucket_axolotl(true);

    assert_eq!(axolotl.axolotl_variant(), AxolotlVariant::Blue);
    assert!(axolotl.is_playing_dead_axolotl());
    assert!(axolotl.is_from_bucket_axolotl());
}

#[test]
fn generic_entity_pig_and_sheep_metadata_api_matches_minestom_meta_surface() {
    let mut pig = GenericEntity::new(EntityType::PIG);
    let mut sheep = GenericEntity::new(EntityType::SHEEP);

    assert_eq!(pig.pig_boost_time(), 0);
    assert_eq!(sheep.sheep_color(), DyeColor::White);
    assert!(!sheep.is_sheared_sheep());

    pig.set_pig_boost_time(80);
    sheep.set_sheep_color(DyeColor::Blue);
    sheep.set_sheared_sheep(true);

    assert_eq!(pig.pig_boost_time(), 80);
    assert_eq!(sheep.sheep_color(), DyeColor::Blue);
    assert!(sheep.is_sheared_sheep());
}

#[test]
fn generic_entity_remaining_animal_scalar_metadata_matches_minestom() {
    let mut horse = GenericEntity::new(EntityType::HORSE);
    let mut camel = GenericEntity::new(EntityType::CAMEL);
    let mut donkey = GenericEntity::new(EntityType::DONKEY);
    let mut ocelot = GenericEntity::new(EntityType::OCELOT);
    let mut turtle = GenericEntity::new(EntityType::TURTLE);
    let mut polar_bear = GenericEntity::new(EntityType::POLAR_BEAR);
    let mut hoglin = GenericEntity::new(EntityType::HOGLIN);
    let mut strider = GenericEntity::new(EntityType::STRIDER);
    let mut wolf = GenericEntity::new(EntityType::WOLF);
    let mut nautilus = GenericEntity::new(EntityType::NAUTILUS);
    let mut happy_ghast = GenericEntity::new(EntityType::HAPPY_GHAST);
    let owner = uuid::Uuid::new_v4();

    horse.set_tamed_horse(true);
    horse.set_horse_has_bred(true);
    horse.set_eating_horse(true);
    horse.set_rearing_horse(true);
    horse.set_horse_mouth_open(true);
    camel.set_dashing_camel(true);
    camel.set_camel_last_pose_change_tick(42);
    donkey.set_chested_horse_has_chest(true);
    ocelot.set_trusting_ocelot(true);
    turtle.set_turtle_has_egg(true);
    turtle.set_laying_egg_turtle(true);
    polar_bear.set_standing_polar_bear(true);
    hoglin.set_hoglin_immune_to_zombification(true);
    strider.set_strider_boost_time(80);
    strider.set_shaking_strider(true);
    wolf.set_sitting_tameable_animal(true);
    wolf.set_tamed_animal(true);
    wolf.set_tameable_animal_owner(Some(owner));
    nautilus.set_dashing_nautilus(true);
    happy_ghast.set_happy_ghast_leash_holder(true);
    happy_ghast.set_happy_ghast_stays_still(true);

    assert!(horse.is_tamed_horse());
    assert!(horse.horse_has_bred());
    assert!(horse.is_eating_horse());
    assert!(horse.is_rearing_horse());
    assert!(horse.is_horse_mouth_open());
    assert!(camel.is_dashing_camel());
    assert_eq!(camel.camel_last_pose_change_tick(), 42);
    assert!(donkey.chested_horse_has_chest());
    assert!(ocelot.is_trusting_ocelot());
    assert!(turtle.turtle_has_egg());
    assert!(turtle.is_laying_egg_turtle());
    assert!(polar_bear.is_standing_polar_bear());
    assert!(hoglin.hoglin_is_immune_to_zombification());
    assert_eq!(strider.strider_boost_time(), 80);
    assert!(strider.is_shaking_strider());
    assert!(wolf.is_sitting_tameable_animal());
    assert!(wolf.is_tamed_animal());
    assert_eq!(wolf.tameable_animal_owner(), Some(owner));
    assert!(nautilus.is_dashing_nautilus());
    assert!(happy_ghast.happy_ghast_is_leash_holder());
    assert!(happy_ghast.happy_ghast_stays_still());
}

#[test]
fn generic_entity_cat_and_wolf_scalar_components_match_minestom() {
    let mut cat = GenericEntity::new(EntityType::CAT);
    let mut wolf = GenericEntity::new(EntityType::WOLF);

    assert!(!cat.is_lying_cat());
    assert!(!cat.is_relaxed_cat());
    assert_eq!(cat.cat_collar_color(), DyeColor::Red);
    assert!(!wolf.is_begging_wolf());
    assert_eq!(wolf.wolf_collar_color(), DyeColor::Red);
    assert_eq!(wolf.wolf_anger_time(), -1);

    cat.set_lying_cat(true);
    cat.set_relaxed_cat(true);
    cat.set_component(
        spinel_registry::data_components::vanilla_components::CAT_COLLAR,
        DyeColor::Blue,
    );
    wolf.set_begging_wolf(true);
    wolf.set_component(
        spinel_registry::data_components::vanilla_components::WOLF_COLLAR,
        DyeColor::Green,
    );
    wolf.set_wolf_anger_time(120);

    assert!(cat.is_lying_cat());
    assert!(cat.is_relaxed_cat());
    assert_eq!(cat.cat_collar_color(), DyeColor::Blue);
    assert_eq!(
        cat.component(spinel_registry::data_components::vanilla_components::CAT_COLLAR),
        Some(DyeColor::Blue)
    );
    assert!(wolf.is_begging_wolf());
    assert_eq!(wolf.wolf_collar_color(), DyeColor::Green);
    assert_eq!(
        wolf.component(spinel_registry::data_components::vanilla_components::WOLF_COLLAR),
        Some(DyeColor::Green)
    );
    assert_eq!(wolf.wolf_anger_time(), 120);
}

#[test]
fn generic_entity_static_animal_variants_and_genes_match_minestom() {
    let mut horse = GenericEntity::new(EntityType::HORSE);
    let mut llama = GenericEntity::new(EntityType::LLAMA);
    let mut fox = GenericEntity::new(EntityType::FOX);
    let mut panda = GenericEntity::new(EntityType::PANDA);
    let mut rabbit = GenericEntity::new(EntityType::RABBIT);
    let mut mooshroom = GenericEntity::new(EntityType::MOOSHROOM);
    let mut parrot = GenericEntity::new(EntityType::PARROT);
    let trusted_fox = uuid::Uuid::new_v4();
    let second_trusted_fox = uuid::Uuid::new_v4();
    let horse_variant = HorseVariant::new(HorseMarking::WhiteDots, HorseColor::DarkBrown);

    horse.set_horse_variant(horse_variant);
    llama.set_llama_strength(5);
    llama.set_llama_carpet_color(11);
    llama.set_llama_variant(LlamaVariant::Gray);
    fox.set_fox_variant(FoxVariant::Snow);
    fox.set_sitting_fox(true);
    fox.set_sneaking_fox(true);
    fox.set_interested_fox(true);
    fox.set_pouncing_fox(true);
    fox.set_sleeping_fox(true);
    fox.set_faceplanted_fox(true);
    fox.set_defending_fox(true);
    fox.set_fox_first_uuid(Some(trusted_fox));
    fox.set_fox_second_uuid(Some(second_trusted_fox));
    panda.set_panda_breed_timer(1);
    panda.set_panda_sneeze_timer(2);
    panda.set_panda_eat_timer(3);
    panda.set_panda_main_gene(PandaGene::Brown);
    panda.set_panda_hidden_gene(PandaGene::Weak);
    panda.set_sneezing_panda(true);
    panda.set_rolling_panda(true);
    panda.set_sitting_panda(true);
    panda.set_on_back_panda(true);
    rabbit.set_rabbit_variant(RabbitVariant::KillerBunny);
    mooshroom.set_mooshroom_variant(MooshroomVariant::Brown);
    parrot.set_parrot_color(ParrotColor::YellowBlue);

    assert_eq!(horse.horse_variant(), horse_variant);
    assert_eq!(horse_variant.protocol_id(), (3 << 8) + 6);
    assert_eq!(llama.llama_strength(), 5);
    assert_eq!(llama.llama_carpet_color(), 11);
    assert_eq!(llama.llama_variant(), LlamaVariant::Gray);
    assert_eq!(fox.fox_variant(), FoxVariant::Snow);
    assert!(fox.is_sitting_fox());
    assert!(fox.is_sneaking_fox());
    assert!(fox.is_interested_fox());
    assert!(fox.is_pouncing_fox());
    assert!(fox.is_sleeping_fox());
    assert!(fox.is_faceplanted_fox());
    assert!(fox.is_defending_fox());
    assert_eq!(fox.fox_first_uuid(), Some(trusted_fox));
    assert_eq!(fox.fox_second_uuid(), Some(second_trusted_fox));
    assert_eq!(panda.panda_breed_timer(), 1);
    assert_eq!(panda.panda_sneeze_timer(), 2);
    assert_eq!(panda.panda_eat_timer(), 3);
    assert_eq!(panda.panda_main_gene(), PandaGene::Brown);
    assert_eq!(panda.panda_hidden_gene(), PandaGene::Weak);
    assert!(panda.is_sneezing_panda());
    assert!(panda.is_rolling_panda());
    assert!(panda.is_sitting_panda());
    assert!(panda.is_on_back_panda());
    assert_eq!(rabbit.rabbit_variant(), RabbitVariant::KillerBunny);
    assert_eq!(RabbitVariant::KillerBunny.protocol_id(), 99);
    assert_eq!(mooshroom.mooshroom_variant(), MooshroomVariant::Brown);
    assert_eq!(parrot.parrot_color(), ParrotColor::YellowBlue);
}

#[test]
fn generic_entity_static_variant_components_bridge_owned_metadata() {
    use spinel_registry::data_components::vanilla_components::{
        AXOLOTL_VARIANT, FOX_VARIANT, HORSE_VARIANT, LLAMA_VARIANT, MOOSHROOM_VARIANT,
        PARROT_VARIANT, RABBIT_VARIANT, SALMON_SIZE, TROPICAL_FISH_BASE_COLOR,
        TROPICAL_FISH_PATTERN, TROPICAL_FISH_PATTERN_COLOR,
    };

    let mut axolotl = GenericEntity::new(EntityType::AXOLOTL);
    let mut fox = GenericEntity::new(EntityType::FOX);
    let mut horse = GenericEntity::new(EntityType::HORSE);
    let mut llama = GenericEntity::new(EntityType::LLAMA);
    let mut trader_llama = GenericEntity::new(EntityType::TRADER_LLAMA);
    let mut mooshroom = GenericEntity::new(EntityType::MOOSHROOM);
    let mut parrot = GenericEntity::new(EntityType::PARROT);
    let mut rabbit = GenericEntity::new(EntityType::RABBIT);
    let mut salmon = GenericEntity::new(EntityType::SALMON);
    let mut tropical_fish = GenericEntity::new(EntityType::TROPICAL_FISH);

    horse.set_horse_variant(HorseVariant::new(
        HorseMarking::BlackDots,
        HorseColor::White,
    ));
    tropical_fish.set_tropical_fish_variant(TropicalFishVariant::new(
        TropicalFishPattern::Kob,
        DyeColor::White,
        DyeColor::Black,
    ));

    axolotl.set_component(AXOLOTL_VARIANT, AxolotlVariant::Blue);
    fox.set_component(FOX_VARIANT, FoxVariant::Snow);
    horse.set_component(HORSE_VARIANT, HorseColor::DarkBrown);
    llama.set_component(LLAMA_VARIANT, LlamaVariant::Gray);
    trader_llama.set_component(LLAMA_VARIANT, LlamaVariant::Brown);
    mooshroom.set_component(MOOSHROOM_VARIANT, MooshroomVariant::Brown);
    parrot.set_component(PARROT_VARIANT, ParrotColor::Green);
    rabbit.set_component(RABBIT_VARIANT, RabbitVariant::KillerBunny);
    salmon.set_component(SALMON_SIZE, SalmonSize::Large);
    tropical_fish.set_component(TROPICAL_FISH_PATTERN, TropicalFishPattern::Betty);
    tropical_fish.set_component(TROPICAL_FISH_BASE_COLOR, DyeColor::Blue);
    tropical_fish.set_component(TROPICAL_FISH_PATTERN_COLOR, DyeColor::Red);

    assert_eq!(
        axolotl.component(AXOLOTL_VARIANT),
        Some(AxolotlVariant::Blue)
    );
    assert_eq!(fox.component(FOX_VARIANT), Some(FoxVariant::Snow));
    assert_eq!(horse.component(HORSE_VARIANT), Some(HorseColor::DarkBrown));
    assert_eq!(horse.horse_variant().marking(), HorseMarking::BlackDots);
    assert_eq!(llama.component(LLAMA_VARIANT), Some(LlamaVariant::Gray));
    assert_eq!(
        trader_llama.component(LLAMA_VARIANT),
        Some(LlamaVariant::Brown)
    );
    assert_eq!(
        mooshroom.component(MOOSHROOM_VARIANT),
        Some(MooshroomVariant::Brown)
    );
    assert_eq!(parrot.component(PARROT_VARIANT), Some(ParrotColor::Green));
    assert_eq!(
        rabbit.component(RABBIT_VARIANT),
        Some(RabbitVariant::KillerBunny)
    );
    assert_eq!(salmon.component(SALMON_SIZE), Some(SalmonSize::Large));
    assert_eq!(
        tropical_fish.component(TROPICAL_FISH_PATTERN),
        Some(TropicalFishPattern::Betty)
    );
    assert_eq!(
        tropical_fish.component(TROPICAL_FISH_BASE_COLOR),
        Some(DyeColor::Blue)
    );
    assert_eq!(
        tropical_fish.component(TROPICAL_FISH_PATTERN_COLOR),
        Some(DyeColor::Red)
    );
}

#[test]
fn generic_entity_fish_metadata_api_matches_minestom_meta_surface() {
    let mut pufferfish = GenericEntity::new(EntityType::PUFFERFISH);
    let mut salmon = GenericEntity::new(EntityType::SALMON);
    let mut tropical_fish = GenericEntity::new(EntityType::TROPICAL_FISH);
    let tropical_variant =
        TropicalFishVariant::new(TropicalFishPattern::Betty, DyeColor::Blue, DyeColor::Red);

    assert!(!pufferfish.is_fish_from_bucket());
    assert_eq!(pufferfish.pufferfish_state(), PufferfishState::Unpuffed);
    assert_eq!(salmon.salmon_size(), SalmonSize::Small);
    assert_eq!(
        tropical_fish.tropical_fish_variant(),
        TropicalFishVariant::DEFAULT
    );

    pufferfish.set_fish_from_bucket(true);
    pufferfish.set_pufferfish_state(PufferfishState::FullyPuffed);
    salmon.set_salmon_size(SalmonSize::Large);
    tropical_fish.set_tropical_fish_variant(tropical_variant);

    assert!(pufferfish.is_fish_from_bucket());
    assert_eq!(pufferfish.pufferfish_state(), PufferfishState::FullyPuffed);
    assert_eq!(pufferfish.bounding_box().width(), 0.7);
    assert_eq!(salmon.salmon_size(), SalmonSize::Large);
    assert_eq!(salmon.salmon_size().id(), "large");
    assert_eq!(tropical_fish.tropical_fish_variant(), tropical_variant);
}

#[test]
fn generic_entity_snapshot_updater_receives_mutable_snapshot_copy() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let position = EntityPosition::new(3.0, 64.0, 9.0, 45.0, 10.0);
    entity.teleport(position);

    let snapshot = entity.update_snapshot(|snapshot| {
        assert_eq!(snapshot.entity_id(), entity.entity_id());
        assert_eq!(snapshot.entity_type(), EntityType::ZOMBIE);
        assert_eq!(snapshot.position(), position);
    });

    assert_eq!(snapshot.uuid(), entity.uuid());
    assert_eq!(snapshot.position(), position);
}

#[test]
fn generic_entity_event_node_pose_kill_viewer_packets_and_occlusion_match_minestom_surface() {
    static POSE_EVENT_ENTITY_ID: AtomicI32 = AtomicI32::new(0);
    let (mut client, _peer_stream) = test_client();
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity
        .attribute(1, 20.0)
        .add_modifier(EntityAttributeModifier::attack_speed(
            spinel_network::types::Identifier::minecraft("test_modifier"),
            1.0,
        ));

    entity.event_node().listen("EntityPoseEvent", |entity_id| {
        POSE_EVENT_ENTITY_ID.store(entity_id.value(), Ordering::SeqCst);
    });
    entity.set_pose(2);

    assert_eq!(
        POSE_EVENT_ENTITY_ID.load(Ordering::SeqCst),
        entity.entity_id().value()
    );
    assert_eq!(entity.event_node().listener_count("EntityPoseEvent"), 1);
    assert!(entity.update_new_viewer(&mut client).is_ok());
    assert!(
        client
            .queued_outbound_packet_ids()
            .contains(&SetEquipmentPacket::get_id())
    );
    assert!(
        client
            .queued_outbound_packet_ids()
            .contains(&UpdateAttributesPacket::get_id())
    );
    assert!(entity.add_leashed_entity(EntityId::from_raw(44)));
    assert!(entity.update_old_viewer(&mut client).is_ok());
    assert!(
        client
            .queued_outbound_packet_ids()
            .contains(&RemoveEntitiesPacket::get_id())
    );
    assert!(!entity.is_occluded());
    assert!(entity.kill());
    assert!(entity.is_dead());
    assert_eq!(entity.pose(), 6);
}

fn test_client() -> (Client, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let client_stream = TcpStream::connect(addr).unwrap();
    let (peer_stream, _) = listener.accept().unwrap();
    (Client::new(client_stream, addr), peer_stream)
}
