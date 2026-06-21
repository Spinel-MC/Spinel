use crate::entity::metadata::{
    ArmadilloState, AxolotlVariant, CopperGolemState, CopperGolemWeatherState, CreeperState,
    EnderDragonPhase, FoxVariant, HorseColor, HorseMarking, HorseVariant, LlamaVariant,
    MooshroomVariant, PandaGene, ParrotColor, PufferfishState, RabbitVariant, SalmonSize,
    SnifferState, SpellcasterIllagerSpell, TropicalFishPattern, TropicalFishVariant, VillagerData,
    VillagerLevel,
};
use crate::entity::{EntityId, EntityPose, EntityPosition, GenericEntity, PlayerHand};
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
    MainHand, Particle, Position, ResolvableProfile, TeleportFlags, Vector3d, Vector3f, Velocity,
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
    assert_eq!(plain_teleport.get_position(), position);
    assert_eq!(entity.get_position(), position);

    let velocity_teleport = entity.teleport_with_velocity(position, velocity);
    assert_eq!(velocity_teleport.get_velocity(), velocity);
    assert_eq!(entity.get_velocity(), velocity);

    let chunk_teleport = entity.teleport_with_chunks_and_flags(
        position,
        Some(vec![1, 2, 3]),
        TeleportFlags::absolute(),
    );
    assert_eq!(chunk_teleport.get_chunks(), Some([1, 2, 3].as_slice()));
    assert_eq!(chunk_teleport.get_flags().bitmask(), TeleportFlags::DELTA_COORD);

    let full_teleport = entity.teleport_with_velocity_chunks_and_flags(
        EntityPosition::new(4.0, 70.0, 5.0, 180.0, 0.0),
        velocity,
        Some(vec![4]),
        TeleportFlags::absolute(),
    );
    assert_eq!(full_teleport.get_position().get_x(), 4.0);
    assert_eq!(full_teleport.get_chunks(), Some([4].as_slice()));
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
        preserved_velocity.get_velocity(),
        Velocity(Vector3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        })
    );
    assert_eq!(
        preserved_velocity.get_flags().bitmask(),
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
        teleport.get_position(),
        EntityPosition::new(6.0, 70.0, 3.0, 22.0, 25.0)
    );
    assert_eq!(
        teleport.get_velocity(),
        Velocity(Vector3d {
            x: 1.5,
            y: 8.0,
            z: 2.5,
        })
    );
    assert_eq!(
        teleport.get_teleport_position(),
        EntityPosition::new(2.0, 70.0, -3.0, 15.0, 25.0)
    );
    assert_eq!(
        teleport.get_teleport_velocity(),
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
    assert_eq!(entity.get_position().get_x(), 1.0);

    entity.refresh_position_ignoring_view(EntityPosition::new(6.0, 7.0, 8.0, 9.0, 10.0), true);
    assert_eq!(entity.get_position().get_x(), 6.0);
    assert_eq!(entity.get_position().get_x(), 1.0);
}

#[test]
fn generic_entity_from_cient_swing_overloads_preserve_hand_animation() {
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
    assert_eq!(hover_entity.id, entity.get_uuid().to_string());
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
    assert_eq!(entity.get_air_ticks(), 300);
    assert_eq!(entity.get_ticks_frozen(), 0);

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
    assert_eq!(entity.get_air_ticks(), 42);
    assert_eq!(entity.get_ticks_frozen(), 7);
    assert!(
        entity
            .get_metadata()
            .get_entries()
            .iter()
            .any(|entry| entry.index == 1 && entry.value == MetadataValue::VarInt(42))
    );
    assert!(
        entity
            .get_metadata()
            .get_entries()
            .iter()
            .any(|entry| entry.index == 7 && entry.value == MetadataValue::VarInt(7))
    );
}

#[test]
fn generic_entity_living_mob_and_ageable_metadata_api_matches_minestom_meta_surface() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let original_bounding_box = entity.get_bounding_box();
    let effect_particle = Particle::effect();
    let bed_position = Position { x: 1, y: 64, z: 2 };

    assert!(!entity.is_hand_active());
    assert_eq!(entity.get_active_hand(), PlayerHand::Main);
    assert!(!entity.is_in_riptide_spin_attack());
    assert!(entity.get_effect_particles().is_empty());
    assert!(!entity.is_potion_effect_ambient());
    assert_eq!(entity.get_arrow_count(), 0);
    assert_eq!(entity.get_bee_stinger_count(), 0);
    assert_eq!(entity.get_bed_in_which_sleeping_position(), None);
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
    assert_eq!(entity.get_active_hand(), PlayerHand::Off);
    assert!(entity.is_in_riptide_spin_attack());
    assert_eq!(entity.get_effect_particles(), vec![effect_particle]);
    assert!(entity.is_potion_effect_ambient());
    assert_eq!(entity.get_arrow_count(), 3);
    assert_eq!(entity.get_bee_stinger_count(), 4);
    assert_eq!(entity.get_bed_in_which_sleeping_position(), Some(bed_position));
    assert!(entity.is_no_ai());
    assert!(entity.is_left_handed());
    assert!(entity.is_aggressive());
    assert!(entity.is_baby());
    assert_eq!(
        entity.get_bounding_box().get_width(),
        original_bounding_box.get_width() / 2.0
    );
    assert_eq!(
        entity.get_bounding_box().get_height(),
        original_bounding_box.get_height() / 2.0
    );
    assert!(
        entity
            .get_metadata()
            .get_entries()
            .iter()
            .any(|entry| entry.index == 9 && entry.value == MetadataValue::Float(13.5))
    );
    assert!(
        entity
            .get_metadata()
            .get_entries()
            .iter()
            .any(|entry| entry.index == 12 && entry.value == MetadataValue::VarInt(3))
    );

    entity.set_baby(false);

    assert!(!entity.is_baby());
    assert_eq!(entity.get_bounding_box(), original_bounding_box);
}

#[test]
fn generic_entity_vehicle_avatar_player_and_mannequin_metadata_api_matches_minestom_meta_surface() {
    let mut entity = GenericEntity::new(EntityType::PLAYER);
    let mut player_meta = entity.get_entity_meta_mut().as_player().unwrap();
    let mut mannequin = GenericEntity::new(EntityType::MANNEQUIN);
    let description = spinel_utils::component::text::TextComponent::literal("description");
    let profile = ResolvableProfile::default();

    assert_eq!(player_meta.get_main_hand(), MainHand::Right);
    assert_eq!(player_meta.get_displayed_skin_parts(), 0);
    assert_eq!(player_meta.get_additional_hearts(), 0.0);
    assert_eq!(player_meta.get_score(), 0);
    assert_eq!(player_meta.get_left_shoulder_entity_data(), None);
    assert_eq!(player_meta.get_right_shoulder_entity_data(), None);
    assert_eq!(mannequin.get_profile(), profile);
    assert!(!mannequin.is_immovable());

    player_meta.set_main_hand(MainHand::Left);
    player_meta.set_cape_enabled(true);
    player_meta.set_jacket_enabled(true);
    player_meta.set_left_sleeve_enabled(true);
    player_meta.set_right_sleeve_enabled(true);
    player_meta.set_left_leg_enabled(true);
    player_meta.set_right_leg_enabled(true);
    player_meta.set_hat_enabled(true);
    player_meta.set_displayed_skin_parts(0x7f);
    player_meta.set_additional_hearts(2.5);
    player_meta.set_score(12);
    player_meta.set_left_shoulder_entity_data(Some(1));
    player_meta.set_right_shoulder_entity_data(Some(2));
    mannequin.set_profile(profile.clone());
    mannequin.set_immovable(true);
    mannequin.set_description(Some(description.clone()));

    assert_eq!(player_meta.get_main_hand(), MainHand::Left);
    assert!(player_meta.is_cape_enabled());
    assert!(player_meta.is_jacket_enabled());
    assert!(player_meta.is_left_sleeve_enabled());
    assert!(player_meta.is_right_sleeve_enabled());
    assert!(player_meta.is_left_leg_enabled());
    assert!(player_meta.is_right_leg_enabled());
    assert!(player_meta.is_hat_enabled());
    assert_eq!(player_meta.get_displayed_skin_parts(), 0x7f);
    assert_eq!(player_meta.get_additional_hearts(), 2.5);
    assert_eq!(player_meta.get_score(), 12);
    assert_eq!(player_meta.get_left_shoulder_entity_data(), Some(1));
    assert_eq!(player_meta.get_right_shoulder_entity_data(), Some(2));
    assert_eq!(mannequin.get_profile(), profile);
    assert!(mannequin.is_immovable());
    assert_eq!(mannequin.get_description(), Some(description));
}

#[test]
fn typed_display_metadata_api_matches_minestom_display_meta_surface() {
    use crate::entity::metadata::{BillboardConstraints, DisplayContext, TextAlignment};

    let mut block_display = GenericEntity::new(EntityType::BLOCK_DISPLAY);
    let mut item_display = GenericEntity::new(EntityType::ITEM_DISPLAY);
    let mut text_display = GenericEntity::new(EntityType::TEXT_DISPLAY);
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
    let displayed_item = ItemStack::air();
    let block_state =
        spinel_registry::BlockState::from_state_id(12).expect("test block state must exist");
    let text = spinel_utils::component::text::TextComponent::literal("display");

    {
        let mut meta = block_display
            .get_entity_meta_mut()
            .as_block_display()
            .expect("block display must expose BlockDisplayMeta");
        meta.set_transformation_interpolation_start_delta(1);
        meta.set_transformation_interpolation_duration(2);
        meta.set_position_rotation_interpolation_duration(3);
        meta.set_translation(translation);
        meta.set_scale(scale);
        meta.set_left_rotation(rotation);
        meta.set_right_rotation(rotation);
        meta.set_billboard_render_constraints(BillboardConstraints::Center);
        meta.set_brightness(4, 5);
        meta.set_view_range(6.0);
        meta.set_shadow_radius(7.0);
        meta.set_shadow_strength(8.0);
        meta.set_width(9.0);
        meta.set_height(10.0);
        meta.set_glow_color_override(11);
        meta.set_block_state(block_state);
    }
    {
        let mut meta = item_display
            .get_entity_meta_mut()
            .as_item_display()
            .expect("item display must expose ItemDisplayMeta");
        meta.set_item_stack(displayed_item.clone());
        meta.set_display_context(DisplayContext::Gui);
    }
    {
        let mut meta = text_display
            .get_entity_meta_mut()
            .as_text_display()
            .expect("text display must expose TextDisplayMeta");
        meta.set_text(text.clone());
        meta.set_line_width(13);
        meta.set_background_color(14);
        meta.set_text_opacity(15);
        meta.set_shadow(true);
        meta.set_see_through(true);
        meta.set_uses_default_background(true);
        meta.set_alignment(TextAlignment::Right);
    }

    let block_meta = block_display
        .get_entity_meta_mut()
        .as_block_display()
        .expect("block display must expose BlockDisplayMeta");
    assert_eq!(block_meta.get_transformation_interpolation_start_delta(), 1);
    assert_eq!(block_meta.get_transformation_interpolation_duration(), 2);
    assert_eq!(block_meta.get_position_rotation_interpolation_duration(), 3);
    assert_eq!(block_meta.get_translation(), translation);
    assert_eq!(block_meta.get_scale(), scale);
    assert_eq!(block_meta.get_left_rotation(), rotation);
    assert_eq!(block_meta.get_right_rotation(), rotation);
    assert_eq!(
        block_meta.get_billboard_render_constraints(),
        BillboardConstraints::Center
    );
    assert_eq!(block_meta.get_block_light(), 4);
    assert_eq!(block_meta.get_sky_light(), 5);
    assert_eq!(block_meta.get_view_range(), 6.0);
    assert_eq!(block_meta.get_shadow_radius(), 7.0);
    assert_eq!(block_meta.get_shadow_strength(), 8.0);
    assert_eq!(block_meta.get_width(), 9.0);
    assert_eq!(block_meta.get_height(), 10.0);
    assert_eq!(block_meta.get_glow_color_override(), 11);
    assert_eq!(block_meta.get_block_state(), block_state);

    let item_meta = item_display
        .get_entity_meta_mut()
        .as_item_display()
        .expect("item display must expose ItemDisplayMeta");
    assert_eq!(item_meta.get_item_stack(), displayed_item);
    assert_eq!(item_meta.get_display_context(), DisplayContext::Gui);

    let text_meta = text_display
        .get_entity_meta_mut()
        .as_text_display()
        .expect("text display must expose TextDisplayMeta");
    assert_eq!(text_meta.get_text(), text);
    assert_eq!(text_meta.get_line_width(), 13);
    assert_eq!(text_meta.get_background_color(), 14);
    assert_eq!(text_meta.get_text_opacity(), 15);
    assert!(text_meta.is_shadow());
    assert!(text_meta.is_see_through());
    assert!(text_meta.has_default_background());
    assert_eq!(text_meta.get_alignment(), TextAlignment::Right);
}
#[test]
fn typed_object_metadata_api_matches_minestom_other_meta_surface() {
    let mut area_effect_cloud = GenericEntity::new(EntityType::AREA_EFFECT_CLOUD);
    let mut fishing_hook = GenericEntity::new(EntityType::FISHING_BOBBER);
    let owner_entity_id = EntityId::next();
    let hooked_entity_id = EntityId::from_raw(6);
    let mut boat = GenericEntity::new(EntityType::OAK_BOAT);
    let mut minecart = GenericEntity::new(EntityType::MINECART);
    let mut furnace_minecart = GenericEntity::new(EntityType::FURNACE_MINECART);
    let mut command_block_minecart = GenericEntity::new(EntityType::COMMAND_BLOCK_MINECART);
    let mut end_crystal = GenericEntity::new(EntityType::END_CRYSTAL);
    let mut falling_block = GenericEntity::new(EntityType::FALLING_BLOCK);
    let mut item_frame = GenericEntity::new(EntityType::ITEM_FRAME);
    let mut painting = GenericEntity::new(EntityType::PAINTING);
    let mut primed_tnt = GenericEntity::new(EntityType::TNT);
    let mut ominous_item_spawner = GenericEntity::new(EntityType::OMINOUS_ITEM_SPAWNER);
    let particle = Particle::effect();
    let position = Position { x: 1, y: 64, z: 2 };
    let item_stack = ItemStack::air();
    let output = spinel_utils::component::text::TextComponent::literal("output");
    let custom_minecart_block_state =
        spinel_registry::BlockState::from_state_id(10).expect("test block state must exist");
    let falling_block_state =
        spinel_registry::BlockState::from_state_id(15).expect("test block state must exist");
    let primed_tnt_block_state =
        spinel_registry::BlockState::from_state_id(6).expect("test block state must exist");

    {
        let mut area_effect_cloud_meta = area_effect_cloud
            .get_entity_meta_mut()
            .as_area_effect_cloud()
            .expect("area effect cloud entity must expose AreaEffectCloudMeta");
        assert_eq!(area_effect_cloud_meta.get_radius(), 0.5);
        assert!(!area_effect_cloud_meta.is_waiting());
        area_effect_cloud_meta.set_radius(3.5);
        area_effect_cloud_meta.set_waiting(true);
        area_effect_cloud_meta.set_particle(particle.clone());
    }
    {
        let mut fishing_hook_meta = fishing_hook
            .get_entity_meta_mut()
            .as_fishing_hook()
            .expect("fishing bobber entity must expose FishingHookMeta");
        fishing_hook_meta.set_hooked_entity(Some(hooked_entity_id));
        fishing_hook_meta.set_catchable(true);
        fishing_hook_meta.set_owner_entity(Some(owner_entity_id));
    }
    boat.get_entity_meta_mut()
        .as_boat()
        .expect("boat entity must expose BoatMeta")
        .set_left_paddle_turning(true);
    boat.get_entity_meta_mut()
        .as_boat()
        .expect("boat entity must expose BoatMeta")
        .set_right_paddle_turning(true);
    boat.get_entity_meta_mut()
        .as_boat()
        .expect("boat entity must expose BoatMeta")
        .set_splash_timer(9);
    minecart
        .get_entity_meta_mut()
        .as_minecart()
        .expect("minecart entity must expose MinecartMeta")
        .set_custom_block_state(Some(custom_minecart_block_state));
    minecart
        .get_entity_meta_mut()
        .as_minecart()
        .expect("minecart entity must expose MinecartMeta")
        .set_custom_block_y_position(11);
    furnace_minecart
        .get_entity_meta_mut()
        .as_furnace_minecart()
        .expect("furnace minecart entity must expose FurnaceMinecartMeta")
        .set_has_fuel(true);
    command_block_minecart
        .get_entity_meta_mut()
        .as_command_block_minecart()
        .expect("command block minecart entity must expose CommandBlockMinecartMeta")
        .set_command("say hi".to_string());
    command_block_minecart
        .get_entity_meta_mut()
        .as_command_block_minecart()
        .expect("command block minecart entity must expose CommandBlockMinecartMeta")
        .set_last_output(output.clone());
    end_crystal
        .get_entity_meta_mut()
        .as_end_crystal()
        .expect("end crystal entity must expose EndCrystalMeta")
        .set_beam_target(Some(position));
    end_crystal
        .get_entity_meta_mut()
        .as_end_crystal()
        .expect("end crystal entity must expose EndCrystalMeta")
        .set_showing_bottom(false);
    falling_block
        .get_entity_meta_mut()
        .as_falling_block()
        .expect("falling block entity must expose FallingBlockMeta")
        .set_block(falling_block_state);
    {
        let mut item_frame_meta = item_frame
            .get_entity_meta_mut()
            .as_item_frame()
            .expect("item frame entity must expose ItemFrameMeta");
        item_frame_meta.set_direction(2);
        item_frame_meta.set_item(item_stack.clone());
        item_frame_meta.set_rotation(3);
    }
    painting
        .get_entity_meta_mut()
        .as_painting()
        .expect("painting entity must expose PaintingMeta")
        .set_variant(4);
    {
        let mut primed_tnt_meta = primed_tnt
            .get_entity_meta_mut()
            .as_primed_tnt()
            .expect("tnt entity must expose PrimedTntMeta");
        primed_tnt_meta.set_fuse_time(5);
        primed_tnt_meta.set_block_state(primed_tnt_block_state);
    }
    ominous_item_spawner
        .get_entity_meta_mut()
        .as_ominous_item_spawner()
        .expect("ominous item spawner entity must expose OminousItemSpawnerMeta")
        .set_item(item_stack.clone());

    let area_effect_cloud_meta = area_effect_cloud
        .get_entity_meta_mut()
        .as_area_effect_cloud()
        .expect("area effect cloud entity must expose AreaEffectCloudMeta");
    assert_eq!(area_effect_cloud_meta.get_radius(), 3.5);
    assert!(area_effect_cloud_meta.is_waiting());
    assert_eq!(area_effect_cloud_meta.get_particle(), particle);
    let fishing_hook_meta = fishing_hook
        .get_entity_meta_mut()
        .as_fishing_hook()
        .expect("fishing bobber entity must expose FishingHookMeta");
    assert_eq!(fishing_hook_meta.get_hooked_entity_id(), 7);
    assert!(fishing_hook_meta.is_catchable());
    assert_eq!(fishing_hook_meta.get_owner_entity_id(), Some(owner_entity_id));
    assert_eq!(fishing_hook.spawn_packet().data, owner_entity_id.get_value());
    assert!(
        boat.get_entity_meta_mut()
            .as_boat()
            .expect("boat entity must expose BoatMeta")
            .is_left_paddle_turning()
    );
    assert!(
        boat.get_entity_meta_mut()
            .as_boat()
            .expect("boat entity must expose BoatMeta")
            .is_right_paddle_turning()
    );
    assert_eq!(
        boat.get_entity_meta_mut()
            .as_boat()
            .expect("boat entity must expose BoatMeta")
            .get_splash_timer(),
        9
    );
    assert_eq!(
        minecart
            .get_entity_meta_mut()
            .as_minecart()
            .expect("minecart entity must expose MinecartMeta")
            .get_custom_block_state(),
        Some(custom_minecart_block_state)
    );
    assert_eq!(
        minecart
            .get_entity_meta_mut()
            .as_minecart()
            .expect("minecart entity must expose MinecartMeta")
            .get_custom_block_y_position(),
        11
    );
    assert!(
        furnace_minecart
            .get_entity_meta_mut()
            .as_furnace_minecart()
            .expect("furnace minecart entity must expose FurnaceMinecartMeta")
            .has_fuel()
    );
    assert_eq!(
        command_block_minecart
            .get_entity_meta_mut()
            .as_command_block_minecart()
            .expect("command block minecart entity must expose CommandBlockMinecartMeta")
            .get_command(),
        "say hi"
    );
    assert_eq!(
        command_block_minecart
            .get_entity_meta_mut()
            .as_command_block_minecart()
            .expect("command block minecart entity must expose CommandBlockMinecartMeta")
            .get_last_output(),
        output
    );
    let end_crystal_meta = end_crystal
        .get_entity_meta_mut()
        .as_end_crystal()
        .expect("end crystal entity must expose EndCrystalMeta");
    assert_eq!(end_crystal_meta.get_beam_target(), Some(position));
    assert!(!end_crystal_meta.is_showing_bottom());
    assert_eq!(
        falling_block
            .get_entity_meta_mut()
            .as_falling_block()
            .expect("falling block entity must expose FallingBlockMeta")
            .get_block(),
        falling_block_state
    );
    assert_eq!(falling_block.spawn_packet().data, 15);
    let (item_frame_direction, item_frame_item, item_frame_rotation) = {
        let item_frame_meta = item_frame
            .get_entity_meta_mut()
            .as_item_frame()
            .expect("item frame entity must expose ItemFrameMeta");
        (
            item_frame_meta.get_direction(),
            item_frame_meta.get_item(),
            item_frame_meta.get_rotation(),
        )
    };
    assert_eq!(item_frame_direction, 2);
    assert_eq!(item_frame.spawn_packet().data, 2);
    assert_eq!(item_frame_item, item_stack);
    assert_eq!(item_frame_rotation, 3);
    assert_eq!(
        painting
            .get_entity_meta_mut()
            .as_painting()
            .expect("painting entity must expose PaintingMeta")
            .get_variant(),
        4
    );
    let primed_tnt_meta = primed_tnt
        .get_entity_meta_mut()
        .as_primed_tnt()
        .expect("tnt entity must expose PrimedTntMeta");
    assert_eq!(primed_tnt_meta.get_fuse_time(), 5);
    assert_eq!(primed_tnt_meta.get_block_state(), primed_tnt_block_state);
    assert_eq!(
        ominous_item_spawner
            .get_entity_meta_mut()
            .as_ominous_item_spawner()
            .expect("ominous item spawner entity must expose OminousItemSpawnerMeta")
            .get_item(),
        item_stack
    );
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
    assert_eq!(zombie.get_velocity_packet().velocity, protocol_velocity);
}

#[test]
fn typed_armor_stand_metadata_api_matches_minestom_meta_surface() {
    let mut armor_stand = GenericEntity::new(EntityType::ARMOR_STAND);
    let rotation = Vector3f {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    {
        let armor_stand_meta = armor_stand
            .get_entity_meta_mut()
            .as_armor_stand()
            .expect("armor stand entity must expose ArmorStandMeta");
        assert!(!armor_stand_meta.is_small());
        assert!(!armor_stand_meta.has_arms());
        assert!(!armor_stand_meta.has_no_base_plate());
        assert!(!armor_stand_meta.is_marker());
        assert_eq!(
            armor_stand_meta.get_left_arm_rotation(),
            Vector3f {
                x: -10.0,
                y: 0.0,
                z: -10.0
            }
        );
    }

    {
        let mut armor_stand_meta = armor_stand
            .get_entity_meta_mut()
            .as_armor_stand()
            .expect("armor stand entity must expose ArmorStandMeta");
        armor_stand_meta.set_small(true);
        armor_stand_meta.set_has_arms(true);
        armor_stand_meta.set_has_no_base_plate(true);
        armor_stand_meta.set_marker(true);
        armor_stand_meta.set_head_rotation(rotation);
        armor_stand_meta.set_body_rotation(rotation);
        armor_stand_meta.set_left_arm_rotation(rotation);
        armor_stand_meta.set_right_arm_rotation(rotation);
        armor_stand_meta.set_left_leg_rotation(rotation);
        armor_stand_meta.set_right_leg_rotation(rotation);
    }

    let armor_stand_meta = armor_stand
        .get_entity_meta_mut()
        .as_armor_stand()
        .expect("armor stand entity must expose ArmorStandMeta");
    assert!(armor_stand_meta.is_small());
    assert!(armor_stand_meta.has_arms());
    assert!(armor_stand_meta.has_no_base_plate());
    assert!(armor_stand_meta.is_marker());
    assert_eq!(armor_stand_meta.get_head_rotation(), rotation);
    assert_eq!(armor_stand_meta.get_body_rotation(), rotation);
    assert_eq!(armor_stand_meta.get_left_arm_rotation(), rotation);
    assert_eq!(armor_stand_meta.get_right_arm_rotation(), rotation);
    assert_eq!(armor_stand_meta.get_left_leg_rotation(), rotation);
    assert_eq!(armor_stand_meta.get_right_leg_rotation(), rotation);
}
#[test]
fn typed_slime_metadata_api_matches_minestom_meta_surface() {
    let mut slime = GenericEntity::new(EntityType::SLIME);
    let mut magma_cube = GenericEntity::new(EntityType::MAGMA_CUBE);

    assert_eq!(
        slime
            .get_entity_meta_mut()
            .as_slime()
            .expect("slime entity must expose SlimeMeta")
            .get_size(),
        1
    );

    slime
        .get_entity_meta_mut()
        .as_slime()
        .expect("slime entity must expose SlimeMeta")
        .set_size(4);
    magma_cube
        .get_entity_meta_mut()
        .as_magma_cube()
        .expect("magma cube entity must expose MagmaCubeMeta")
        .set_size(3);

    assert_eq!(
        slime
            .get_entity_meta_mut()
            .as_slime()
            .expect("slime entity must expose SlimeMeta")
            .get_size(),
        4
    );
    assert_eq!(
        slime.get_bounding_box().get_width(),
        f64::from(0.51000005_f32 * 4.0)
    );
    assert_eq!(
        slime.get_bounding_box().get_height(),
        f64::from(0.51000005_f32 * 4.0)
    );
    assert_eq!(
        slime.get_bounding_box().depth(),
        f64::from(0.51000005_f32 * 4.0)
    );
    assert_eq!(
        magma_cube
            .get_entity_meta_mut()
            .as_magma_cube()
            .expect("magma cube entity must expose MagmaCubeMeta")
            .get_size(),
        3
    );
}
#[test]
fn typed_ender_dragon_metadata_api_matches_minestom_meta_surface() {
    let mut ender_dragon = GenericEntity::new(EntityType::ENDER_DRAGON);

    assert_eq!(
        ender_dragon
            .get_entity_meta_mut()
            .as_ender_dragon()
            .expect("ender dragon entity must expose EnderDragonMeta")
            .get_phase(),
        EnderDragonPhase::HoveringWithoutAi
    );

    ender_dragon
        .get_entity_meta_mut()
        .as_ender_dragon()
        .expect("ender dragon entity must expose EnderDragonMeta")
        .set_phase(EnderDragonPhase::Roar);

    assert_eq!(
        ender_dragon
            .get_entity_meta_mut()
            .as_ender_dragon()
            .expect("ender dragon entity must expose EnderDragonMeta")
            .get_phase(),
        EnderDragonPhase::Roar
    );
}

#[test]
fn bat_and_bee_metadata_owners_match_minestom_meta_surface() {
    let mut bat = GenericEntity::new(EntityType::BAT);
    let mut bee = GenericEntity::new(EntityType::BEE);
    let mut zombie = GenericEntity::new(EntityType::ZOMBIE);

    {
        let mut bat_meta = bat
            .get_entity_meta_mut()
            .as_bat()
            .expect("bat entity must expose BatMeta");
        assert!(!bat_meta.is_hanging());
        bat_meta.set_hanging(true);
        assert!(bat_meta.is_hanging());
    }
    assert!(zombie.get_entity_meta_mut().as_bat().is_none());
    assert!(zombie.get_entity_meta_mut().as_bee().is_none());
    {
        let mut bee_meta = bee
            .get_entity_meta_mut()
            .as_bee()
            .expect("bee entity must expose BeeMeta");
        assert!(!bee_meta.is_angry());
        assert!(!bee_meta.has_stung());
        assert!(!bee_meta.has_nectar());
        assert_eq!(bee_meta.get_anger_ticks(), -1);
        bee_meta.set_angry(true);
        bee_meta.set_has_stung(true);
        bee_meta.set_has_nectar(true);
        bee_meta.set_anger_ticks(45);
        bee_meta.set_baby(true);
        assert!(bee_meta.is_angry());
        assert!(bee_meta.has_stung());
        assert!(bee_meta.has_nectar());
        assert_eq!(bee_meta.get_anger_ticks(), 45);
        assert!(bee_meta.is_baby());
    }
    assert_eq!(
        bee.get_bounding_box().get_width(),
        EntityType::BEE.get_bounding_box().get_width() / 2.0
    );
}

#[test]
fn allay_armadillo_and_sniffer_metadata_owners_match_minestom() {
    let mut allay = GenericEntity::new(EntityType::ALLAY);
    let mut armadillo = GenericEntity::new(EntityType::ARMADILLO);
    let mut sniffer = GenericEntity::new(EntityType::SNIFFER);

    {
        let mut allay_meta = allay
            .get_entity_meta_mut()
            .as_allay()
            .expect("allay entity must expose AllayMeta");
        assert!(!allay_meta.is_dancing());
        assert!(allay_meta.can_duplicate());
        allay_meta.set_dancing(true);
        allay_meta.set_can_duplicate(false);
        assert!(allay_meta.is_dancing());
        assert!(!allay_meta.can_duplicate());
    }
    {
        let armadillo_meta = armadillo
            .get_entity_meta_mut()
            .as_armadillo()
            .expect("armadillo entity must expose ArmadilloMeta");
        assert_eq!(armadillo_meta.get_state(), ArmadilloState::Idle);
    }
    armadillo
        .get_entity_meta_mut()
        .as_armadillo()
        .expect("armadillo entity must expose ArmadilloMeta")
        .set_state(ArmadilloState::Scared);
    assert_eq!(
        armadillo
            .get_entity_meta_mut()
            .as_armadillo()
            .expect("armadillo entity must expose ArmadilloMeta")
            .get_state(),
        ArmadilloState::Scared
    );
    {
        let mut sniffer_meta = sniffer
            .get_entity_meta_mut()
            .as_sniffer()
            .expect("sniffer entity must expose SnifferMeta");
        assert_eq!(sniffer_meta.get_state(), SnifferState::Idling);
        assert_eq!(sniffer_meta.get_drop_seed_at_tick(), 0);
        sniffer_meta.set_state(SnifferState::Digging);
        sniffer_meta.set_drop_seed_at_tick(42);
        assert_eq!(sniffer_meta.get_state(), SnifferState::Digging);
        assert_eq!(sniffer_meta.get_drop_seed_at_tick(), 42);
    }
}

#[test]
fn generic_entity_blaze_creeper_and_golem_metadata_match_minestom() {
    let mut blaze = GenericEntity::new(EntityType::BLAZE);
    let mut creeper = GenericEntity::new(EntityType::CREEPER);
    let mut iron_golem = GenericEntity::new(EntityType::IRON_GOLEM);
    let mut snow_golem = GenericEntity::new(EntityType::SNOW_GOLEM);

    assert!(
        !blaze
            .get_entity_meta_mut()
            .as_blaze()
            .expect("blaze entity must expose BlazeMeta")
            .is_on_fire()
    );
    {
        let creeper_meta = creeper
            .get_entity_meta_mut()
            .as_creeper()
            .expect("creeper entity must expose CreeperMeta");
        assert_eq!(creeper_meta.get_state(), CreeperState::Idle);
        assert!(!creeper_meta.is_charged());
        assert!(!creeper_meta.is_ignited());
    }
    assert!(
        !iron_golem
            .get_entity_meta_mut()
            .as_iron_golem()
            .expect("iron golem entity must expose IronGolemMeta")
            .is_player_created()
    );
    assert!(
        snow_golem
            .get_entity_meta_mut()
            .as_snow_golem()
            .expect("snow golem entity must expose SnowGolemMeta")
            .has_pumpkin_hat()
    );

    blaze
        .get_entity_meta_mut()
        .as_blaze()
        .expect("blaze entity must expose BlazeMeta")
        .set_on_fire(true);
    {
        let mut creeper_meta = creeper
            .get_entity_meta_mut()
            .as_creeper()
            .expect("creeper entity must expose CreeperMeta");
        creeper_meta.set_state(CreeperState::Fuse);
        creeper_meta.set_charged(true);
        creeper_meta.set_ignited(true);
    }
    iron_golem
        .get_entity_meta_mut()
        .as_iron_golem()
        .expect("iron golem entity must expose IronGolemMeta")
        .set_player_created(true);
    snow_golem
        .get_entity_meta_mut()
        .as_snow_golem()
        .expect("snow golem entity must expose SnowGolemMeta")
        .set_has_pumpkin_hat(false);

    assert!(
        blaze
            .get_entity_meta_mut()
            .as_blaze()
            .expect("blaze entity must expose BlazeMeta")
            .is_on_fire()
    );
    {
        let creeper_meta = creeper
            .get_entity_meta_mut()
            .as_creeper()
            .expect("creeper entity must expose CreeperMeta");
        assert_eq!(creeper_meta.get_state(), CreeperState::Fuse);
        assert!(creeper_meta.is_charged());
        assert!(creeper_meta.is_ignited());
    }
    assert!(
        iron_golem
            .get_entity_meta_mut()
            .as_iron_golem()
            .expect("iron golem entity must expose IronGolemMeta")
            .is_player_created()
    );
    assert!(
        !snow_golem
            .get_entity_meta_mut()
            .as_snow_golem()
            .expect("snow golem entity must expose SnowGolemMeta")
            .has_pumpkin_hat()
    );
}

#[test]
fn generic_entity_copper_golem_shulker_and_spellcaster_metadata_match_minestom() {
    let mut copper_golem = GenericEntity::new(EntityType::COPPER_GOLEM);
    let mut shulker = GenericEntity::new(EntityType::SHULKER);
    let mut evoker = GenericEntity::new(EntityType::EVOKER);

    assert_eq!(
        copper_golem
            .get_entity_meta_mut()
            .as_copper_golem()
            .expect("copper golem entity must expose CopperGolemMeta")
            .get_weather_state(),
        CopperGolemWeatherState::Unaffected
    );
    assert_eq!(
        copper_golem
            .get_entity_meta_mut()
            .as_copper_golem()
            .expect("copper golem entity must expose CopperGolemMeta")
            .get_state(),
        CopperGolemState::Idle
    );
    assert_eq!(
        shulker
            .get_entity_meta_mut()
            .as_shulker()
            .expect("shulker entity must expose ShulkerMeta")
            .get_attach_face(),
        spinel_registry::BlockFaceDirection::Down
    );
    assert_eq!(
        shulker
            .get_entity_meta_mut()
            .as_shulker()
            .expect("shulker entity must expose ShulkerMeta")
            .get_shield_height(),
        0
    );
    assert_eq!(
        shulker
            .get_entity_meta_mut()
            .as_shulker()
            .expect("shulker entity must expose ShulkerMeta")
            .get_color(),
        DyeColor::Purple
    );
    assert_eq!(
        evoker
            .get_entity_meta_mut()
            .as_evoker()
            .expect("evoker entity must expose EvokerMeta")
            .get_spell(),
        SpellcasterIllagerSpell::None
    );

    copper_golem
        .get_entity_meta_mut()
        .as_copper_golem()
        .expect("copper golem entity must expose CopperGolemMeta")
        .set_weather_state(CopperGolemWeatherState::Oxidized);
    copper_golem
        .get_entity_meta_mut()
        .as_copper_golem()
        .expect("copper golem entity must expose CopperGolemMeta")
        .set_state(CopperGolemState::DroppingNoItem);
    shulker
        .get_entity_meta_mut()
        .as_shulker()
        .expect("shulker entity must expose ShulkerMeta")
        .set_attach_face(spinel_registry::BlockFaceDirection::North);
    shulker
        .get_entity_meta_mut()
        .as_shulker()
        .expect("shulker entity must expose ShulkerMeta")
        .set_shield_height(100);
    shulker.set_component(
        spinel_registry::data_components::vanilla_components::SHULKER_COLOR,
        DyeColor::Blue,
    );
    evoker
        .get_entity_meta_mut()
        .as_evoker()
        .expect("evoker entity must expose EvokerMeta")
        .set_spell(SpellcasterIllagerSpell::SummonVex);

    assert_eq!(
        copper_golem
            .get_entity_meta_mut()
            .as_copper_golem()
            .expect("copper golem entity must expose CopperGolemMeta")
            .get_weather_state(),
        CopperGolemWeatherState::Oxidized
    );
    assert_eq!(
        copper_golem
            .get_entity_meta_mut()
            .as_copper_golem()
            .expect("copper golem entity must expose CopperGolemMeta")
            .get_state(),
        CopperGolemState::DroppingNoItem
    );
    assert_eq!(
        shulker
            .get_entity_meta_mut()
            .as_shulker()
            .expect("shulker entity must expose ShulkerMeta")
            .get_attach_face(),
        spinel_registry::BlockFaceDirection::North
    );
    assert_eq!(
        shulker
            .get_entity_meta_mut()
            .as_shulker()
            .expect("shulker entity must expose ShulkerMeta")
            .get_shield_height(),
        100
    );
    assert_eq!(
        shulker
            .get_entity_meta_mut()
            .as_shulker()
            .expect("shulker entity must expose ShulkerMeta")
            .get_color(),
        DyeColor::Blue
    );
    assert_eq!(
        shulker.component(spinel_registry::data_components::vanilla_components::SHULKER_COLOR),
        Some(DyeColor::Blue)
    );
    assert_eq!(
        evoker
            .get_entity_meta_mut()
            .as_evoker()
            .expect("evoker entity must expose EvokerMeta")
            .get_spell(),
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

    assert!(
        !bogged
            .get_entity_meta_mut()
            .as_bogged()
            .expect("bogged entity must expose BoggedMeta")
            .is_sheared()
    );
    {
        let creaking_meta = creaking
            .get_entity_meta_mut()
            .as_creaking()
            .expect("creaking entity must expose CreakingMeta");
        assert!(creaking_meta.can_move());
        assert!(!creaking_meta.is_active());
        assert!(!creaking_meta.is_tearing_down());
        assert_eq!(creaking_meta.get_home_position(), None);
    }
    assert!(
        !spider
            .get_entity_meta_mut()
            .as_spider()
            .expect("spider entity must expose SpiderMeta")
            .is_climbing()
    );
    assert!(
        !vex.get_entity_meta_mut()
            .as_vex()
            .expect("vex entity must expose VexMeta")
            .is_attacking()
    );

    bogged
        .get_entity_meta_mut()
        .as_bogged()
        .expect("bogged entity must expose BoggedMeta")
        .set_sheared(true);
    {
        let mut creaking_meta = creaking
            .get_entity_meta_mut()
            .as_creaking()
            .expect("creaking entity must expose CreakingMeta");
        creaking_meta.set_can_move(false);
        creaking_meta.set_active(true);
        creaking_meta.set_tearing_down(true);
        creaking_meta.set_home_position(Some(home_position));
    }
    spider
        .get_entity_meta_mut()
        .as_spider()
        .expect("spider entity must expose SpiderMeta")
        .set_climbing(true);
    vex.get_entity_meta_mut()
        .as_vex()
        .expect("vex entity must expose VexMeta")
        .set_attacking(true);

    assert!(
        bogged
            .get_entity_meta_mut()
            .as_bogged()
            .expect("bogged entity must expose BoggedMeta")
            .is_sheared()
    );
    {
        let creaking_meta = creaking
            .get_entity_meta_mut()
            .as_creaking()
            .expect("creaking entity must expose CreakingMeta");
        assert!(!creaking_meta.can_move());
        assert!(creaking_meta.is_active());
        assert!(creaking_meta.is_tearing_down());
        assert_eq!(creaking_meta.get_home_position(), Some(home_position));
    }
    assert!(
        spider
            .get_entity_meta_mut()
            .as_spider()
            .expect("spider entity must expose SpiderMeta")
            .is_climbing()
    );
    assert!(
        vex.get_entity_meta_mut()
            .as_vex()
            .expect("vex entity must expose VexMeta")
            .is_attacking()
    );
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

    {
        let guardian_meta = guardian
            .get_entity_meta_mut()
            .as_guardian()
            .expect("guardian entity must expose GuardianMeta");
        assert!(!guardian_meta.is_retracting_spikes());
        assert_eq!(guardian_meta.get_target_entity_id(), 0);
    }
    assert!(
        !raider
            .get_entity_meta_mut()
            .as_ravager()
            .expect("ravager entity must expose RavagerMeta")
            .is_celebrating()
    );
    assert!(
        !pillager
            .get_entity_meta_mut()
            .as_pillager()
            .expect("pillager entity must expose PillagerMeta")
            .is_charging_crossbow()
    );
    assert!(
        !witch
            .get_entity_meta_mut()
            .as_witch()
            .expect("witch entity must expose WitchMeta")
            .is_drinking_potion()
    );
    assert_eq!(
        warden
            .get_entity_meta_mut()
            .as_warden()
            .expect("warden entity must expose WardenMeta")
            .get_anger_level(),
        0
    );
    assert_eq!(
        wither
            .get_entity_meta_mut()
            .as_wither()
            .expect("wither entity must expose WitherMeta")
            .get_center_head_entity_id(),
        0
    );
    assert_eq!(
        wither
            .get_entity_meta_mut()
            .as_wither()
            .expect("wither entity must expose WitherMeta")
            .get_left_head_entity_id(),
        0
    );
    assert_eq!(
        wither
            .get_entity_meta_mut()
            .as_wither()
            .expect("wither entity must expose WitherMeta")
            .get_right_head_entity_id(),
        0
    );
    assert_eq!(
        wither
            .get_entity_meta_mut()
            .as_wither()
            .expect("wither entity must expose WitherMeta")
            .get_invulnerable_time(),
        0
    );
    assert!(
        !zoglin
            .get_entity_meta_mut()
            .as_zoglin()
            .expect("zoglin entity must expose ZoglinMeta")
            .is_baby()
    );
    assert!(
        !zombie
            .get_entity_meta_mut()
            .as_zombie()
            .expect("zombie entity must expose ZombieMeta")
            .is_baby()
    );
    assert!(
        !zombie
            .get_entity_meta_mut()
            .as_zombie()
            .expect("zombie entity must expose ZombieMeta")
            .is_becoming_drowned()
    );

    {
        let mut guardian_meta = guardian
            .get_entity_meta_mut()
            .as_guardian()
            .expect("guardian entity must expose GuardianMeta");
        guardian_meta.set_retracting_spikes(true);
        guardian_meta.set_target_entity_id(15);
    }
    raider
        .get_entity_meta_mut()
        .as_ravager()
        .expect("ravager entity must expose RavagerMeta")
        .set_celebrating(true);
    pillager
        .get_entity_meta_mut()
        .as_pillager()
        .expect("pillager entity must expose PillagerMeta")
        .set_charging_crossbow(true);
    witch
        .get_entity_meta_mut()
        .as_witch()
        .expect("witch entity must expose WitchMeta")
        .set_drinking_potion(true);
    warden
        .get_entity_meta_mut()
        .as_warden()
        .expect("warden entity must expose WardenMeta")
        .set_anger_level(80);
    wither
        .get_entity_meta_mut()
        .as_wither()
        .expect("wither entity must expose WitherMeta")
        .set_center_head_entity_id(1);
    wither
        .get_entity_meta_mut()
        .as_wither()
        .expect("wither entity must expose WitherMeta")
        .set_left_head_entity_id(2);
    wither
        .get_entity_meta_mut()
        .as_wither()
        .expect("wither entity must expose WitherMeta")
        .set_right_head_entity_id(3);
    wither
        .get_entity_meta_mut()
        .as_wither()
        .expect("wither entity must expose WitherMeta")
        .set_invulnerable_time(20);
    zoglin
        .get_entity_meta_mut()
        .as_zoglin()
        .expect("zoglin entity must expose ZoglinMeta")
        .set_baby(true);
    zombie
        .get_entity_meta_mut()
        .as_zombie()
        .expect("zombie entity must expose ZombieMeta")
        .set_baby(true);
    zombie
        .get_entity_meta_mut()
        .as_zombie()
        .expect("zombie entity must expose ZombieMeta")
        .set_becoming_drowned(true);

    {
        let guardian_meta = guardian
            .get_entity_meta_mut()
            .as_guardian()
            .expect("guardian entity must expose GuardianMeta");
        assert!(guardian_meta.is_retracting_spikes());
        assert_eq!(guardian_meta.get_target_entity_id(), 15);
    }
    assert!(
        raider
            .get_entity_meta_mut()
            .as_ravager()
            .expect("ravager entity must expose RavagerMeta")
            .is_celebrating()
    );
    assert!(
        pillager
            .get_entity_meta_mut()
            .as_pillager()
            .expect("pillager entity must expose PillagerMeta")
            .is_charging_crossbow()
    );
    assert!(
        witch
            .get_entity_meta_mut()
            .as_witch()
            .expect("witch entity must expose WitchMeta")
            .is_drinking_potion()
    );
    assert_eq!(
        warden
            .get_entity_meta_mut()
            .as_warden()
            .expect("warden entity must expose WardenMeta")
            .get_anger_level(),
        80
    );
    assert_eq!(
        wither
            .get_entity_meta_mut()
            .as_wither()
            .expect("wither entity must expose WitherMeta")
            .get_center_head_entity_id(),
        1
    );
    assert_eq!(
        wither
            .get_entity_meta_mut()
            .as_wither()
            .expect("wither entity must expose WitherMeta")
            .get_left_head_entity_id(),
        2
    );
    assert_eq!(
        wither
            .get_entity_meta_mut()
            .as_wither()
            .expect("wither entity must expose WitherMeta")
            .get_right_head_entity_id(),
        3
    );
    assert_eq!(
        wither
            .get_entity_meta_mut()
            .as_wither()
            .expect("wither entity must expose WitherMeta")
            .get_invulnerable_time(),
        20
    );
    assert!(
        zoglin
            .get_entity_meta_mut()
            .as_zoglin()
            .expect("zoglin entity must expose ZoglinMeta")
            .is_baby()
    );
    assert!(
        zombie
            .get_entity_meta_mut()
            .as_zombie()
            .expect("zombie entity must expose ZombieMeta")
            .is_baby()
    );
    assert!(
        zombie
            .get_entity_meta_mut()
            .as_zombie()
            .expect("zombie entity must expose ZombieMeta")
            .is_becoming_drowned()
    );
}

#[test]
fn generic_entity_piglin_enderman_ghast_and_phantom_metadata_match_minestom() {
    let mut piglin = GenericEntity::new(EntityType::PIGLIN);
    let mut enderman = GenericEntity::new(EntityType::ENDERMAN);
    let mut ghast = GenericEntity::new(EntityType::GHAST);
    let mut phantom = GenericEntity::new(EntityType::PHANTOM);
    let stone = spinel_registry::vanilla_world_blocks::Block::STONE.default_state();
    let air = spinel_registry::vanilla_world_blocks::Block::AIR.default_state();

    {
        let piglin_meta = piglin
            .get_entity_meta_mut()
            .as_piglin()
            .expect("piglin entity must expose PiglinMeta");
        assert!(!piglin_meta.is_immune_to_zombification());
        assert!(!piglin_meta.is_baby());
        assert!(!piglin_meta.is_charging_crossbow());
        assert!(!piglin_meta.is_dancing());
    }
    {
        let enderman_meta = enderman
            .get_entity_meta_mut()
            .as_enderman()
            .expect("enderman entity must expose EndermanMeta");
        assert_eq!(enderman_meta.get_carried_block(), None);
        assert!(!enderman_meta.is_screaming());
        assert!(!enderman_meta.is_staring());
    }
    assert!(
        !ghast
            .get_entity_meta_mut()
            .as_ghast()
            .expect("ghast entity must expose GhastMeta")
            .is_attacking()
    );
    assert_eq!(
        phantom
            .get_entity_meta_mut()
            .as_phantom()
            .expect("phantom entity must expose PhantomMeta")
            .get_size(),
        0
    );

    {
        let mut piglin_meta = piglin
            .get_entity_meta_mut()
            .as_piglin()
            .expect("piglin entity must expose PiglinMeta");
        piglin_meta.set_immune_to_zombification(true);
        piglin_meta.set_baby(true);
        piglin_meta.set_charging_crossbow(true);
        piglin_meta.set_dancing(true);
    }
    {
        let mut enderman_meta = enderman
            .get_entity_meta_mut()
            .as_enderman()
            .expect("enderman entity must expose EndermanMeta");
        enderman_meta.set_carried_block(Some(stone));
        enderman_meta.set_screaming(true);
        enderman_meta.set_staring(true);
    }
    ghast
        .get_entity_meta_mut()
        .as_ghast()
        .expect("ghast entity must expose GhastMeta")
        .set_attacking(true);
    phantom
        .get_entity_meta_mut()
        .as_phantom()
        .expect("phantom entity must expose PhantomMeta")
        .set_size(4);

    {
        let piglin_meta = piglin
            .get_entity_meta_mut()
            .as_piglin()
            .expect("piglin entity must expose PiglinMeta");
        assert!(piglin_meta.is_immune_to_zombification());
        assert!(piglin_meta.is_baby());
        assert!(piglin_meta.is_charging_crossbow());
        assert!(piglin_meta.is_dancing());
    }
    {
        let enderman_meta = enderman
            .get_entity_meta_mut()
            .as_enderman()
            .expect("enderman entity must expose EndermanMeta");
        assert_eq!(enderman_meta.get_carried_block(), Some(stone));
        assert!(enderman_meta.is_screaming());
        assert!(enderman_meta.is_staring());
    }
    assert!(
        ghast
            .get_entity_meta_mut()
            .as_ghast()
            .expect("ghast entity must expose GhastMeta")
            .is_attacking()
    );
    assert_eq!(
        phantom
            .get_entity_meta_mut()
            .as_phantom()
            .expect("phantom entity must expose PhantomMeta")
            .get_size(),
        4
    );

    enderman
        .get_entity_meta_mut()
        .as_enderman()
        .expect("enderman entity must expose EndermanMeta")
        .set_carried_block(Some(air));
    assert_eq!(
        enderman
            .get_entity_meta_mut()
            .as_enderman()
            .expect("enderman entity must expose EndermanMeta")
            .get_carried_block(),
        None
    );
}

#[test]
fn generic_entity_villager_scalar_metadata_matches_minestom() {
    let mut villager = GenericEntity::new(EntityType::VILLAGER);
    let mut zombie_villager = GenericEntity::new(EntityType::ZOMBIE_VILLAGER);
    let librarian = VillagerData::DEFAULT
        .with_type(VillagerType::PLAINS)
        .with_profession(VillagerProfession::LIBRARIAN)
        .with_level(VillagerLevel::Master);

    assert_eq!(
        villager
            .get_entity_meta_mut()
            .as_villager()
            .expect("villager entity must expose VillagerMeta")
            .get_head_shake_timer(),
        0
    );
    assert_eq!(
        villager
            .get_entity_meta_mut()
            .as_villager()
            .expect("villager entity must expose VillagerMeta")
            .get_villager_data(),
        VillagerData::DEFAULT
    );
    assert_eq!(
        villager.component(spinel_registry::data_components::vanilla_components::VILLAGER_VARIANT),
        Some(VillagerType::DESERT)
    );
    assert!(
        !zombie_villager
            .get_entity_meta_mut()
            .as_zombie_villager()
            .expect("zombie villager entity must expose ZombieVillagerMeta")
            .is_converting()
    );
    assert_eq!(
        zombie_villager
            .get_entity_meta_mut()
            .as_zombie_villager()
            .expect("zombie villager entity must expose ZombieVillagerMeta")
            .get_villager_data(),
        VillagerData::DEFAULT
    );

    villager
        .get_entity_meta_mut()
        .as_villager()
        .expect("villager entity must expose VillagerMeta")
        .set_head_shake_timer(12);
    villager
        .get_entity_meta_mut()
        .as_villager()
        .expect("villager entity must expose VillagerMeta")
        .set_villager_data(librarian.clone());
    villager.set_component(
        spinel_registry::data_components::vanilla_components::VILLAGER_VARIANT,
        VillagerType::TAIGA,
    );
    zombie_villager
        .get_entity_meta_mut()
        .as_zombie_villager()
        .expect("zombie villager entity must expose ZombieVillagerMeta")
        .set_converting(true);
    zombie_villager
        .get_entity_meta_mut()
        .as_zombie_villager()
        .expect("zombie villager entity must expose ZombieVillagerMeta")
        .set_villager_data(librarian.clone());
    zombie_villager.set_component(
        spinel_registry::data_components::vanilla_components::VILLAGER_VARIANT,
        VillagerType::SWAMP,
    );

    assert_eq!(
        villager
            .get_entity_meta_mut()
            .as_villager()
            .expect("villager entity must expose VillagerMeta")
            .get_head_shake_timer(),
        12
    );
    assert_eq!(
        villager
            .get_entity_meta_mut()
            .as_villager()
            .expect("villager entity must expose VillagerMeta")
            .get_villager_data(),
        librarian.with_type(VillagerType::TAIGA)
    );
    assert!(
        zombie_villager
            .get_entity_meta_mut()
            .as_zombie_villager()
            .expect("zombie villager entity must expose ZombieVillagerMeta")
            .is_converting()
    );
    assert_eq!(
        zombie_villager
            .get_entity_meta_mut()
            .as_zombie_villager()
            .expect("zombie villager entity must expose ZombieVillagerMeta")
            .get_villager_data(),
        librarian.with_type(VillagerType::SWAMP)
    );
}

#[test]
fn dolphin_and_goat_metadata_owners_match_minestom_meta_surface() {
    let mut dolphin = GenericEntity::new(EntityType::DOLPHIN);
    let mut goat = GenericEntity::new(EntityType::GOAT);
    let treasure_position = spinel_network::types::Position { x: 4, y: 5, z: 6 };

    {
        let mut dolphin_meta = dolphin
            .get_entity_meta_mut()
            .as_dolphin()
            .expect("dolphin entity must expose DolphinMeta");
        assert_eq!(
            dolphin_meta.get_treasure_position(),
            spinel_network::types::Position { x: 0, y: 0, z: 0 }
        );
        assert!(!dolphin_meta.has_fish());
        assert_eq!(dolphin_meta.get_moisture_level(), 2400);
        dolphin_meta.set_treasure_position(treasure_position);
        dolphin_meta.set_has_fish(true);
        dolphin_meta.set_moisture_level(1200);
        assert_eq!(dolphin_meta.get_treasure_position(), treasure_position);
        assert!(dolphin_meta.has_fish());
        assert_eq!(dolphin_meta.get_moisture_level(), 1200);
    }
    {
        let mut goat_meta = goat
            .get_entity_meta_mut()
            .as_goat()
            .expect("goat entity must expose GoatMeta");
        assert!(!goat_meta.is_screaming());
        assert!(goat_meta.has_left_horn());
        assert!(goat_meta.has_right_horn());
        goat_meta.set_screaming(true);
        goat_meta.set_has_left_horn(false);
        goat_meta.set_has_right_horn(false);
        assert!(goat_meta.is_screaming());
        assert!(!goat_meta.has_left_horn());
        assert!(!goat_meta.has_right_horn());
    }
}

#[test]
fn axolotl_metadata_owner_matches_minestom_meta_surface() {
    let mut axolotl = GenericEntity::new(EntityType::AXOLOTL);
    let mut axolotl_meta = axolotl
        .get_entity_meta_mut()
        .as_axolotl()
        .expect("axolotl entity must expose AxolotlMeta");

    assert_eq!(axolotl_meta.get_variant(), AxolotlVariant::Lucy);
    assert!(!axolotl_meta.is_playing_dead());
    assert!(!axolotl_meta.is_from_bucket());

    axolotl_meta.set_variant(AxolotlVariant::Blue);
    axolotl_meta.set_playing_dead(true);
    axolotl_meta.set_from_bucket(true);

    assert_eq!(axolotl_meta.get_variant(), AxolotlVariant::Blue);
    assert!(axolotl_meta.is_playing_dead());
    assert!(axolotl_meta.is_from_bucket());
}

#[test]
fn generic_entity_pig_and_sheep_metadata_api_matches_minestom_meta_surface() {
    let mut pig = GenericEntity::new(EntityType::PIG);
    let mut sheep = GenericEntity::new(EntityType::SHEEP);

    assert_eq!(
        pig.get_entity_meta_mut()
            .as_pig()
            .expect("pig entity must expose PigMeta")
            .get_time_to_boost(),
        0
    );
    {
        let sheep_meta = sheep
            .get_entity_meta_mut()
            .as_sheep()
            .expect("sheep entity must expose SheepMeta");
        assert_eq!(sheep_meta.get_color(), DyeColor::White);
        assert!(!sheep_meta.is_sheared());
    }

    {
        let mut sheep_meta = sheep
            .get_entity_meta_mut()
            .as_sheep()
            .expect("sheep entity must expose SheepMeta");
        sheep_meta.set_color(DyeColor::Blue);
        sheep_meta.set_sheared(true);
    }

    {
        let sheep_meta = sheep
            .get_entity_meta_mut()
            .as_sheep()
            .expect("sheep entity must expose SheepMeta");
        assert_eq!(sheep_meta.get_color(), DyeColor::Blue);
        assert!(sheep_meta.is_sheared());
    }
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

    {
        let mut horse_meta = horse
            .get_entity_meta_mut()
            .as_horse()
            .expect("horse entity must expose HorseMeta");
        horse_meta.set_tamed(true);
        horse_meta.set_has_bred(true);
        horse_meta.set_eating(true);
        horse_meta.set_rearing(true);
        horse_meta.set_mouth_open(true);
    }
    {
        let mut camel_meta = camel
            .get_entity_meta_mut()
            .as_camel()
            .expect("camel entity must expose CamelMeta");
        camel_meta.set_dashing(true);
        camel_meta.set_last_pose_change_tick(42);
    }
    donkey
        .get_entity_meta_mut()
        .as_donkey()
        .expect("donkey entity must expose DonkeyMeta")
        .set_has_chest(true);
    ocelot
        .get_entity_meta_mut()
        .as_ocelot()
        .expect("ocelot entity must expose OcelotMeta")
        .set_trusting(true);
    {
        let mut turtle_meta = turtle
            .get_entity_meta_mut()
            .as_turtle()
            .expect("turtle entity must expose TurtleMeta");
        turtle_meta.set_has_egg(true);
        turtle_meta.set_laying_egg(true);
    }
    polar_bear
        .get_entity_meta_mut()
        .as_polar_bear()
        .expect("polar bear entity must expose PolarBearMeta")
        .set_standing_up(true);
    hoglin
        .get_entity_meta_mut()
        .as_hoglin()
        .expect("hoglin entity must expose HoglinMeta")
        .set_immune_to_zombification(true);
    {
        let mut strider_meta = strider
            .get_entity_meta_mut()
            .as_strider()
            .expect("strider entity must expose StriderMeta");
        strider_meta.set_time_to_boost(80);
        strider_meta.set_shaking(true);
    }
    {
        let mut wolf_meta = wolf
            .get_entity_meta_mut()
            .as_wolf()
            .expect("wolf entity must expose WolfMeta");
        wolf_meta.set_sitting(true);
        wolf_meta.set_tamed(true);
        wolf_meta.set_owner(Some(owner));
    }
    nautilus
        .get_entity_meta_mut()
        .as_nautilus()
        .expect("nautilus entity must expose NautilusMeta")
        .set_dashing(true);
    {
        let mut happy_ghast_meta = happy_ghast
            .get_entity_meta_mut()
            .as_happy_ghast()
            .expect("happy ghast entity must expose HappyGhastMeta");
        happy_ghast_meta.set_leash_holder(true);
        happy_ghast_meta.set_stays_still(true);
    }

    {
        let horse_meta = horse
            .get_entity_meta_mut()
            .as_horse()
            .expect("horse entity must expose HorseMeta");
        assert!(horse_meta.is_tamed());
        assert!(horse_meta.has_bred());
        assert!(horse_meta.is_eating());
        assert!(horse_meta.is_rearing());
        assert!(horse_meta.is_mouth_open());
    }
    {
        let camel_meta = camel
            .get_entity_meta_mut()
            .as_camel()
            .expect("camel entity must expose CamelMeta");
        assert!(camel_meta.is_dashing());
        assert_eq!(camel_meta.get_last_pose_change_tick(), 42);
    }
    assert!(
        donkey
            .get_entity_meta_mut()
            .as_donkey()
            .expect("donkey entity must expose DonkeyMeta")
            .has_chest()
    );
    assert!(
        ocelot
            .get_entity_meta_mut()
            .as_ocelot()
            .is_some_and(|ocelot_meta| ocelot_meta.is_trusting())
    );
    assert!(
        turtle
            .get_entity_meta_mut()
            .as_turtle()
            .is_some_and(|turtle_meta| turtle_meta.has_egg() && turtle_meta.is_laying_egg())
    );
    assert!(
        polar_bear
            .get_entity_meta_mut()
            .as_polar_bear()
            .is_some_and(|polar_bear_meta| polar_bear_meta.is_standing_up())
    );
    assert!(
        hoglin
            .get_entity_meta_mut()
            .as_hoglin()
            .is_some_and(|hoglin_meta| hoglin_meta.is_immune_to_zombification())
    );
    assert!(
        strider
            .get_entity_meta_mut()
            .as_strider()
            .is_some_and(|strider_meta| {
                strider_meta.get_time_to_boost() == 80 && strider_meta.is_shaking()
            })
    );
    {
        let wolf_meta = wolf
            .get_entity_meta_mut()
            .as_wolf()
            .expect("wolf entity must expose WolfMeta");
        assert!(wolf_meta.is_sitting());
        assert!(wolf_meta.is_tamed());
        assert_eq!(wolf_meta.get_owner(), Some(owner));
    }
    assert!(
        nautilus
            .get_entity_meta_mut()
            .as_nautilus()
            .is_some_and(|nautilus_meta| nautilus_meta.is_dashing())
    );
    assert!(
        happy_ghast
            .get_entity_meta_mut()
            .as_happy_ghast()
            .is_some_and(|happy_ghast_meta| {
                happy_ghast_meta.is_leash_holder() && happy_ghast_meta.should_stay_still()
            })
    );
}

#[test]
fn generic_entity_cat_and_wolf_scalar_components_match_minestom() {
    let mut cat = GenericEntity::new(EntityType::CAT);
    let mut wolf = GenericEntity::new(EntityType::WOLF);

    {
        let cat_meta = cat
            .get_entity_meta_mut()
            .as_cat()
            .expect("cat entity must expose CatMeta");
        assert!(!cat_meta.is_lying());
        assert!(!cat_meta.is_relaxed());
        assert_eq!(cat_meta.get_collar_color(), DyeColor::Red);
    }
    {
        let wolf_meta = wolf
            .get_entity_meta_mut()
            .as_wolf()
            .expect("wolf entity must expose WolfMeta");
        assert!(!wolf_meta.is_begging());
        assert_eq!(wolf_meta.get_collar_color(), DyeColor::Red);
        assert_eq!(wolf_meta.get_anger_time(), -1);
    }

    {
        let mut cat_meta = cat
            .get_entity_meta_mut()
            .as_cat()
            .expect("cat entity must expose CatMeta");
        cat_meta.set_lying(true);
        cat_meta.set_relaxed(true);
    }
    cat.set_component(
        spinel_registry::data_components::vanilla_components::CAT_COLLAR,
        DyeColor::Blue,
    );
    wolf.get_entity_meta_mut()
        .as_wolf()
        .expect("wolf entity must expose WolfMeta")
        .set_begging(true);
    wolf.set_component(
        spinel_registry::data_components::vanilla_components::WOLF_COLLAR,
        DyeColor::Green,
    );
    wolf.get_entity_meta_mut()
        .as_wolf()
        .expect("wolf entity must expose WolfMeta")
        .set_anger_time(120);

    {
        let cat_meta = cat
            .get_entity_meta_mut()
            .as_cat()
            .expect("cat entity must expose CatMeta");
        assert!(cat_meta.is_lying());
        assert!(cat_meta.is_relaxed());
        assert_eq!(cat_meta.get_collar_color(), DyeColor::Blue);
    }
    assert_eq!(
        cat.component(spinel_registry::data_components::vanilla_components::CAT_COLLAR),
        Some(DyeColor::Blue)
    );
    {
        let wolf_meta = wolf
            .get_entity_meta_mut()
            .as_wolf()
            .expect("wolf entity must expose WolfMeta");
        assert!(wolf_meta.is_begging());
        assert_eq!(wolf_meta.get_collar_color(), DyeColor::Green);
        assert_eq!(wolf_meta.get_anger_time(), 120);
    }
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

    horse
        .get_entity_meta_mut()
        .as_horse()
        .expect("horse entity must expose HorseMeta")
        .set_variant(horse_variant);
    {
        let mut llama_meta = llama
            .get_entity_meta_mut()
            .as_llama()
            .expect("llama entity must expose LlamaMeta");
        llama_meta.set_strength(5);
        llama_meta.set_carpet_color(11);
        llama_meta.set_variant(LlamaVariant::Gray);
    }
    {
        let mut fox_meta = fox
            .get_entity_meta_mut()
            .as_fox()
            .expect("fox entity must expose FoxMeta");
        fox_meta.set_variant(FoxVariant::Snow);
        fox_meta.set_sitting(true);
        fox_meta.set_fox_sneaking(true);
        fox_meta.set_interested(true);
        fox_meta.set_pouncing(true);
        fox_meta.set_sleeping(true);
        fox_meta.set_faceplanted(true);
        fox_meta.set_defending(true);
        fox_meta.set_first_uuid(Some(trusted_fox));
        fox_meta.set_second_uuid(Some(second_trusted_fox));
    }
    {
        let mut panda_meta = panda
            .get_entity_meta_mut()
            .as_panda()
            .expect("panda entity must expose PandaMeta");
        panda_meta.set_breed_timer(1);
        panda_meta.set_sneeze_timer(2);
        panda_meta.set_eat_timer(3);
        panda_meta.set_main_gene(PandaGene::Brown);
        panda_meta.set_hidden_gene(PandaGene::Weak);
        panda_meta.set_sneezing(true);
        panda_meta.set_rolling(true);
        panda_meta.set_sitting(true);
        panda_meta.set_on_back(true);
    }
    rabbit
        .get_entity_meta_mut()
        .as_rabbit()
        .expect("rabbit entity must expose RabbitMeta")
        .set_variant(RabbitVariant::KillerBunny);
    mooshroom
        .get_entity_meta_mut()
        .as_mooshroom()
        .expect("mooshroom entity must expose MooshroomMeta")
        .set_variant(MooshroomVariant::Brown);
    parrot
        .get_entity_meta_mut()
        .as_parrot()
        .expect("parrot entity must expose ParrotMeta")
        .set_color(ParrotColor::YellowBlue);

    assert_eq!(
        horse
            .get_entity_meta_mut()
            .as_horse()
            .expect("horse entity must expose HorseMeta")
            .get_variant(),
        horse_variant
    );
    assert_eq!(horse_variant.get_protocol_id(), (3 << 8) + 6);
    {
        let llama_meta = llama
            .get_entity_meta_mut()
            .as_llama()
            .expect("llama entity must expose LlamaMeta");
        assert_eq!(llama_meta.get_strength(), 5);
        assert_eq!(llama_meta.get_carpet_color(), 11);
        assert_eq!(llama_meta.get_variant(), LlamaVariant::Gray);
    }
    {
        let fox_meta = fox
            .get_entity_meta_mut()
            .as_fox()
            .expect("fox entity must expose FoxMeta");
        assert_eq!(fox_meta.get_variant(), FoxVariant::Snow);
        assert!(fox_meta.is_sitting());
        assert!(fox_meta.is_fox_sneaking());
        assert!(fox_meta.is_interested());
        assert!(fox_meta.is_pouncing());
        assert!(fox_meta.is_sleeping());
        assert!(fox_meta.is_faceplanted());
        assert!(fox_meta.is_defending());
        assert_eq!(fox_meta.get_first_uuid(), Some(trusted_fox));
        assert_eq!(fox_meta.get_second_uuid(), Some(second_trusted_fox));
    }
    {
        let panda_meta = panda
            .get_entity_meta_mut()
            .as_panda()
            .expect("panda entity must expose PandaMeta");
        assert_eq!(panda_meta.get_breed_timer(), 1);
        assert_eq!(panda_meta.get_sneeze_timer(), 2);
        assert_eq!(panda_meta.get_eat_timer(), 3);
        assert_eq!(panda_meta.get_main_gene(), PandaGene::Brown);
        assert_eq!(panda_meta.get_hidden_gene(), PandaGene::Weak);
        assert!(panda_meta.is_sneezing());
        assert!(panda_meta.is_rolling());
        assert!(panda_meta.is_sitting());
        assert!(panda_meta.is_on_back());
    }
    assert_eq!(
        rabbit
            .get_entity_meta_mut()
            .as_rabbit()
            .expect("rabbit entity must expose RabbitMeta")
            .get_variant(),
        RabbitVariant::KillerBunny
    );
    assert_eq!(RabbitVariant::KillerBunny.protocol_id(), 99);
    assert_eq!(
        mooshroom
            .get_entity_meta_mut()
            .as_mooshroom()
            .expect("mooshroom entity must expose MooshroomMeta")
            .get_variant(),
        MooshroomVariant::Brown
    );
    assert_eq!(
        parrot
            .get_entity_meta_mut()
            .as_parrot()
            .expect("parrot entity must expose ParrotMeta")
            .get_color(),
        ParrotColor::YellowBlue
    );
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

    horse
        .get_entity_meta_mut()
        .as_horse()
        .expect("horse entity must expose HorseMeta")
        .set_variant(HorseVariant::new(
            HorseMarking::BlackDots,
            HorseColor::White,
        ));
    tropical_fish
        .get_entity_meta_mut()
        .as_tropical_fish()
        .expect("tropical fish entity must expose TropicalFishMeta")
        .set_variant(TropicalFishVariant::new(
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
    assert_eq!(
        horse
            .get_entity_meta_mut()
            .as_horse()
            .expect("horse entity must expose HorseMeta")
            .get_variant()
            .get_marking(),
        HorseMarking::BlackDots
    );
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
fn fish_metadata_owners_match_minestom_meta_surface() {
    let mut pufferfish = GenericEntity::new(EntityType::PUFFERFISH);
    let mut salmon = GenericEntity::new(EntityType::SALMON);
    let mut tropical_fish = GenericEntity::new(EntityType::TROPICAL_FISH);
    let tropical_variant =
        TropicalFishVariant::new(TropicalFishPattern::Betty, DyeColor::Blue, DyeColor::Red);

    {
        let mut pufferfish_meta = pufferfish
            .get_entity_meta_mut()
            .as_pufferfish()
            .expect("pufferfish entity must expose PufferfishMeta");
        assert!(!pufferfish_meta.is_from_bucket());
        assert_eq!(pufferfish_meta.get_state(), PufferfishState::Unpuffed);
        assert_eq!(pufferfish_meta.get_entity().get_bounding_box().get_width(), 0.35);
        pufferfish_meta.set_from_bucket(true);
        pufferfish_meta.set_state(PufferfishState::FullyPuffed);
        assert!(pufferfish_meta.is_from_bucket());
        assert_eq!(pufferfish_meta.get_state(), PufferfishState::FullyPuffed);
        assert_eq!(pufferfish_meta.get_entity().get_bounding_box().get_width(), 0.7);
    }
    {
        let mut salmon_meta = salmon
            .get_entity_meta_mut()
            .as_salmon()
            .expect("salmon entity must expose SalmonMeta");
        assert_eq!(salmon_meta.get_size(), SalmonSize::Small);
        salmon_meta.set_size(SalmonSize::Large);
        assert_eq!(salmon_meta.get_size(), SalmonSize::Large);
        assert_eq!(salmon_meta.get_size().id(), "large");
    }
    {
        let mut tropical_fish_meta = tropical_fish
            .get_entity_meta_mut()
            .as_tropical_fish()
            .expect("tropical fish entity must expose TropicalFishMeta");
        assert_eq!(tropical_fish_meta.get_variant(), TropicalFishVariant::DEFAULT);
        tropical_fish_meta.set_variant(tropical_variant);
        assert_eq!(tropical_fish_meta.get_variant(), tropical_variant);
    }
}

#[test]
fn generic_entity_snapshot_updater_receives_mutable_snapshot_copy() {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    let position = EntityPosition::new(3.0, 64.0, 9.0, 45.0, 10.0);
    entity.teleport(position);

    let snapshot = entity.update_snapshot(|snapshot| {
        assert_eq!(snapshot.get_entity_id(), entity.get_entity_id());
        assert_eq!(snapshot.get_entity_type(), EntityType::ZOMBIE);
        assert_eq!(snapshot.get_position(), position);
    });

    assert_eq!(snapshot.get_uuid(), entity.get_uuid());
    assert_eq!(snapshot.get_position(), position);
}

#[test]
fn generic_entity_event_node_pose_kill_viewer_packets_and_occlusion_match_minestom_surface() {
    static POSE_EVENT_ENTITY_ID: AtomicI32 = AtomicI32::new(0);
    let (mut client, _peer_stream) = test_client();
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity
        .get_attribute(1, 20.0)
        .add_modifier(EntityAttributeModifier::attack_speed(
            spinel_network::types::Identifier::minecraft("test_modifier"),
            1.0,
        ));

    entity.get_event_node().listen("EntityPoseEvent", |entity_id| {
        POSE_EVENT_ENTITY_ID.store(entity_id.get_value(), Ordering::SeqCst);
    });
    entity.set_pose(EntityPose::Sleeping);

    assert_eq!(
        POSE_EVENT_ENTITY_ID.load(Ordering::SeqCst),
        entity.get_entity_id().get_value()
    );
    assert_eq!(entity.get_event_node().listener_count("EntityPoseEvent"), 1);
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
    assert_eq!(entity.get_pose(), EntityPose::Dying);
}

fn test_client() -> (Client, TcpStream) {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let client_stream = TcpStream::connect(addr).unwrap();
    let (peer_stream, _) = listener.accept().unwrap();
    (Client::new(client_stream, addr), peer_stream)
}
