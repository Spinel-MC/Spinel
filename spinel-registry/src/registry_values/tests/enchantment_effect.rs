use super::super::enchantment_effect::{
    AttributeEffect, ConditionalEffect, DamageImmunityEffect, EnchantmentEffect,
    EnchantmentEffectBranch, EnchantmentTarget, EntityEffect, EntityEffectAllOf,
    EntityEffectApplyExhaustion, EntityEffectApplyImpulse, EntityEffectApplyMobEffect,
    EntityEffectChangeItemDamage, EntityEffectDamageEntity, EntityEffectExplode,
    EntityEffectIgnite, EntityEffectPlaySound, EntityEffectReplaceBlock, EntityEffectReplaceDisk,
    EntityEffectRunFunction, EntityEffectSetBlockProperties, EntityEffectSpawnParticles,
    EntityEffectSummonEntity, TaggedEntityEffect, TaggedLocationEffect, TaggedValueEffect,
    TargetedConditionalEffect,
};
use crate::{AttributeOperation, Identifier, RegistryTagReference};
use spinel_nbt::{Nbt, NbtCompound, parse_snbt_compound};
use std::collections::BTreeMap;

#[test]
fn enchantment_effect_top_level_branches_are_distinct() {
    let attribute = AttributeEffect::new(
        Identifier::minecraft("attack_damage"),
        Identifier::minecraft("generic.attack_damage"),
        Nbt::Int(1),
        AttributeOperation::AddValue,
    );
    let entity = TaggedEntityEffect::new(Identifier::minecraft("ignite"), NbtCompound::new());
    let location =
        TaggedLocationEffect::new(Identifier::minecraft("attribute"), NbtCompound::new());
    let value = TaggedValueEffect::new(Identifier::minecraft("add"), NbtCompound::new());
    let conditional = ConditionalEffect::new(
        EnchantmentEffect::Value(value.clone()),
        Some(requirements_payload()),
    );
    let targeted = TargetedConditionalEffect::new(
        EnchantmentTarget::Attacker,
        Some(EnchantmentTarget::Victim),
        EnchantmentEffect::Entity(entity.clone()),
        None,
    );

    assert_eq!(
        EnchantmentEffect::Attribute(attribute).get_branch(),
        EnchantmentEffectBranch::Attribute
    );
    assert_eq!(
        EnchantmentEffect::Conditional(Box::new(conditional)).get_branch(),
        EnchantmentEffectBranch::Conditional
    );
    assert_eq!(
        EnchantmentEffect::DamageImmunity(DamageImmunityEffect::new()).get_branch(),
        EnchantmentEffectBranch::DamageImmunity
    );
    assert_eq!(
        EnchantmentEffect::Entity(entity).get_branch(),
        EnchantmentEffectBranch::Entity
    );
    assert_eq!(
        EnchantmentEffect::Location(location).get_branch(),
        EnchantmentEffectBranch::Location
    );
    assert_eq!(
        EnchantmentEffect::TargetedConditional(Box::new(targeted)).get_branch(),
        EnchantmentEffectBranch::TargetedConditional
    );
    assert_eq!(
        EnchantmentEffect::Value(value).get_branch(),
        EnchantmentEffectBranch::Value
    );
}

#[test]
fn attribute_effect_preserves_minestom_codec_field_names() {
    let amount = Nbt::Compound(level_value_payload(2.0));
    let effect = AttributeEffect::new(
        Identifier::minecraft("bonus"),
        Identifier::minecraft("generic.movement_speed"),
        amount.clone(),
        AttributeOperation::AddMultipliedTotal,
    );
    let nbt = effect.to_nbt_compound();

    assert_eq!(effect.get_id(), &Identifier::minecraft("bonus"));
    assert_eq!(
        effect.get_attribute(),
        &Identifier::minecraft("generic.movement_speed")
    );
    assert_eq!(effect.get_amount(), &amount);
    assert_eq!(
        effect.get_operation(),
        AttributeOperation::AddMultipliedTotal
    );
    assert_eq!(
        nbt.get("id"),
        Some(&Nbt::String("minecraft:bonus".to_owned()))
    );
    assert_eq!(
        nbt.get("attribute"),
        Some(&Nbt::String("minecraft:generic.movement_speed".to_owned()))
    );
    assert_eq!(nbt.get("amount"), Some(&amount));
    assert_eq!(
        nbt.get("operation"),
        Some(&Nbt::String("add_multiplied_total".to_owned()))
    );
}

#[test]
fn conditional_effect_preserves_effect_and_optional_requirements_fields() {
    let effect = EnchantmentEffect::Value(TaggedValueEffect::new(
        Identifier::minecraft("set"),
        level_value_payload(3.0),
    ));
    let conditional = ConditionalEffect::new(effect.clone(), Some(requirements_payload()));
    let nbt = conditional.to_nbt_compound();

    assert_eq!(conditional.get_effect(), &effect);
    assert!(conditional.get_requirements().is_some());
    assert!(matches!(nbt.get("effect"), Some(Nbt::Compound(_))));
    assert!(matches!(nbt.get("requirements"), Some(Nbt::Compound(_))));

    let unconditional = ConditionalEffect::new(effect, None);
    assert!(unconditional.get_requirements().is_none());
    assert!(!unconditional.to_nbt_compound().contains_key("requirements"));
}

#[test]
fn targeted_conditional_effect_preserves_target_and_nested_effect_fields() {
    let entity_effect = EnchantmentEffect::Entity(TaggedEntityEffect::new(
        Identifier::minecraft("ignite"),
        level_value_payload(4.0),
    ));
    let targeted = TargetedConditionalEffect::new(
        EnchantmentTarget::DamagingEntity,
        Some(EnchantmentTarget::Victim),
        entity_effect.clone(),
        Some(requirements_payload()),
    );
    let nbt = targeted.to_nbt_compound();

    assert_eq!(targeted.get_enchanted(), EnchantmentTarget::DamagingEntity);
    assert_eq!(targeted.get_affected(), Some(EnchantmentTarget::Victim));
    assert_eq!(targeted.get_effect(), &entity_effect);
    assert!(targeted.get_requirements().is_some());
    assert_eq!(
        nbt.get("enchanted"),
        Some(&Nbt::String("damaging_entity".to_owned()))
    );
    assert_eq!(nbt.get("affected"), Some(&Nbt::String("victim".to_owned())));
    assert!(matches!(nbt.get("effect"), Some(Nbt::Compound(_))));
    assert!(matches!(nbt.get("requirements"), Some(Nbt::Compound(_))));
}

#[test]
fn tagged_entity_location_and_value_effects_preserve_type_and_payload() {
    let mut payload = NbtCompound::new();
    payload.insert("value".to_owned(), Nbt::Float(2.0));
    let entity = TaggedEntityEffect::new(Identifier::minecraft("ignite"), payload.clone());
    let location = TaggedLocationEffect::new(Identifier::minecraft("attribute"), payload.clone());
    let value = TaggedValueEffect::new(Identifier::minecraft("add"), payload.clone());

    assert_eq!(entity.get_type_key(), &Identifier::minecraft("ignite"));
    assert_eq!(location.get_type_key(), &Identifier::minecraft("attribute"));
    assert_eq!(value.get_type_key(), &Identifier::minecraft("add"));
    assert_eq!(entity.get_payload(), &payload);
    assert_eq!(location.get_payload(), &payload);
    assert_eq!(value.get_payload(), &payload);
    assert_eq!(
        entity.to_nbt_compound().get("type"),
        Some(&Nbt::String("minecraft:ignite".to_owned()))
    );
    assert_eq!(
        location.to_nbt_compound().get("type"),
        Some(&Nbt::String("minecraft:attribute".to_owned()))
    );
    assert_eq!(
        value.to_nbt_compound().get("type"),
        Some(&Nbt::String("minecraft:add".to_owned()))
    );
}

#[test]
fn enchantment_target_nbt_names_match_minestom_target_enum() {
    assert_eq!(EnchantmentTarget::Attacker.nbt_name(), "attacker");
    assert_eq!(
        EnchantmentTarget::DamagingEntity.nbt_name(),
        "damaging_entity"
    );
    assert_eq!(EnchantmentTarget::Victim.nbt_name(), "victim");
    assert_eq!(
        EnchantmentTarget::from_nbt_name("attacker"),
        Some(EnchantmentTarget::Attacker)
    );
    assert_eq!(
        EnchantmentTarget::from_nbt_name("damaging_entity"),
        Some(EnchantmentTarget::DamagingEntity)
    );
    assert_eq!(
        EnchantmentTarget::from_nbt_name("victim"),
        Some(EnchantmentTarget::Victim)
    );
    assert_eq!(EnchantmentTarget::from_nbt_name("source"), None);
}

#[test]
fn entity_effect_union_preserves_all_of_and_single_value_leaf_shapes() {
    let ignite = EntityEffect::Ignite(EntityEffectIgnite::new(level_value_payload_nbt(5.0)));
    let exhaustion = EntityEffect::ApplyExhaustion(EntityEffectApplyExhaustion::new(
        level_value_payload_nbt(0.25),
    ));
    let change_item_damage = EntityEffect::ChangeItemDamage(EntityEffectChangeItemDamage::new(
        level_value_payload_nbt(3.0),
    ));
    let all_of = EntityEffect::AllOf(EntityEffectAllOf::new(vec![
        ignite.clone(),
        exhaustion.clone(),
        change_item_damage.clone(),
    ]));

    assert_eq!(ignite.get_type_key(), Identifier::minecraft("ignite"));
    assert_eq!(
        exhaustion.get_type_key(),
        Identifier::minecraft("apply_exhaustion")
    );
    assert_eq!(
        change_item_damage.get_type_key(),
        Identifier::minecraft("change_item_damage")
    );
    assert_eq!(all_of.get_type_key(), Identifier::minecraft("all_of"));
    assert_eq!(
        ignite.to_nbt_compound().get("duration"),
        Some(&level_value_payload_nbt(5.0))
    );
    assert_eq!(
        exhaustion.to_nbt_compound().get("amount"),
        Some(&level_value_payload_nbt(0.25))
    );
    assert_eq!(
        change_item_damage.to_nbt_compound().get("amount"),
        Some(&level_value_payload_nbt(3.0))
    );
    assert!(matches!(
        all_of.to_nbt_compound().get("effects"),
        Some(Nbt::List(effects)) if effects.len() == 3
    ));
}

#[test]
fn entity_effect_apply_mob_effect_preserves_minestom_codec_fields() {
    let to_apply = RegistryTagReference::direct(vec![Identifier::minecraft("speed")]);
    let effect = EntityEffectApplyMobEffect::new(
        to_apply.clone(),
        level_value_payload_nbt(20.0),
        level_value_payload_nbt(60.0),
        level_value_payload_nbt(1.0),
        level_value_payload_nbt(3.0),
    );
    let nbt = effect.to_nbt_compound();

    assert_eq!(effect.get_to_apply(), &to_apply);
    assert_eq!(effect.get_min_duration(), &level_value_payload_nbt(20.0));
    assert_eq!(effect.get_max_duration(), &level_value_payload_nbt(60.0));
    assert_eq!(effect.get_min_amplifier(), &level_value_payload_nbt(1.0));
    assert_eq!(effect.get_max_amplifier(), &level_value_payload_nbt(3.0));
    assert_eq!(nbt.get("to_apply"), Some(&to_apply.to_nbt()));
    assert_eq!(
        nbt.get("min_duration"),
        Some(&level_value_payload_nbt(20.0))
    );
    assert_eq!(
        nbt.get("max_duration"),
        Some(&level_value_payload_nbt(60.0))
    );
    assert_eq!(
        nbt.get("min_amplifier"),
        Some(&level_value_payload_nbt(1.0))
    );
    assert_eq!(
        nbt.get("max_amplifier"),
        Some(&level_value_payload_nbt(3.0))
    );
}

#[test]
fn entity_effect_damage_impulse_function_and_summon_preserve_codec_fields() {
    let damage = EntityEffectDamageEntity::new(
        Identifier::minecraft("generic"),
        level_value_payload_nbt(2.0),
        level_value_payload_nbt(7.0),
    );
    let impulse = EntityEffectApplyImpulse::new(
        vector_payload(0.0, 1.0, 0.0),
        vector_payload(1.0, 0.0, 1.0),
        level_value_payload_nbt(0.8),
    );
    let function = EntityEffectRunFunction::new("minecraft:smite_target");
    let summon =
        EntityEffectSummonEntity::new(Nbt::String("minecraft:lightning_bolt".to_owned()), true);

    assert_eq!(damage.get_damage_type(), &Identifier::minecraft("generic"));
    assert_eq!(damage.get_min_damage(), &level_value_payload_nbt(2.0));
    assert_eq!(damage.get_max_damage(), &level_value_payload_nbt(7.0));
    assert_eq!(impulse.get_direction(), &vector_payload(0.0, 1.0, 0.0));
    assert_eq!(
        impulse.get_coordinate_scale(),
        &vector_payload(1.0, 0.0, 1.0)
    );
    assert_eq!(impulse.get_magnitude(), &level_value_payload_nbt(0.8));
    assert_eq!(function.get_function(), "minecraft:smite_target");
    assert_eq!(
        summon.get_entity(),
        &Nbt::String("minecraft:lightning_bolt".to_owned())
    );
    assert!(summon.should_join_team());
    assert_eq!(
        damage.to_nbt_compound().get("damage_type"),
        Some(&Nbt::String("minecraft:generic".to_owned()))
    );
    assert_eq!(
        function.to_nbt_compound().get("function"),
        Some(&Nbt::String("minecraft:smite_target".to_owned()))
    );
    assert_eq!(
        summon.to_nbt_compound().get("join_team"),
        Some(&Nbt::Byte(1))
    );
}

#[test]
fn entity_effect_set_block_properties_preserves_optional_game_event() {
    let properties = string_list_payload(vec!["snowy", "waterlogged"]);
    let offset = vector_payload(0.0, -1.0, 0.0);
    let game_event = Nbt::String("minecraft:block_change".to_owned());
    let effect = EntityEffectSetBlockProperties::new(
        properties.clone(),
        offset.clone(),
        Some(game_event.clone()),
    );
    let nbt = effect.to_nbt_compound();

    assert_eq!(effect.get_properties(), &properties);
    assert_eq!(effect.get_offset(), &offset);
    assert_eq!(effect.get_trigger_game_event(), Some(&game_event));
    assert_eq!(nbt.get("properties"), Some(&properties));
    assert_eq!(nbt.get("offset"), Some(&offset));
    assert_eq!(nbt.get("trigger_game_event"), Some(&game_event));

    let without_event = EntityEffectSetBlockProperties::new(properties, offset, None);
    assert!(without_event.get_trigger_game_event().is_none());
    assert!(
        !without_event
            .to_nbt_compound()
            .contains_key("trigger_game_event")
    );
}
fn level_value_payload(value: f32) -> NbtCompound {
    let mut payload = NbtCompound::new();
    payload.insert("value".to_owned(), Nbt::Float(value));
    payload
}

#[test]
fn entity_effect_play_sound_preserves_sound_list_and_value_fields() {
    let sounds = vec![
        Nbt::String("minecraft:entity.player.levelup".to_owned()),
        Nbt::String("minecraft:block.note_block.bell".to_owned()),
    ];
    let volume = level_value_payload_nbt(0.8);
    let pitch = level_value_payload_nbt(1.2);
    let effect = EntityEffectPlaySound::new(sounds.clone(), volume.clone(), pitch.clone());
    let nbt = EntityEffect::PlaySound(effect.clone()).to_nbt_compound();

    assert_eq!(effect.get_sounds(), sounds.as_slice());
    assert_eq!(effect.get_volume(), &volume);
    assert_eq!(effect.get_pitch(), &pitch);
    assert_eq!(
        nbt.get("sound"),
        Some(&Nbt::List(sounds.clone().into_boxed_slice()))
    );
    assert_eq!(nbt.get("volume"), Some(&volume));
    assert_eq!(nbt.get("pitch"), Some(&pitch));
    assert_eq!(
        nbt.get("type"),
        Some(&Nbt::String("minecraft:play_sound".to_owned()))
    );
}

#[test]
fn entity_effect_replace_block_and_disk_preserve_raw_and_optional_fields() {
    let offset = vector_payload(0.0, -1.0, 0.0);
    let predicate = Nbt::String("minecraft:replaceable".to_owned());
    let block_state = Nbt::String("minecraft:ice".to_owned());
    let game_event = Nbt::String("minecraft:block_change".to_owned());
    let replace_block = EntityEffectReplaceBlock::new(
        offset.clone(),
        predicate.clone(),
        block_state.clone(),
        game_event.clone(),
    );
    let replace_block_nbt = EntityEffect::ReplaceBlock(replace_block.clone()).to_nbt_compound();
    let replace_disk = EntityEffectReplaceDisk::new(
        level_value_payload_nbt(3.0),
        level_value_payload_nbt(2.0),
        offset.clone(),
        None,
        block_state.clone(),
        None,
    );
    let replace_disk_nbt = EntityEffect::ReplaceDisk(replace_disk.clone()).to_nbt_compound();

    assert_eq!(replace_block.get_offset(), &offset);
    assert_eq!(replace_block.get_predicate(), &predicate);
    assert_eq!(replace_block.get_block_state(), &block_state);
    assert_eq!(replace_block.get_trigger_game_event(), &game_event);
    assert_eq!(replace_block_nbt.get("offset"), Some(&offset));
    assert_eq!(replace_block_nbt.get("predicate"), Some(&predicate));
    assert_eq!(replace_block_nbt.get("block_state"), Some(&block_state));
    assert_eq!(
        replace_block_nbt.get("trigger_game_event"),
        Some(&game_event)
    );
    assert_eq!(
        replace_block_nbt.get("type"),
        Some(&Nbt::String("minecraft:replace_block".to_owned()))
    );
    assert_eq!(replace_disk.get_offset(), &offset);
    assert!(replace_disk.get_predicate().is_none());
    assert_eq!(replace_disk.get_block_state(), &block_state);
    assert!(replace_disk.get_trigger_game_event().is_none());
    assert_eq!(
        replace_disk_nbt.get("radius"),
        Some(&level_value_payload_nbt(3.0))
    );
    assert_eq!(
        replace_disk_nbt.get("height"),
        Some(&level_value_payload_nbt(2.0))
    );
    assert_eq!(replace_disk_nbt.get("offset"), Some(&offset));
    assert_eq!(replace_disk_nbt.get("block_state"), Some(&block_state));
    assert!(!replace_disk_nbt.contains_key("predicate"));
    assert!(!replace_disk_nbt.contains_key("trigger_game_event"));
    assert_eq!(
        replace_disk_nbt.get("type"),
        Some(&Nbt::String("minecraft:replace_disk".to_owned()))
    );
}

#[test]
fn entity_effect_explode_preserves_required_optional_and_default_codec_fields() {
    let damage_type = Identifier::minecraft("explosion");
    let knockback_multiplier = level_value_payload_nbt(1.5);
    let immune_blocks = Nbt::String("minecraft:wither_immune".to_owned());
    let offset = vector_payload(0.0, 1.0, 0.0);
    let radius = level_value_payload_nbt(4.0);
    let block_interaction = Nbt::String("destroy".to_owned());
    let small_particle = Nbt::String("minecraft:explosion".to_owned());
    let large_particle = Nbt::String("minecraft:explosion_emitter".to_owned());
    let sound = Nbt::String("minecraft:entity.generic.explode".to_owned());
    let effect = EntityEffectExplode::new(
        true,
        Some(damage_type.clone()),
        Some(knockback_multiplier.clone()),
        Some(immune_blocks.clone()),
        offset.clone(),
        radius.clone(),
        true,
        block_interaction.clone(),
        small_particle.clone(),
        large_particle.clone(),
        sound.clone(),
    );
    let nbt = EntityEffect::Explode(effect.clone()).to_nbt_compound();

    assert!(effect.is_attributed_to_user());
    assert_eq!(effect.get_damage_type(), Some(&damage_type));
    assert_eq!(
        effect.get_knockback_multiplier(),
        Some(&knockback_multiplier)
    );
    assert_eq!(effect.get_immune_blocks(), Some(&immune_blocks));
    assert_eq!(effect.get_offset(), &offset);
    assert_eq!(effect.get_radius(), &radius);
    assert!(effect.does_create_fire());
    assert_eq!(effect.get_block_interaction(), &block_interaction);
    assert_eq!(effect.get_small_particle(), &small_particle);
    assert_eq!(effect.get_large_particle(), &large_particle);
    assert_eq!(effect.get_sound(), &sound);
    assert_eq!(nbt.get("attribute_to_user"), Some(&Nbt::Byte(1)));
    assert_eq!(
        nbt.get("damage_type"),
        Some(&Nbt::String(damage_type.to_string()))
    );
    assert_eq!(nbt.get("knockback_multiplier"), Some(&knockback_multiplier));
    assert_eq!(nbt.get("immune_blocks"), Some(&immune_blocks));
    assert_eq!(nbt.get("offset"), Some(&offset));
    assert_eq!(nbt.get("radius"), Some(&radius));
    assert_eq!(nbt.get("create_fire"), Some(&Nbt::Byte(1)));
    assert_eq!(nbt.get("block_interaction"), Some(&block_interaction));
    assert_eq!(nbt.get("small_particle"), Some(&small_particle));
    assert_eq!(nbt.get("large_particle"), Some(&large_particle));
    assert_eq!(nbt.get("sound"), Some(&sound));
    assert_eq!(
        nbt.get("type"),
        Some(&Nbt::String("minecraft:explode".to_owned()))
    );

    let default_effect = EntityEffectExplode::new(
        false,
        None,
        None,
        None,
        vector_payload(0.0, 0.0, 0.0),
        radius,
        false,
        block_interaction,
        small_particle,
        large_particle,
        sound,
    );
    let default_nbt = default_effect.to_nbt_compound();

    assert!(!default_effect.is_attributed_to_user());
    assert!(default_effect.get_damage_type().is_none());
    assert!(default_effect.get_knockback_multiplier().is_none());
    assert!(default_effect.get_immune_blocks().is_none());
    assert!(!default_effect.does_create_fire());
    assert!(!default_nbt.contains_key("attribute_to_user"));
    assert!(!default_nbt.contains_key("damage_type"));
    assert!(!default_nbt.contains_key("knockback_multiplier"));
    assert!(!default_nbt.contains_key("immune_blocks"));
    assert!(!default_nbt.contains_key("create_fire"));
}
#[test]
fn entity_effect_spawn_particles_preserves_every_raw_codec_field() {
    let particle = Nbt::String("minecraft:flame".to_owned());
    let horizontal_position = level_value_payload_nbt(0.25);
    let vertical_position = level_value_payload_nbt(0.5);
    let horizontal_velocity = level_value_payload_nbt(0.1);
    let vertical_velocity = level_value_payload_nbt(0.2);
    let speed = level_value_payload_nbt(0.3);
    let effect = EntityEffectSpawnParticles::new(
        particle.clone(),
        horizontal_position.clone(),
        vertical_position.clone(),
        horizontal_velocity.clone(),
        vertical_velocity.clone(),
        speed.clone(),
    );
    let nbt = EntityEffect::SpawnParticles(effect.clone()).to_nbt_compound();

    assert_eq!(effect.get_particle(), &particle);
    assert_eq!(effect.get_horizontal_position(), &horizontal_position);
    assert_eq!(effect.get_vertical_position(), &vertical_position);
    assert_eq!(effect.get_horizontal_velocity(), &horizontal_velocity);
    assert_eq!(effect.get_vertical_velocity(), &vertical_velocity);
    assert_eq!(effect.get_speed(), &speed);
    assert_eq!(nbt.get("particle"), Some(&particle));
    assert_eq!(nbt.get("horizontal_position"), Some(&horizontal_position));
    assert_eq!(nbt.get("vertical_position"), Some(&vertical_position));
    assert_eq!(nbt.get("horizontal_velocity"), Some(&horizontal_velocity));
    assert_eq!(nbt.get("vertical_velocity"), Some(&vertical_velocity));
    assert_eq!(nbt.get("speed"), Some(&speed));
    assert_eq!(
        nbt.get("type"),
        Some(&Nbt::String("minecraft:spawn_particles".to_owned()))
    );
}

fn requirements_payload() -> NbtCompound {
    let mut requirements = NbtCompound::new();
    requirements.insert(
        "predicate".to_owned(),
        Nbt::String("minecraft:test".to_owned()),
    );
    requirements
}

fn level_value_payload_nbt(value: f32) -> Nbt {
    Nbt::Compound(level_value_payload(value))
}

fn vector_payload(x: f64, y: f64, z: f64) -> Nbt {
    let mut vector = NbtCompound::new();
    vector.insert("x".to_owned(), Nbt::Double(x));
    vector.insert("y".to_owned(), Nbt::Double(y));
    vector.insert("z".to_owned(), Nbt::Double(z));
    Nbt::Compound(vector)
}

fn string_list_payload(values: Vec<&str>) -> Nbt {
    Nbt::List(
        values
            .into_iter()
            .map(|value| Nbt::String(value.to_owned()))
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    )
}

#[test]
fn entity_effect_from_nbt_compound_decodes_all_minestom_registered_leaf_types() {
    let effects = vec![
        EntityEffect::AllOf(EntityEffectAllOf::new(vec![EntityEffect::Ignite(
            EntityEffectIgnite::new(level_value_payload_nbt(5.0)),
        )])),
        EntityEffect::ApplyMobEffect(EntityEffectApplyMobEffect::new(
            RegistryTagReference::direct(vec![Identifier::minecraft("speed")]),
            level_value_payload_nbt(20.0),
            level_value_payload_nbt(60.0),
            level_value_payload_nbt(1.0),
            level_value_payload_nbt(3.0),
        )),
        EntityEffect::ChangeItemDamage(EntityEffectChangeItemDamage::new(level_value_payload_nbt(
            3.0,
        ))),
        EntityEffect::DamageEntity(EntityEffectDamageEntity::new(
            Identifier::minecraft("generic"),
            level_value_payload_nbt(2.0),
            level_value_payload_nbt(7.0),
        )),
        EntityEffect::Explode(EntityEffectExplode::new(
            false,
            None,
            None,
            None,
            vector_payload(0.0, 0.0, 0.0),
            level_value_payload_nbt(4.0),
            false,
            Nbt::String("destroy".to_owned()),
            Nbt::String("minecraft:explosion".to_owned()),
            Nbt::String("minecraft:explosion_emitter".to_owned()),
            Nbt::String("minecraft:entity.generic.explode".to_owned()),
        )),
        EntityEffect::Ignite(EntityEffectIgnite::new(level_value_payload_nbt(5.0))),
        EntityEffect::ApplyImpulse(EntityEffectApplyImpulse::new(
            vector_payload(0.0, 1.0, 0.0),
            vector_payload(1.0, 0.0, 1.0),
            level_value_payload_nbt(0.8),
        )),
        EntityEffect::ApplyExhaustion(EntityEffectApplyExhaustion::new(level_value_payload_nbt(
            0.25,
        ))),
        EntityEffect::PlaySound(EntityEffectPlaySound::new(
            vec![Nbt::String("minecraft:entity.player.levelup".to_owned())],
            level_value_payload_nbt(0.8),
            level_value_payload_nbt(1.2),
        )),
        EntityEffect::ReplaceBlock(EntityEffectReplaceBlock::new(
            vector_payload(0.0, -1.0, 0.0),
            Nbt::String("minecraft:replaceable".to_owned()),
            Nbt::String("minecraft:ice".to_owned()),
            Nbt::String("minecraft:block_change".to_owned()),
        )),
        EntityEffect::ReplaceDisk(EntityEffectReplaceDisk::new(
            level_value_payload_nbt(3.0),
            level_value_payload_nbt(2.0),
            vector_payload(0.0, -1.0, 0.0),
            None,
            Nbt::String("minecraft:ice".to_owned()),
            None,
        )),
        EntityEffect::RunFunction(EntityEffectRunFunction::new("minecraft:smite_target")),
        EntityEffect::SpawnParticles(EntityEffectSpawnParticles::new(
            Nbt::String("minecraft:flame".to_owned()),
            level_value_payload_nbt(0.25),
            level_value_payload_nbt(0.5),
            level_value_payload_nbt(0.1),
            level_value_payload_nbt(0.2),
            level_value_payload_nbt(0.3),
        )),
        EntityEffect::SetBlockProperties(EntityEffectSetBlockProperties::new(
            string_list_payload(vec!["snowy", "waterlogged"]),
            vector_payload(0.0, -1.0, 0.0),
            Some(Nbt::String("minecraft:block_change".to_owned())),
        )),
        EntityEffect::SummonEntity(EntityEffectSummonEntity::new(
            Nbt::String("minecraft:lightning_bolt".to_owned()),
            true,
        )),
    ];

    for effect in effects {
        let encoded = effect.to_nbt_compound();
        let decoded = EntityEffect::from_nbt_compound(&encoded).unwrap();

        assert_eq!(decoded.get_type_key(), effect.get_type_key());
        assert_eq!(decoded.to_nbt_compound(), encoded);
    }
}

#[test]
fn entity_effect_from_nbt_compound_rejects_malformed_payloads() {
    assert_eq!(EntityEffect::from_nbt_compound(&NbtCompound::new()), None);

    let mut unknown_type = NbtCompound::new();
    unknown_type.insert(
        "type".to_owned(),
        Nbt::String("minecraft:unknown_effect".to_owned()),
    );
    assert_eq!(EntityEffect::from_nbt_compound(&unknown_type), None);

    let mut invalid_damage_type = NbtCompound::new();
    invalid_damage_type.insert(
        "type".to_owned(),
        Nbt::String("minecraft:damage_entity".to_owned()),
    );
    invalid_damage_type.insert("damage_type".to_owned(), Nbt::String("bad key".to_owned()));
    invalid_damage_type.insert("min_damage".to_owned(), level_value_payload_nbt(1.0));
    invalid_damage_type.insert("max_damage".to_owned(), level_value_payload_nbt(2.0));
    assert_eq!(EntityEffect::from_nbt_compound(&invalid_damage_type), None);

    let mut missing_required_field = NbtCompound::new();
    missing_required_field.insert(
        "type".to_owned(),
        Nbt::String("minecraft:ignite".to_owned()),
    );
    assert_eq!(
        EntityEffect::from_nbt_compound(&missing_required_field),
        None
    );

    let mut malformed_all_of = NbtCompound::new();
    malformed_all_of.insert(
        "type".to_owned(),
        Nbt::String("minecraft:all_of".to_owned()),
    );
    malformed_all_of.insert(
        "effects".to_owned(),
        Nbt::List(vec![Nbt::String("minecraft:ignite".to_owned())].into_boxed_slice()),
    );
    assert_eq!(EntityEffect::from_nbt_compound(&malformed_all_of), None);
}

#[test]
fn entity_effect_replace_block_rejects_missing_required_fields() {
    let mut payload = NbtCompound::new();
    payload.insert(
        "type".to_owned(),
        Nbt::String("minecraft:replace_block".to_owned()),
    );
    payload.insert("offset".to_owned(), vector_payload(0.0, -1.0, 0.0));
    payload.insert(
        "block_state".to_owned(),
        Nbt::String("minecraft:frosted_ice".to_owned()),
    );

    assert_eq!(EntityEffect::from_nbt_compound(&payload), None);
}
#[test]
fn generated_enchantment_entity_effect_payloads_decode() {
    let generated_entries: BTreeMap<String, String> =
        serde_json::from_str(include_str!("../../../assets/enchantments.json"))
            .expect("generated enchantment JSON should decode");
    let decoded_effect_count = generated_entries
        .values()
        .map(|entry| parse_snbt_compound(entry).expect("generated enchantment SNBT should decode"))
        .map(|entry| generated_entity_effect_payload_count(&Nbt::Compound(entry)))
        .sum::<usize>();

    assert!(decoded_effect_count > 0);
}

fn generated_entity_effect_payload_count(enchantment: &Nbt) -> usize {
    let Nbt::Compound(enchantment) = enchantment else {
        return 0;
    };
    let Some(Nbt::Compound(effects)) = enchantment.get("effects") else {
        return 0;
    };

    effects
        .0
        .iter()
        .filter(|(component_key, _)| is_entity_effect_component(component_key))
        .map(|(_, trigger_effects)| entity_effects_in_trigger_component(trigger_effects))
        .sum()
}

fn entity_effects_in_trigger_component(trigger_effects: &Nbt) -> usize {
    let Nbt::List(trigger_effects) = trigger_effects else {
        return 0;
    };

    trigger_effects
        .iter()
        .filter_map(|trigger_effect| match trigger_effect {
            Nbt::Compound(trigger_effect) => trigger_effect.get("effect"),
            _ => None,
        })
        .map(|effect| {
            let Nbt::Compound(effect) = effect else {
                panic!("enchantment entity effect trigger payload should be a compound");
            };
            assert!(EntityEffect::from_nbt_compound(effect).is_some());
            1
        })
        .sum()
}

fn is_entity_effect_component(component_key: &String) -> bool {
    matches!(
        component_key.as_str(),
        "minecraft:post_attack"
            | "minecraft:post_piercing_attack"
            | "minecraft:hit_block"
            | "minecraft:tick"
            | "minecraft:projectile_spawned"
    )
}
